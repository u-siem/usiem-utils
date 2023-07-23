use std::{collections::HashMap, io::Cursor, path::{PathBuf, Path}};
use tokio::io::AsyncBufReadExt;
use usiem::{
    prelude::geo_ip::{GeoIpDataset, GeoIpInfo},
    utilities::types::LogString,
};

use crate::{
    common::{parse_ip4_network, parse_ip6_network},
    err::TempResult,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn download_maxmind_geo_litle2_asn(
    api_key: &str,
) -> TempResult<std::path::PathBuf> {
    let database_asn_url = format!("https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-ASN-CSV&license_key={}&suffix=zip",api_key);
    let body = reqwest::get(database_asn_url)
    .await?;
    let mut reader = Cursor::new(body.bytes().await?);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let file_path = std::env::temp_dir().join(format!("GeoLite2-ASN-{}.zip", nanos));
    let mut file = tokio::fs::File::create(&file_path).await?;
    tokio::io::copy(&mut reader, &mut file).await?;
    Ok(std::path::PathBuf::from(file_path))
}
pub async fn download_maxmind_geo_litle2_city(
    api_key: &str,
) -> TempResult<std::path::PathBuf> {
    let database_asn_url = format!("https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-City-CSV&license_key={}&suffix=zip",api_key);
    let body = reqwest::get(database_asn_url)
    .await?;
    let mut reader = Cursor::new(body.bytes().await?);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let file_path = std::env::temp_dir().join(format!("GeoLite2-city-{}.zip", nanos));
    let mut file = tokio::fs::File::create(&file_path).await?;
    tokio::io::copy(&mut reader, &mut file).await?;
    Ok(std::path::PathBuf::from(file_path))
}
pub async fn download_maxmind_geo_litle2_country(
    api_key: &str,
) -> TempResult<std::path::PathBuf> {

    let database_asn_url = format!("https://download.maxmind.com/app/geoip_download?edition_id=GeoLite2-Country-CSV&license_key={}&suffix=zip",api_key);
    let body = reqwest::get(database_asn_url)
    .await?;
    let mut reader = Cursor::new(body.bytes().await?);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let file_path = std::env::temp_dir().join(format!("GeoLite2-country-{}.zip", nanos));
    let mut file = tokio::fs::File::create(&file_path).await?;
    tokio::io::copy(&mut reader, &mut file).await?;
    Ok(std::path::PathBuf::from(file_path))
}

pub async fn extract_zip_db(path: &PathBuf) -> TempResult<PathBuf> {
    let path = path.clone();
    match tokio::task::spawn_blocking(move || {
        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);
        let mut zip = zip::ZipArchive::new(reader)?;
        let pth = match path.file_stem() {
            Some(v) => v.to_string_lossy(),
            None => return Err(crate::err::TempErr::Base("Invalid path")),
        };
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let extract_dir = std::env::temp_dir().join(format!("{}_{}_db", pth, nanos));
        let mut extract_dir = std::path::Path::new(&extract_dir).to_path_buf();
        std::fs::create_dir(&extract_dir)?;
        zip.extract(&extract_dir)?;

        loop {
            let mut directory_entries = std::fs::read_dir(&extract_dir)?;
            loop {
                let entry = match directory_entries.next() {
                    Some(v) => v?,
                    None => return Ok(extract_dir),
                };
                match entry.file_type() {
                    Ok(v) => {
                        if v.is_dir() {
                            extract_dir = entry.path();
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }).await {
        Ok(v) => return v,
        Err(_) => return Err(crate::err::TempErr::Base("Error extracting zip"))
    }
}

#[derive(Clone, Default)]
pub struct CountryInfo {
    pub continent_code: LogString,
    pub continent_name: LogString,
    pub country_iso_code: LogString,
    pub country_name: LogString,
}

#[derive(Clone, Default)]
pub struct CityInfo {
    pub city_name: LogString,
    pub country_name: LogString,
}
#[cfg(not(feature = "slow_geoip"))]
pub async fn process_maxmind_geo_lite2_csv<P: AsRef<Path>>(
    path: P,
    enable_city: bool,
    language : &str
) -> Result<GeoIpDataset, std::io::Error> {
    let dataset = {
        let geonames_country: HashMap<u32, CountryInfo> = process_maxmind_geo_lite2_country_csv(
            path.as_ref().join(&format!("GeoLite2-Country-Locations-{}.csv", language)),
        )
        .await?;
        let geonames_city: HashMap<u32, CityInfo> = process_maxmind_geo_lite2_city_csv(
            path.as_ref().join(&format!("GeoLite2-City-Locations-{}.csv", language)),
            enable_city,
        )
        .await?;
        let mut networks4: HashMap<String, GeoIpInfo> = process_maxmind_geo_lite2_city_block_csv(
            path.as_ref().join("GeoLite2-City-Blocks-IPv4.csv"),
            &geonames_city,
            &geonames_country,
            enable_city,
        )
        .await?;
        let mut networks6: HashMap<String, GeoIpInfo> = process_maxmind_geo_lite2_city_block_csv(
            path.as_ref().join("GeoLite2-City-Blocks-IPv6.csv"),
            &geonames_city,
            &geonames_country,
            enable_city,
        )
        .await?;
        process_maxmind_geo_lite2_asn_block_csv(
            path.as_ref().join("GeoLite2-ASN-Blocks-IPv4.csv"),
            &mut networks4,
        )
        .await?;
        process_maxmind_geo_lite2_asn_block_csv(
            path.as_ref().join("GeoLite2-ASN-Blocks-IPv6.csv"),
            &mut networks6,
        )
        .await?;
        let mut dataset = GeoIpDataset::new();
        for (network, data) in networks4 {
            parse_ip4_network(&network).and_then(|(ip, net)| {
                dataset.insert(usiem::prelude::SiemIp::V4(ip), net, data);
                Some((ip, net))
            });
        }
        for (network, data) in networks6 {
            parse_ip6_network(&network).and_then(|(ip, net)| {
                dataset.insert(usiem::prelude::SiemIp::V6(ip), net, data);
                Some((ip, net))
            });
        }
        dataset
    };

    Ok(dataset)
}

#[cfg(feature = "slow_geoip")]
pub async fn process_maxmind_geo_lite2_csv<P: AsRef<Path>>(
    path: P,
    enable_city: bool,
    language : &str,
    db_location : &str
) -> Result<GeoIpDataset, std::io::Error> {
    let dataset = {
        let geonames_country: HashMap<u32, CountryInfo> = process_maxmind_geo_lite2_country_csv(
            path.as_ref().join(&format!("GeoLite2-Country-Locations-{}.csv", language)),
        )
        .await?;
        let geonames_city: HashMap<u32, CityInfo> = process_maxmind_geo_lite2_city_csv(
            path.as_ref().join(&format!("GeoLite2-City-Locations-{}.csv", language)),
            enable_city,
        )
        .await?;
        let mut networks4: HashMap<String, GeoIpInfo> = process_maxmind_geo_lite2_city_block_csv(
            path.as_ref().join("GeoLite2-City-Blocks-IPv4.csv"),
            &geonames_city,
            &geonames_country,
            enable_city,
        )
        .await?;
        let mut networks6: HashMap<String, GeoIpInfo> = process_maxmind_geo_lite2_city_block_csv(
            path.as_ref().join("GeoLite2-City-Blocks-IPv6.csv"),
            &geonames_city,
            &geonames_country,
            enable_city,
        )
        .await?;
        process_maxmind_geo_lite2_asn_block_csv(
            path.as_ref().join("GeoLite2-ASN-Blocks-IPv4.csv"),
            &mut networks4,
        )
        .await?;
        process_maxmind_geo_lite2_asn_block_csv(
            path.as_ref().join("GeoLite2-ASN-Blocks-IPv6.csv"),
            &mut networks6,
        )
        .await?;
        let mut dataset = GeoIpDataset::new(db_location);
        for (network, data) in networks4 {
            parse_ip4_network(&network).and_then(|(ip, net)| {
                dataset.insert(usiem::prelude::SiemIp::V4(ip), net, data);
                Some((ip, net))
            });
        }
        for (network, data) in networks6 {
            parse_ip6_network(&network).and_then(|(ip, net)| {
                dataset.insert(usiem::prelude::SiemIp::V6(ip), net, data);
                Some((ip, net))
            });
        }
        dataset
    };

    Ok(dataset)
}

pub async fn process_maxmind_geo_lite2_city_block_csv<P: AsRef<Path>>(
    path: P,
    geo_city: &HashMap<u32, CityInfo>,
    geo_country: &HashMap<u32, CountryInfo>,
    enable_city: bool,
) -> Result<HashMap<String, GeoIpInfo>, std::io::Error> {
    let file = tokio::fs::File::open(path).await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();
    let header = lines.next_line()
        .await?
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, ""))?;
    let column_names: Vec<&str> = header.split(",").collect();
    let mut networks: HashMap<String, GeoIpInfo> = HashMap::new();
    loop {
        if let Some(line) = lines.next_line().await? {
            let column_values: Vec<&str> = split_column_values(&line);
            let mut ip_info = GeoIpInfo::default();
            let mut network = String::new();
            for (name, value) in column_names.iter().zip(column_values) {
                let name = *name;
                if name == "geoname_id" {
                    if !enable_city {
                        continue;
                    }
                    let geoname_id = value.parse::<u32>().unwrap_or_default();
                    geo_city.get(&geoname_id).and_then(|v| {
                        ip_info.city = v.city_name.clone();
                        if ip_info.country.is_empty() && !v.country_name.is_empty() {
                            ip_info.country = v.country_name.clone();
                        }
                        Some(v)
                    });
                } else if name == "registered_country_geoname_id" {
                    let geoname_id = value.parse::<u32>().unwrap_or_default();
                    geo_country.get(&geoname_id).and_then(|v| {
                        if ip_info.country.is_empty() && !v.country_name.is_empty() {
                            ip_info.country = v.country_name.clone();
                        }
                        if ip_info.country_iso.is_empty() && !v.country_iso_code.is_empty() {
                            ip_info.country_iso = v.country_iso_code.clone();
                        }
                        Some(v)
                    });
                } else if name == "network" {
                    network = value.to_string();
                } else if name == "latitude" {
                    ip_info.latitude = value.parse::<f32>().unwrap_or_default();
                } else if name == "network" {
                    ip_info.longitude = value.parse::<f32>().unwrap_or_default();
                }
            }
            networks.insert(network, ip_info);
        } else {
            return Ok(networks);
        }
    }
}

pub async fn process_maxmind_geo_lite2_asn_block_csv<P: AsRef<Path>>(
    path: P,
    networks: &mut HashMap<String, GeoIpInfo>,
) -> Result<(), std::io::Error> {
    let file = tokio::fs::File::open(path).await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();
    let header = lines
        .next_line()
        .await?
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, ""))?;
    let column_names: Vec<&str> = header.split(",").collect();
    loop {
        if let Some(line) = lines.next_line().await? {
            let column_values: Vec<&str> = split_column_values(&line);
            let mut autonomous_system_number = 0;
            let mut autonomous_system_organization = LogString::Borrowed("");
            let mut network = "";
            for (name, value) in column_names.iter().zip(column_values) {
                let name = *name;
                if name == "autonomous_system_number" {
                    autonomous_system_number = value.parse::<u32>().unwrap_or_default();
                } else if name == "autonomous_system_organization" {
                    autonomous_system_organization = static_principal_asns(value);
                } else if name == "network" {
                    network = value;
                }
            }
            networks.get_mut(network).and_then(|v| {
                v.asn = autonomous_system_number;
                v.isp = autonomous_system_organization;
                Some(v)
            });
        } else {
            return Ok(());
        }
    }
}

pub async fn process_maxmind_geo_lite2_city_csv<P: AsRef<Path>>(
    path: P,
    enable_city: bool,
) -> Result<HashMap<u32, CityInfo>, std::io::Error> {
    let file = tokio::fs::File::open(path).await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();
    let header = lines
        .next_line()
        .await?
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, ""))?;
    let column_names: Vec<&str> = header.split(",").collect();
    let mut geonames: HashMap<u32, CityInfo> = HashMap::new();
    loop {
        if let Some(line) = lines.next_line().await? {
            let column_values: Vec<&str> = split_column_values(&line);
            let mut city_info = CityInfo::default();
            let mut geoname_id = 0;
            for (name, value) in column_names.iter().zip(column_values) {
                let name = *name;
                if name == "geoname_id" {
                    geoname_id = match value.parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                } else if name == "city_name" {
                    if !enable_city {
                        continue;
                    }
                    city_info.city_name = LogString::Owned(value.replace('"', ""));
                } else if name == "country_name" {
                    city_info.country_name = get_static_country_name(value);
                }
            }
            geonames.insert(geoname_id, city_info);
        } else {
            return Ok(geonames);
        }
    }
}

pub async fn process_maxmind_geo_lite2_country_csv<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<u32, CountryInfo>, std::io::Error> {
    let file = tokio::fs::File::open(path).await?;
    let reader = tokio::io::BufReader::new(file);
    let mut lines = reader.lines();
    let header = lines
        .next_line()
        .await?
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidData, ""))?;
    let column_names: Vec<&str> = header.split(",").collect();
    let mut geonames: HashMap<u32, CountryInfo> = HashMap::new();
    loop {
        if let Some(line) = lines.next_line().await? {
            let column_values: Vec<&str> = split_column_values(&line);
            let mut country_info = CountryInfo::default();
            let mut geoname_id = 0;
            for (name, value) in column_names.iter().zip(column_values) {
                let name = *name;
                if name == "geoname_id" {
                    geoname_id = match value.parse::<u32>() {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                } else if name == "continent_code" {
                    country_info.continent_code = get_static_continent_code(value);
                } else if name == "continent_name" {
                    country_info.continent_name = get_static_continent_name(value);
                } else if name == "country_iso_code" {
                    country_info.country_iso_code = get_static_country_iso_name(value);
                    country_info.continent_name = get_static_continent_name(value);
                } else if name == "country_name" {
                }
            }
            geonames.insert(geoname_id, country_info);
        } else {
            return Ok(geonames);
        }
    }
}

pub fn split_column_values(values: &str) -> Vec<&str> {
    let mut returned = Vec::with_capacity(32);
    let mut is_escape = false;
    let mut in_string = false;
    let mut last_pos = 0;
    for (pos, char) in values.char_indices() {
        if in_string {
            if char == '\\' {
                is_escape = !is_escape;
            } else {
                if char == '"' && !is_escape {
                    in_string = false;
                    returned.push(&values[last_pos..pos]);
                    last_pos = pos + 2;
                }
                is_escape = false;
            }
        } else {
            if char == ',' {
                if pos >= last_pos {
                    returned.push(&values[last_pos..pos]);
                }
                last_pos = pos + 1;
            } else if char == '"' {
                in_string = true;
                last_pos += 1;
            }
        }
        if last_pos >= values.len() {
            break;
        }
    }
    if last_pos < values.len() {
        returned.push(&values[last_pos..]);
    }
    returned
}

pub fn get_static_continent_code(continent_code: &str) -> LogString {
    match continent_code {
        "AF" => LogString::Borrowed("AF"),
        "AS" => LogString::Borrowed("AS"),
        "EU" => LogString::Borrowed("EU"),
        "OC" => LogString::Borrowed("OC"),
        "AN" => LogString::Borrowed("AN"),
        "SA" => LogString::Borrowed("SA"),
        "NA" => LogString::Borrowed("NA"),
        _ => LogString::Owned(continent_code.to_string()),
    }
}

pub fn get_static_continent_name(continent_code: &str) -> LogString {
    match continent_code {
        "Africa" => LogString::Borrowed("Africa"),
        "Asia" => LogString::Borrowed("Asia"),
        "Europe" => LogString::Borrowed("Europe"),
        "Oceania" => LogString::Borrowed("Oceania"),
        "Antartica" => LogString::Borrowed("Antartica"),
        "South America" => LogString::Borrowed("South America"),
        "North America" => LogString::Borrowed("North America"),
        "\"South America\"" => LogString::Borrowed("South America"),
        "\"North America\"" => LogString::Borrowed("North America"),
        _ => LogString::Owned(continent_code.to_string()),
    }
}

pub fn get_static_country_iso_name(country_name: &str) -> LogString {
    match country_name {
        "RW" => LogString::Borrowed("RW"),
        "SO" => LogString::Borrowed("SO"),
        "YE" => LogString::Borrowed("YE"),
        "IQ" => LogString::Borrowed("IQ"),
        "SA" => LogString::Borrowed("SA"),
        "IR" => LogString::Borrowed("IR"),
        "CY" => LogString::Borrowed("CY"),
        "TZ" => LogString::Borrowed("TZ"),
        "SY" => LogString::Borrowed("SY"),
        "AM" => LogString::Borrowed("AM"),
        "KE" => LogString::Borrowed("KE"),
        "CD" => LogString::Borrowed("CD"),
        "DJ" => LogString::Borrowed("DJ"),
        "UG" => LogString::Borrowed("UG"),
        "CF" => LogString::Borrowed("CF"),
        "SC" => LogString::Borrowed("SC"),
        "JO" => LogString::Borrowed("JO"),
        "LB" => LogString::Borrowed("LB"),
        "KW" => LogString::Borrowed("KW"),
        "OM" => LogString::Borrowed("OM"),
        "QA" => LogString::Borrowed("QA"),
        "BH" => LogString::Borrowed("BH"),
        "AE" => LogString::Borrowed("AE"),
        "IL" => LogString::Borrowed("IL"),
        "TR" => LogString::Borrowed("TR"),
        "ET" => LogString::Borrowed("ET"),
        "ER" => LogString::Borrowed("ER"),
        "EG" => LogString::Borrowed("EG"),
        "SD" => LogString::Borrowed("SD"),
        "GR" => LogString::Borrowed("GR"),
        "BI" => LogString::Borrowed("BI"),
        "EE" => LogString::Borrowed("EE"),
        "LV" => LogString::Borrowed("LV"),
        "AZ" => LogString::Borrowed("AZ"),
        "LT" => LogString::Borrowed("LT"),
        "SJ" => LogString::Borrowed("SJ"),
        "GE" => LogString::Borrowed("GE"),
        "MD" => LogString::Borrowed("MD"),
        "BY" => LogString::Borrowed("BY"),
        "FI" => LogString::Borrowed("FI"),
        "AX" => LogString::Borrowed("AX"),
        "UA" => LogString::Borrowed("UA"),
        "MK" => LogString::Borrowed("MK"),
        "HU" => LogString::Borrowed("HU"),
        "BG" => LogString::Borrowed("BG"),
        "AL" => LogString::Borrowed("AL"),
        "PL" => LogString::Borrowed("PL"),
        "RO" => LogString::Borrowed("RO"),
        "XK" => LogString::Borrowed("XK"),
        "ZW" => LogString::Borrowed("ZW"),
        "ZM" => LogString::Borrowed("ZM"),
        "KM" => LogString::Borrowed("KM"),
        "MW" => LogString::Borrowed("MW"),
        "LS" => LogString::Borrowed("LS"),
        "BW" => LogString::Borrowed("BW"),
        "MU" => LogString::Borrowed("MU"),
        "SZ" => LogString::Borrowed("SZ"),
        "RE" => LogString::Borrowed("RE"),
        "ZA" => LogString::Borrowed("ZA"),
        "YT" => LogString::Borrowed("YT"),
        "MZ" => LogString::Borrowed("MZ"),
        "MG" => LogString::Borrowed("MG"),
        "AF" => LogString::Borrowed("AF"),
        "PK" => LogString::Borrowed("PK"),
        "BD" => LogString::Borrowed("BD"),
        "TM" => LogString::Borrowed("TM"),
        "TJ" => LogString::Borrowed("TJ"),
        "LK" => LogString::Borrowed("LK"),
        "BT" => LogString::Borrowed("BT"),
        "IN" => LogString::Borrowed("IN"),
        "MV" => LogString::Borrowed("MV"),
        "IO" => LogString::Borrowed("IO"),
        "NP" => LogString::Borrowed("NP"),
        "MM" => LogString::Borrowed("MM"),
        "UZ" => LogString::Borrowed("UZ"),
        "KZ" => LogString::Borrowed("KZ"),
        "KG" => LogString::Borrowed("KG"),
        "TF" => LogString::Borrowed("TF"),
        "HM" => LogString::Borrowed("HM"),
        "CC" => LogString::Borrowed("CC"),
        "PW" => LogString::Borrowed("PW"),
        "VN" => LogString::Borrowed("VN"),
        "TH" => LogString::Borrowed("TH"),
        "ID" => LogString::Borrowed("ID"),
        "LA" => LogString::Borrowed("LA"),
        "TW" => LogString::Borrowed("TW"),
        "PH" => LogString::Borrowed("PH"),
        "MY" => LogString::Borrowed("MY"),
        "CN" => LogString::Borrowed("CN"),
        "HK" => LogString::Borrowed("HK"),
        "BN" => LogString::Borrowed("BN"),
        "MO" => LogString::Borrowed("MO"),
        "KH" => LogString::Borrowed("KH"),
        "KR" => LogString::Borrowed("KR"),
        "JP" => LogString::Borrowed("JP"),
        "KP" => LogString::Borrowed("KP"),
        "SG" => LogString::Borrowed("SG"),
        "CK" => LogString::Borrowed("CK"),
        "TL" => LogString::Borrowed("TL"),
        "RU" => LogString::Borrowed("RU"),
        "MN" => LogString::Borrowed("MN"),
        "AU" => LogString::Borrowed("AU"),
        "CX" => LogString::Borrowed("CX"),
        "MH" => LogString::Borrowed("MH"),
        "FM" => LogString::Borrowed("FM"),
        "PG" => LogString::Borrowed("PG"),
        "SB" => LogString::Borrowed("SB"),
        "TV" => LogString::Borrowed("TV"),
        "NR" => LogString::Borrowed("NR"),
        "VU" => LogString::Borrowed("VU"),
        "NC" => LogString::Borrowed("NC"),
        "NF" => LogString::Borrowed("NF"),
        "NZ" => LogString::Borrowed("NZ"),
        "FJ" => LogString::Borrowed("FJ"),
        "LY" => LogString::Borrowed("LY"),
        "CM" => LogString::Borrowed("CM"),
        "SN" => LogString::Borrowed("SN"),
        "CG" => LogString::Borrowed("CG"),
        "PT" => LogString::Borrowed("PT"),
        "LR" => LogString::Borrowed("LR"),
        "CI" => LogString::Borrowed("CI"),
        "GH" => LogString::Borrowed("GH"),
        "GQ" => LogString::Borrowed("GQ"),
        "NG" => LogString::Borrowed("NG"),
        "BF" => LogString::Borrowed("BF"),
        "TG" => LogString::Borrowed("TG"),
        "GW" => LogString::Borrowed("GW"),
        "MR" => LogString::Borrowed("MR"),
        "BJ" => LogString::Borrowed("BJ"),
        "GA" => LogString::Borrowed("GA"),
        "SL" => LogString::Borrowed("SL"),
        "ST" => LogString::Borrowed("ST"),
        "GI" => LogString::Borrowed("GI"),
        "GM" => LogString::Borrowed("GM"),
        "GN" => LogString::Borrowed("GN"),
        "TD" => LogString::Borrowed("TD"),
        "NE" => LogString::Borrowed("NE"),
        "ML" => LogString::Borrowed("ML"),
        "EH" => LogString::Borrowed("EH"),
        "TN" => LogString::Borrowed("TN"),
        "ES" => LogString::Borrowed("ES"),
        "MA" => LogString::Borrowed("MA"),
        "MT" => LogString::Borrowed("MT"),
        "DZ" => LogString::Borrowed("DZ"),
        "FO" => LogString::Borrowed("FO"),
        "DK" => LogString::Borrowed("DK"),
        "IS" => LogString::Borrowed("IS"),
        "GB" => LogString::Borrowed("GB"),
        "CH" => LogString::Borrowed("CH"),
        "SE" => LogString::Borrowed("SE"),
        "NL" => LogString::Borrowed("NL"),
        "AT" => LogString::Borrowed("AT"),
        "BE" => LogString::Borrowed("BE"),
        "DE" => LogString::Borrowed("DE"),
        "LU" => LogString::Borrowed("LU"),
        "IE" => LogString::Borrowed("IE"),
        "MC" => LogString::Borrowed("MC"),
        "FR" => LogString::Borrowed("FR"),
        "AD" => LogString::Borrowed("AD"),
        "LI" => LogString::Borrowed("LI"),
        "JE" => LogString::Borrowed("JE"),
        "IM" => LogString::Borrowed("IM"),
        "GG" => LogString::Borrowed("GG"),
        "SK" => LogString::Borrowed("SK"),
        "CZ" => LogString::Borrowed("CZ"),
        "NO" => LogString::Borrowed("NO"),
        "VA" => LogString::Borrowed("VA"),
        "SM" => LogString::Borrowed("SM"),
        "IT" => LogString::Borrowed("IT"),
        "SI" => LogString::Borrowed("SI"),
        "ME" => LogString::Borrowed("ME"),
        "HR" => LogString::Borrowed("HR"),
        "BA" => LogString::Borrowed("BA"),
        "AO" => LogString::Borrowed("AO"),
        "NA" => LogString::Borrowed("NA"),
        "SH" => LogString::Borrowed("SH"),
        "BV" => LogString::Borrowed("BV"),
        "BB" => LogString::Borrowed("BB"),
        "CV" => LogString::Borrowed("CV"),
        "GY" => LogString::Borrowed("GY"),
        "GF" => LogString::Borrowed("GF"),
        "SR" => LogString::Borrowed("SR"),
        "PM" => LogString::Borrowed("PM"),
        "GL" => LogString::Borrowed("GL"),
        "PY" => LogString::Borrowed("PY"),
        "UY" => LogString::Borrowed("UY"),
        "BR" => LogString::Borrowed("BR"),
        "FK" => LogString::Borrowed("FK"),
        "GS" => LogString::Borrowed("GS"),
        "JM" => LogString::Borrowed("JM"),
        "DO" => LogString::Borrowed("DO"),
        "CU" => LogString::Borrowed("CU"),
        "MQ" => LogString::Borrowed("MQ"),
        "BS" => LogString::Borrowed("BS"),
        "BM" => LogString::Borrowed("BM"),
        "AI" => LogString::Borrowed("AI"),
        "TT" => LogString::Borrowed("TT"),
        "KN" => LogString::Borrowed("KN"),
        "DM" => LogString::Borrowed("DM"),
        "AG" => LogString::Borrowed("AG"),
        "LC" => LogString::Borrowed("LC"),
        "TC" => LogString::Borrowed("TC"),
        "AW" => LogString::Borrowed("AW"),
        "VG" => LogString::Borrowed("VG"),
        "VC" => LogString::Borrowed("VC"),
        "MS" => LogString::Borrowed("MS"),
        "MF" => LogString::Borrowed("MF"),
        "BL" => LogString::Borrowed("BL"),
        "GP" => LogString::Borrowed("GP"),
        "GD" => LogString::Borrowed("GD"),
        "KY" => LogString::Borrowed("KY"),
        "BZ" => LogString::Borrowed("BZ"),
        "SV" => LogString::Borrowed("SV"),
        "GT" => LogString::Borrowed("GT"),
        "HN" => LogString::Borrowed("HN"),
        "NI" => LogString::Borrowed("NI"),
        "CR" => LogString::Borrowed("CR"),
        "VE" => LogString::Borrowed("VE"),
        "EC" => LogString::Borrowed("EC"),
        "CO" => LogString::Borrowed("CO"),
        "PA" => LogString::Borrowed("PA"),
        "HT" => LogString::Borrowed("HT"),
        "AR" => LogString::Borrowed("AR"),
        "CL" => LogString::Borrowed("CL"),
        "BO" => LogString::Borrowed("BO"),
        "PE" => LogString::Borrowed("PE"),
        "MX" => LogString::Borrowed("MX"),
        "PF" => LogString::Borrowed("PF"),
        "PN" => LogString::Borrowed("PN"),
        "KI" => LogString::Borrowed("KI"),
        "TK" => LogString::Borrowed("TK"),
        "TO" => LogString::Borrowed("TO"),
        "WF" => LogString::Borrowed("WF"),
        "WS" => LogString::Borrowed("WS"),
        "NU" => LogString::Borrowed("NU"),
        "MP" => LogString::Borrowed("MP"),
        "GU" => LogString::Borrowed("GU"),
        "PR" => LogString::Borrowed("PR"),
        "VI" => LogString::Borrowed("VI"),
        "UM" => LogString::Borrowed("UM"),
        "AS" => LogString::Borrowed("AS"),
        "CA" => LogString::Borrowed("CA"),
        "US" => LogString::Borrowed("US"),
        "PS" => LogString::Borrowed("PS"),
        "RS" => LogString::Borrowed("RS"),
        "AQ" => LogString::Borrowed("AQ"),
        "SX" => LogString::Borrowed("SX"),
        "CW" => LogString::Borrowed("CW"),
        "BQ" => LogString::Borrowed("BQ"),
        "SS" => LogString::Borrowed("SS"),
        _ => LogString::Owned(country_name.to_string()),
    }
}

pub fn get_static_country_name(country_name: &str) -> LogString {
    match country_name {
        "" => LogString::Borrowed(""),
        "Rwanda" => LogString::Borrowed("Rwanda"),
        "Somalia" => LogString::Borrowed("Somalia"),
        "Yemen" => LogString::Borrowed("Yemen"),
        "Iraq" => LogString::Borrowed("Iraq"),
        "Saudi Arabia" => LogString::Borrowed("Saudi Arabia"),
        "Iran" => LogString::Borrowed("Iran"),
        "Cyprus" => LogString::Borrowed("Cyprus"),
        "Tanzania" => LogString::Borrowed("Tanzania"),
        "Syria" => LogString::Borrowed("Syria"),
        "Armenia" => LogString::Borrowed("Armenia"),
        "Kenya" => LogString::Borrowed("Kenya"),
        "DR Congo" => LogString::Borrowed("DR Congo"),
        "Djibouti" => LogString::Borrowed("Djibouti"),
        "Uganda" => LogString::Borrowed("Uganda"),
        "Central African Republic" => LogString::Borrowed("Central African Republic"),
        "Seychelles" => LogString::Borrowed("Seychelles"),
        "Jordan" => LogString::Borrowed("Jordan"),
        "Lebanon" => LogString::Borrowed("Lebanon"),
        "Kuwait" => LogString::Borrowed("Kuwait"),
        "Oman" => LogString::Borrowed("Oman"),
        "Qatar" => LogString::Borrowed("Qatar"),
        "Bahrain" => LogString::Borrowed("Bahrain"),
        "United Arab Emirates" => LogString::Borrowed("United Arab Emirates"),
        "Israel" => LogString::Borrowed("Israel"),
        "Turkey" => LogString::Borrowed("Turkey"),
        "Ethiopia" => LogString::Borrowed("Ethiopia"),
        "Eritrea" => LogString::Borrowed("Eritrea"),
        "Egypt" => LogString::Borrowed("Egypt"),
        "Sudan" => LogString::Borrowed("Sudan"),
        "Greece" => LogString::Borrowed("Greece"),
        "Burundi" => LogString::Borrowed("Burundi"),
        "Estonia" => LogString::Borrowed("Estonia"),
        "Latvia" => LogString::Borrowed("Latvia"),
        "Azerbaijan" => LogString::Borrowed("Azerbaijan"),
        "Lithuania" => LogString::Borrowed("Lithuania"),
        "Svalbard and Jan Mayen" => LogString::Borrowed("Svalbard and Jan Mayen"),
        "Georgia" => LogString::Borrowed("Georgia"),
        "Moldova" => LogString::Borrowed("Moldova"),
        "Belarus" => LogString::Borrowed("Belarus"),
        "Finland" => LogString::Borrowed("Finland"),
        "Åland Islands" => LogString::Borrowed("Åland Islands"),
        "Ukraine" => LogString::Borrowed("Ukraine"),
        "North Macedonia" => LogString::Borrowed("North Macedonia"),
        "Hungary" => LogString::Borrowed("Hungary"),
        "Bulgaria" => LogString::Borrowed("Bulgaria"),
        "Albania" => LogString::Borrowed("Albania"),
        "Poland" => LogString::Borrowed("Poland"),
        "Romania" => LogString::Borrowed("Romania"),
        "Kosovo" => LogString::Borrowed("Kosovo"),
        "Zimbabwe" => LogString::Borrowed("Zimbabwe"),
        "Zambia" => LogString::Borrowed("Zambia"),
        "Comoros" => LogString::Borrowed("Comoros"),
        "Malawi" => LogString::Borrowed("Malawi"),
        "Lesotho" => LogString::Borrowed("Lesotho"),
        "Botswana" => LogString::Borrowed("Botswana"),
        "Mauritius" => LogString::Borrowed("Mauritius"),
        "Eswatini" => LogString::Borrowed("Eswatini"),
        "Réunion" => LogString::Borrowed("Réunion"),
        "South Africa" => LogString::Borrowed("South Africa"),
        "Mayotte" => LogString::Borrowed("Mayotte"),
        "Mozambique" => LogString::Borrowed("Mozambique"),
        "Madagascar" => LogString::Borrowed("Madagascar"),
        "Afghanistan" => LogString::Borrowed("Afghanistan"),
        "Pakistan" => LogString::Borrowed("Pakistan"),
        "Bangladesh" => LogString::Borrowed("Bangladesh"),
        "Turkmenistan" => LogString::Borrowed("Turkmenistan"),
        "Tajikistan" => LogString::Borrowed("Tajikistan"),
        "Sri Lanka" => LogString::Borrowed("Sri Lanka"),
        "Bhutan" => LogString::Borrowed("Bhutan"),
        "India" => LogString::Borrowed("India"),
        "Maldives" => LogString::Borrowed("Maldives"),
        "British Indian Ocean Territory" => LogString::Borrowed("British Indian Ocean Territory"),
        "Nepal" => LogString::Borrowed("Nepal"),
        "Myanmar" => LogString::Borrowed("Myanmar"),
        "Uzbekistan" => LogString::Borrowed("Uzbekistan"),
        "Kazakhstan" => LogString::Borrowed("Kazakhstan"),
        "Kyrgyzstan" => LogString::Borrowed("Kyrgyzstan"),
        "French Southern Territories" => LogString::Borrowed("French Southern Territories"),
        "Heard and McDonald Islands" => LogString::Borrowed("Heard and McDonald Islands"),
        "Cocos (Keeling) Islands" => LogString::Borrowed("Cocos (Keeling) Islands"),
        "Palau" => LogString::Borrowed("Palau"),
        "Vietnam" => LogString::Borrowed("Vietnam"),
        "Thailand" => LogString::Borrowed("Thailand"),
        "Indonesia" => LogString::Borrowed("Indonesia"),
        "Laos" => LogString::Borrowed("Laos"),
        "Taiwan" => LogString::Borrowed("Taiwan"),
        "Philippines" => LogString::Borrowed("Philippines"),
        "Malaysia" => LogString::Borrowed("Malaysia"),
        "China" => LogString::Borrowed("China"),
        "Hong Kong" => LogString::Borrowed("Hong Kong"),
        "Brunei" => LogString::Borrowed("Brunei"),
        "Macao" => LogString::Borrowed("Macao"),
        "Cambodia" => LogString::Borrowed("Cambodia"),
        "South Korea" => LogString::Borrowed("South Korea"),
        "Japan" => LogString::Borrowed("Japan"),
        "North Korea" => LogString::Borrowed("North Korea"),
        "Singapore" => LogString::Borrowed("Singapore"),
        "Cook Islands" => LogString::Borrowed("Cook Islands"),
        "Timor-Leste" => LogString::Borrowed("Timor-Leste"),
        "Russia" => LogString::Borrowed("Russia"),
        "Mongolia" => LogString::Borrowed("Mongolia"),
        "Australia" => LogString::Borrowed("Australia"),
        "Christmas Island" => LogString::Borrowed("Christmas Island"),
        "Marshall Islands" => LogString::Borrowed("Marshall Islands"),
        "Federated States of Micronesia" => LogString::Borrowed("Federated States of Micronesia"),
        "Papua New Guinea" => LogString::Borrowed("Papua New Guinea"),
        "Solomon Islands" => LogString::Borrowed("Solomon Islands"),
        "Tuvalu" => LogString::Borrowed("Tuvalu"),
        "Nauru" => LogString::Borrowed("Nauru"),
        "Vanuatu" => LogString::Borrowed("Vanuatu"),
        "New Caledonia" => LogString::Borrowed("New Caledonia"),
        "Norfolk Island" => LogString::Borrowed("Norfolk Island"),
        "New Zealand" => LogString::Borrowed("New Zealand"),
        "Fiji" => LogString::Borrowed("Fiji"),
        "Libya" => LogString::Borrowed("Libya"),
        "Cameroon" => LogString::Borrowed("Cameroon"),
        "Senegal" => LogString::Borrowed("Senegal"),
        "Congo Republic" => LogString::Borrowed("Congo Republic"),
        "Portugal" => LogString::Borrowed("Portugal"),
        "Liberia" => LogString::Borrowed("Liberia"),
        "Ivory Coast" => LogString::Borrowed("Ivory Coast"),
        "Ghana" => LogString::Borrowed("Ghana"),
        "Equatorial Guinea" => LogString::Borrowed("Equatorial Guinea"),
        "Nigeria" => LogString::Borrowed("Nigeria"),
        "Burkina Faso" => LogString::Borrowed("Burkina Faso"),
        "Togo" => LogString::Borrowed("Togo"),
        "Guinea-Bissau" => LogString::Borrowed("Guinea-Bissau"),
        "Mauritania" => LogString::Borrowed("Mauritania"),
        "Benin" => LogString::Borrowed("Benin"),
        "Gabon" => LogString::Borrowed("Gabon"),
        "Sierra Leone" => LogString::Borrowed("Sierra Leone"),
        "São Tomé and Príncipe" => LogString::Borrowed("São Tomé and Príncipe"),
        "Gibraltar" => LogString::Borrowed("Gibraltar"),
        "Gambia" => LogString::Borrowed("Gambia"),
        "Guinea" => LogString::Borrowed("Guinea"),
        "Chad" => LogString::Borrowed("Chad"),
        "Niger" => LogString::Borrowed("Niger"),
        "Mali" => LogString::Borrowed("Mali"),
        "Western Sahara" => LogString::Borrowed("Western Sahara"),
        "Tunisia" => LogString::Borrowed("Tunisia"),
        "Spain" => LogString::Borrowed("Spain"),
        "Morocco" => LogString::Borrowed("Morocco"),
        "Malta" => LogString::Borrowed("Malta"),
        "Algeria" => LogString::Borrowed("Algeria"),
        "Faroe Islands" => LogString::Borrowed("Faroe Islands"),
        "Denmark" => LogString::Borrowed("Denmark"),
        "Iceland" => LogString::Borrowed("Iceland"),
        "United Kingdom" => LogString::Borrowed("United Kingdom"),
        "Switzerland" => LogString::Borrowed("Switzerland"),
        "Sweden" => LogString::Borrowed("Sweden"),
        "Netherlands" => LogString::Borrowed("Netherlands"),
        "Austria" => LogString::Borrowed("Austria"),
        "Belgium" => LogString::Borrowed("Belgium"),
        "Germany" => LogString::Borrowed("Germany"),
        "Luxembourg" => LogString::Borrowed("Luxembourg"),
        "Ireland" => LogString::Borrowed("Ireland"),
        "Monaco" => LogString::Borrowed("Monaco"),
        "France" => LogString::Borrowed("France"),
        "Andorra" => LogString::Borrowed("Andorra"),
        "Liechtenstein" => LogString::Borrowed("Liechtenstein"),
        "Jersey" => LogString::Borrowed("Jersey"),
        "Isle of Man" => LogString::Borrowed("Isle of Man"),
        "Guernsey" => LogString::Borrowed("Guernsey"),
        "Slovakia" => LogString::Borrowed("Slovakia"),
        "Czechia" => LogString::Borrowed("Czechia"),
        "Norway" => LogString::Borrowed("Norway"),
        "Vatican City" => LogString::Borrowed("Vatican City"),
        "San Marino" => LogString::Borrowed("San Marino"),
        "Italy" => LogString::Borrowed("Italy"),
        "Slovenia" => LogString::Borrowed("Slovenia"),
        "Montenegro" => LogString::Borrowed("Montenegro"),
        "Croatia" => LogString::Borrowed("Croatia"),
        "Bosnia and Herzegovina" => LogString::Borrowed("Bosnia and Herzegovina"),
        "Angola" => LogString::Borrowed("Angola"),
        "Namibia" => LogString::Borrowed("Namibia"),
        "Saint Helena" => LogString::Borrowed("Saint Helena"),
        "Bouvet Island" => LogString::Borrowed("Bouvet Island"),
        "Barbados" => LogString::Borrowed("Barbados"),
        "Cabo Verde" => LogString::Borrowed("Cabo Verde"),
        "Guyana" => LogString::Borrowed("Guyana"),
        "French Guiana" => LogString::Borrowed("French Guiana"),
        "Suriname" => LogString::Borrowed("Suriname"),
        "Saint Pierre and Miquelon" => LogString::Borrowed("Saint Pierre and Miquelon"),
        "Greenland" => LogString::Borrowed("Greenland"),
        "Paraguay" => LogString::Borrowed("Paraguay"),
        "Uruguay" => LogString::Borrowed("Uruguay"),
        "Brazil" => LogString::Borrowed("Brazil"),
        "Falkland Islands" => LogString::Borrowed("Falkland Islands"),
        "South Georgia and the South Sandwich Islands" => {
            LogString::Borrowed("South Georgia and the South Sandwich Islands")
        }
        "Jamaica" => LogString::Borrowed("Jamaica"),
        "Dominican Republic" => LogString::Borrowed("Dominican Republic"),
        "Cuba" => LogString::Borrowed("Cuba"),
        "Martinique" => LogString::Borrowed("Martinique"),
        "Bahamas" => LogString::Borrowed("Bahamas"),
        "Bermuda" => LogString::Borrowed("Bermuda"),
        "Anguilla" => LogString::Borrowed("Anguilla"),
        "Trinidad and Tobago" => LogString::Borrowed("Trinidad and Tobago"),
        "St Kitts and Nevis" => LogString::Borrowed("St Kitts and Nevis"),
        "Dominica" => LogString::Borrowed("Dominica"),
        "Antigua and Barbuda" => LogString::Borrowed("Antigua and Barbuda"),
        "Saint Lucia" => LogString::Borrowed("Saint Lucia"),
        "Turks and Caicos Islands" => LogString::Borrowed("Turks and Caicos Islands"),
        "Aruba" => LogString::Borrowed("Aruba"),
        "British Virgin Islands" => LogString::Borrowed("British Virgin Islands"),
        "St Vincent and Grenadines" => LogString::Borrowed("St Vincent and Grenadines"),
        "Montserrat" => LogString::Borrowed("Montserrat"),
        "Saint Martin" => LogString::Borrowed("Saint Martin"),
        "Saint Barthélemy" => LogString::Borrowed("Saint Barthélemy"),
        "Guadeloupe" => LogString::Borrowed("Guadeloupe"),
        "Grenada" => LogString::Borrowed("Grenada"),
        "Cayman Islands" => LogString::Borrowed("Cayman Islands"),
        "Belize" => LogString::Borrowed("Belize"),
        "El Salvador" => LogString::Borrowed("El Salvador"),
        "Guatemala" => LogString::Borrowed("Guatemala"),
        "Honduras" => LogString::Borrowed("Honduras"),
        "Nicaragua" => LogString::Borrowed("Nicaragua"),
        "Costa Rica" => LogString::Borrowed("Costa Rica"),
        "Venezuela" => LogString::Borrowed("Venezuela"),
        "Ecuador" => LogString::Borrowed("Ecuador"),
        "Colombia" => LogString::Borrowed("Colombia"),
        "Panama" => LogString::Borrowed("Panama"),
        "Haiti" => LogString::Borrowed("Haiti"),
        "Argentina" => LogString::Borrowed("Argentina"),
        "Chile" => LogString::Borrowed("Chile"),
        "Bolivia" => LogString::Borrowed("Bolivia"),
        "Peru" => LogString::Borrowed("Peru"),
        "Mexico" => LogString::Borrowed("Mexico"),
        "French Polynesia" => LogString::Borrowed("French Polynesia"),
        "Pitcairn Islands" => LogString::Borrowed("Pitcairn Islands"),
        "Kiribati" => LogString::Borrowed("Kiribati"),
        "Tokelau" => LogString::Borrowed("Tokelau"),
        "Tonga" => LogString::Borrowed("Tonga"),
        "Wallis and Futuna" => LogString::Borrowed("Wallis and Futuna"),
        "Samoa" => LogString::Borrowed("Samoa"),
        "Niue" => LogString::Borrowed("Niue"),
        "Northern Mariana Islands" => LogString::Borrowed("Northern Mariana Islands"),
        "Guam" => LogString::Borrowed("Guam"),
        "Puerto Rico" => LogString::Borrowed("Puerto Rico"),
        "U.S. Virgin Islands" => LogString::Borrowed("U.S. Virgin Islands"),
        "U.S. Outlying Islands" => LogString::Borrowed("U.S. Outlying Islands"),
        "American Samoa" => LogString::Borrowed("American Samoa"),
        "Canada" => LogString::Borrowed("Canada"),
        "United States" => LogString::Borrowed("United States"),
        "Palestine" => LogString::Borrowed("Palestine"),
        "Serbia" => LogString::Borrowed("Serbia"),
        "Antarctica" => LogString::Borrowed("Antarctica"),
        "Sint Maarten" => LogString::Borrowed("Sint Maarten"),
        "Curaçao" => LogString::Borrowed("Curaçao"),
        "\"Bonaire, Sint Eustatius, and Saba\"" => {
            LogString::Borrowed("Bonaire, Sint Eustatius, and Saba")
        }
        "South Sudan" => LogString::Borrowed("South Sudan"),
        _ => LogString::Owned(country_name.replace('"', "")),
    }
}

pub fn static_principal_asns(asn: &str) -> LogString {
    match asn {
        "ATT-INTERNET4" => LogString::Borrowed("ATT-INTERNET4"),
        "UUNET" => LogString::Borrowed("UUNET"),
        "Chinanet" => LogString::Borrowed("Chinanet"),
        "COGENT-174" => LogString::Borrowed("COGENT-174"),
        "Turk Telekom" => LogString::Borrowed("Turk Telekom"),
        "DNIC-ASBLK-00721-00726" => LogString::Borrowed("DNIC-ASBLK-00721-00726"),
        "Korea Telecom" => LogString::Borrowed("Korea Telecom"),
        "LEVEL3" => LogString::Borrowed("LEVEL3"),
        "CENTURYLINK-US-LEGACY-QWEST" => LogString::Borrowed("CENTURYLINK-US-LEGACY-QWEST"),
        "Akamai International B.V." => LogString::Borrowed("Akamai International B.V."),
        "LVLT-3549" => LogString::Borrowed("LVLT-3549"),
        "SPRINTLINK" => LogString::Borrowed("SPRINTLINK"),
        "AKAMAI-AS" => LogString::Borrowed("AKAMAI-AS"),
        "COMCAST-7922" => LogString::Borrowed("COMCAST-7922"),
        "AMAZON-02" => LogString::Borrowed("AMAZON-02"),
        "SK Broadband Co Ltd" => LogString::Borrowed("SK Broadband Co Ltd"),
        "LG DACOM Corporation" => LogString::Borrowed("LG DACOM Corporation"),
        "DNIC-AS-00749" => LogString::Borrowed("DNIC-AS-00749"),
        "Rostelecom" => LogString::Borrowed("Rostelecom"),
        "FRONTIER-FRTR" => LogString::Borrowed("FRONTIER-FRTR"),
        "China Mobile Communications Group Co., Ltd." => {
            LogString::Borrowed("China Mobile Communications Group Co., Ltd.")
        }
        "WINDSTREAM" => LogString::Borrowed("WINDSTREAM"),
        "CENTURYLINK-LEGACY-SAVVIS" => LogString::Borrowed("CENTURYLINK-LEGACY-SAVVIS"),
        "GTT Communications Inc." => LogString::Borrowed("GTT Communications Inc."),
        "ASN-CXA-ALL-CCI-22773-RDC" => LogString::Borrowed("ASN-CXA-ALL-CCI-22773-RDC"),
        "M247 Europe SRL" => LogString::Borrowed("M247 Europe SRL"),
        "XO-AS15" => LogString::Borrowed("XO-AS15"),
        "EGIHOSTING" => LogString::Borrowed("EGIHOSTING"),
        "China Mobile communications corporation" => {
            LogString::Borrowed("China Mobile communications corporation")
        }
        "DNIC-ASBLK-05800-06055" => LogString::Borrowed("DNIC-ASBLK-05800-06055"),
        "ZAYO-6461" => LogString::Borrowed("ZAYO-6461"),
        "DNIC-ASBLK-27032-27159" => LogString::Borrowed("DNIC-ASBLK-27032-27159"),
        "CHINA UNICOM China169 Backbone" => LogString::Borrowed("CHINA UNICOM China169 Backbone"),
        "Ipxo Uk Limited" => LogString::Borrowed("Ipxo Uk Limited"),
        "PVimpelCom" => LogString::Borrowed("PVimpelCom"),
        "BELLSOUTH-NET-BLK" => LogString::Borrowed("BELLSOUTH-NET-BLK"),
        "DNIC-ASBLK-00306-00371" => LogString::Borrowed("DNIC-ASBLK-00306-00371"),
        "NTT-LTD-2914" => LogString::Borrowed("NTT-LTD-2914"),
        "BACOM" => LogString::Borrowed("BACOM"),
        "Telmex Colombia S.A." => LogString::Borrowed("Telmex Colombia S.A."),
        "JSC ER-Telecom Holding" => LogString::Borrowed("JSC ER-Telecom Holding"),
        "A1 Bulgaria EAD" => LogString::Borrowed("A1 Bulgaria EAD"),
        "UCSD" => LogString::Borrowed("UCSD"),
        "AFCONC-BLOCK1-AS" => LogString::Borrowed("AFCONC-BLOCK1-AS"),
        "Mega Cable, S.A. de C.V." => LogString::Borrowed("Mega Cable, S.A. de C.V."),
        "Orange" => LogString::Borrowed("Orange"),
        "TELUS Communications" => LogString::Borrowed("TELUS Communications"),
        "China Unicom Beijing Province Network" => {
            LogString::Borrowed("China Unicom Beijing Province Network")
        }
        "OOO Network of data-centers Selectel" => {
            LogString::Borrowed("OOO Network of data-centers Selectel")
        }
        "CHARTER-20115" => LogString::Borrowed("CHARTER-20115"),
        "Uninet S.A. de C.V." => LogString::Borrowed("Uninet S.A. de C.V."),
        "Datacamp Limited" => LogString::Borrowed("Datacamp Limited"),
        "TOTAL PLAY TELECOMUNICACIONES SA DE CV" => {
            LogString::Borrowed("TOTAL PLAY TELECOMUNICACIONES SA DE CV")
        }
        "TPG Telecom Limited" => LogString::Borrowed("TPG Telecom Limited"),
        "Superonline Iletisim Hizmetleri A.S." => {
            LogString::Borrowed("Superonline Iletisim Hizmetleri A.S.")
        }
        "TELEFONICA BRASIL S.A" => LogString::Borrowed("TELEFONICA BRASIL S.A"),
        "Host Europe GmbH" => LogString::Borrowed("Host Europe GmbH"),
        "PJSC MegaFon" => LogString::Borrowed("PJSC MegaFon"),
        "China Telecom Group" => LogString::Borrowed("China Telecom Group"),
        "TWC-10796-MIDWEST" => LogString::Borrowed("TWC-10796-MIDWEST"),
        "CABLEONE" => LogString::Borrowed("CABLEONE"),
        "HostRoyale Technologies Pvt Ltd" => LogString::Borrowed("HostRoyale Technologies Pvt Ltd"),
        "Bharti Airtel Ltd., Telemedia Services" => {
            LogString::Borrowed("Bharti Airtel Ltd., Telemedia Services")
        }
        "DACOM-PUBNETPLUS" => LogString::Borrowed("DACOM-PUBNETPLUS"),
        "Axtel, S.A.B. de C.V." => LogString::Borrowed("Axtel, S.A.B. de C.V."),
        "SERVER-MANIA" => LogString::Borrowed("SERVER-MANIA"),
        "Sify Limited" => LogString::Borrowed("Sify Limited"),
        "MTS PJSC" => LogString::Borrowed("MTS PJSC"),
        "Iran Telecommunication Company PJS" => {
            LogString::Borrowed("Iran Telecommunication Company PJS")
        }
        "MICROSOFT-CORP-MSN-AS-BLOCK" => LogString::Borrowed("MICROSOFT-CORP-MSN-AS-BLOCK"),
        "AMAZON-AES" => LogString::Borrowed("AMAZON-AES"),
        "China Networks Inter-Exchange" => LogString::Borrowed("China Networks Inter-Exchange"),
        "ASN-QUADRANET-GLOBAL" => LogString::Borrowed("ASN-QUADRANET-GLOBAL"),
        "Deutsche Telekom AG" => LogString::Borrowed("Deutsche Telekom AG"),
        "Avatel Telecom, SA" => LogString::Borrowed("Avatel Telecom, SA"),
        "Vodafone Net Iletisim Hizmetleri Anonim Sirketi" => {
            LogString::Borrowed("Vodafone Net Iletisim Hizmetleri Anonim Sirketi")
        }
        "CELLCO-PART" => LogString::Borrowed("CELLCO-PART"),
        "Clouvider Limited" => LogString::Borrowed("Clouvider Limited"),
        "British Telecommunications PLC" => LogString::Borrowed("British Telecommunications PLC"),
        "BHARTI Airtel Ltd." => LogString::Borrowed("BHARTI Airtel Ltd."),
        "COGECO-PEER1" => LogString::Borrowed("COGECO-PEER1"),
        "Telecom Italia" => LogString::Borrowed("Telecom Italia"),
        "ZEN-ECN" => LogString::Borrowed("ZEN-ECN"),
        _ => LogString::Owned(asn.to_string()),
    }
}

pub async fn join_path_files(dir_paths: Vec<PathBuf>) -> TempResult<PathBuf> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let extract_dir = std::env::temp_dir().join(format!("geoip_{}_db", nanos));
    let extract_dir = std::path::Path::new(&extract_dir).to_path_buf();
    tokio::fs::create_dir(&extract_dir).await?;
    for path in dir_paths {
        let mut files = tokio::fs::read_dir(path).await?;
        loop {
            let file = match files.next_entry().await? {
                Some(v) => v,
                None => break,
            };
            tokio::fs::copy(file.path(), extract_dir.join(file.file_name())).await?;
        }
    }
    Ok(extract_dir)
}

fn _estimated_size(data: &HashMap<String, GeoIpInfo>) -> usize {
    let mut returned =
        data.capacity() * 16 * 10 / 11 + std::mem::size_of::<HashMap<String, GeoIpInfo>>();
    for (key, element) in data {
        returned += key.len();
        returned += std::mem::size_of::<GeoIpInfo>();
        returned += element.city.len();
        returned += element.country.len();
        returned += element.isp.len();
    }
    returned
}

#[test]
fn test_split_csv() {
    let res = split_column_values(r#"1.9.0.0/16,4788,"TM Net, Internet Service Provider""#);
    assert_eq!(&"1.9.0.0/16", res.get(0).unwrap());
    assert_eq!(&"4788", res.get(1).unwrap());
    assert_eq!(&"TM Net, Internet Service Provider", res.get(2).unwrap());

    let res = split_column_values(
        r#"2057192,en,OC,Oceania,AU,Australia,SA,"South Australia",,,Yunta,,Australia/Adelaide,0"#,
    );
    assert_eq!(&"2057192", res.get(0).unwrap());
    assert_eq!(&"Australia/Adelaide", res.get(12).unwrap());
}
