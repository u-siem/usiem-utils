use usiem::{
    prelude::{
        geo_ip::GeoIpSynDataset,
        task::{SiemTaskData, SiemTaskResult, TaskDefinition, TaskFireMode},
        text_map::TextMapSynDataset,
        SiemDatasetType, SiemError,
    },
    utilities::types::LogString,
};

use crate::maxmind::{
    download_maxmind_geo_litle2_asn, download_maxmind_geo_litle2_city,
    download_maxmind_geo_litle2_country, extract_zip_db, join_path_files,
    process_maxmind_geo_lite2_csv,
};

pub fn geoip_definition() -> TaskDefinition {
    TaskDefinition::new(
        SiemTaskData::UPDATE_GEOIP,
        LogString::Borrowed("UpdateGeoIP"),
        LogString::Borrowed("Update geo ip dataset with maxmind database"),
        usiem::prelude::UserRole::Administrator,
        TaskFireMode::Repetitive(86_400_000),
        600_000,
        |task, datasets| {
            let secrets: TextMapSynDataset = match datasets.get(&SiemDatasetType::Secrets(
                LogString::Borrowed("UpdateGeoIP"),
            )) {
                Some(v) => match v.clone().try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(SiemError::Task(format!(
                            "Secrets dataset is not supported by this SIEM implementation"
                        )))
                    }
                },
                None => {
                    return Err(SiemError::Task(format!(
                        "Secrets dataset is not supported by this SIEM implementation"
                    )))
                }
            };
            let geoip: GeoIpSynDataset = match datasets.get(&SiemDatasetType::GeoIp) {
                Some(v) => match v.clone().try_into() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(SiemError::Task(format!(
                            "GeoIpDataset is not supported by this SIEM implementation"
                        )))
                    }
                },
                None => {
                    return Err(SiemError::Task(format!(
                        "GeoIpDataset is not supported by this SIEM implementation"
                    )))
                }
            };
            let maxmind_api = match secrets.get(&LogString::Borrowed("MAXMIND_API")) {
                Some(v) => v.to_string(),
                None => return Err(SiemError::Task(format!("Cannot find MAXMIND_API secret"))),
            };
            let language = match secrets.get(&LogString::Borrowed("MAXMIND_LANGUAGE")) {
                Some(v) => v.to_lowercase(),
                None => format!("en")
            };

            #[cfg(feature = "slow_geoip")]
            let slow_location = {
                let config: TextMapSynDataset = match datasets.get(&SiemDatasetType::Configuration) {
                    Some(v) => match v.clone().try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(SiemError::Task(format!(
                                "Configuration is not supported by this SIEM implementation"
                            )))
                        }
                    },
                    None => {
                        return Err(SiemError::Task(format!(
                            "Configuration is not supported by this SIEM implementation"
                        )))
                    }
                };
                let location = match config.get(&LogString::Borrowed("SLOW_GEO_IP")) {
                    Some(v) => v.to_string(),
                    None => return Err(SiemError::Task(format!(
                        "SLOW_GEO_IP configuration is not setted, cannot update dataset"
                    )))
                };
                location
            };

            Ok(Box::pin(async move {
                let asn_path = match download_maxmind_geo_litle2_asn(&maxmind_api).await {
                    Ok(v) => v,
                    Err(_err) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot download maxmind ASN"))),
                            id: task.id,
                        }
                    }
                };
                let city_path = match download_maxmind_geo_litle2_city(&maxmind_api).await {
                    Ok(v) => v,
                    Err(_err) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot download maxmind City"))),
                            id: task.id,
                        }
                    }
                };

                let country_path = match download_maxmind_geo_litle2_country(&maxmind_api).await {
                    Ok(v) => v,
                    Err(_err) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot download maxmind Country"))),
                            id: task.id,
                        }
                    }
                };
                let city_path = match extract_zip_db(&city_path).await {
                    Ok(v) => v,
                    Err(_) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot extract city database"))),
                            id: task.id,
                        }
                    }
                };
                let country_path = match extract_zip_db(&country_path).await {
                    Ok(v) => v,
                    Err(_) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot extract country database"))),
                            id: task.id,
                        }
                    }
                };
                let asn_path = match extract_zip_db(&asn_path).await {
                    Ok(v) => v,
                    Err(_) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot extract ASN database"))),
                            id: task.id,
                        }
                    }
                };
                let new_path = match join_path_files(vec![city_path, country_path, asn_path]).await
                {
                    Ok(v) => v,
                    Err(_) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot copy database files"))),
                            id: task.id,
                        }
                    }
                };
                #[cfg(not(feature = "slow_geoip"))]
                let tsk = process_maxmind_geo_lite2_csv(new_path, true, &language);
                #[cfg(feature = "slow_geoip")]
                let tsk = process_maxmind_geo_lite2_csv(new_path, true, &language, &slow_location);
                let dataset = match tsk.await {
                    Ok(v) => v,
                    Err(_) => {
                        return SiemTaskResult {
                            data: Some(Err(format!("Cannot process database files"))),
                            id: task.id,
                        }
                    }
                };
                geoip.full_update(dataset);
                SiemTaskResult {
                    data: Some(Ok(format!("Correctly updated GeoIpDatabase"))),
                    id: task.id,
                }
            }))
        },
    )
}
