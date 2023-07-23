use usiem::{
    prelude::{geo_ip::GeoIpSynDataset, LogEnrichment, SiemDatasetType, SiemField, SiemIp},
    utilities::types::LogString,
};

#[derive(Clone)]
pub struct GeoIpEnricher {}

impl LogEnrichment for GeoIpEnricher {
    fn enrich(
        &self,
        mut log: usiem::prelude::SiemLog,
        datasets: &usiem::prelude::holder::DatasetHolder,
    ) -> usiem::prelude::SiemLog {
        let geo_ip: &GeoIpSynDataset = match datasets.get(&SiemDatasetType::GeoIp) {
            Some(v) => match v.try_into() {
                Ok(v) => v,
                Err(_) => return log,
            },
            None => return log,
        };
        let mut new_fields = Vec::with_capacity(32);
        for (field_name, ip_field) in log.ip_fields() {
            let ip: &SiemIp = match ip_field.try_into() {
                Ok(v) => v,
                Err(_) => continue,
            };
            match geo_ip.get(ip) {
                Some(geoip_info) => {
                    if geoip_info.city.len() > 0 {
                        new_fields.push((
                            LogString::Owned(format!("{}.geo.city_name", &field_name[..])),
                            SiemField::Text(geoip_info.city.clone()),
                        ));
                    }
                    if geoip_info.country.len() > 0 {
                        new_fields.push((
                            LogString::Owned(format!("{}.geo.country_name", &field_name[..])),
                            SiemField::Text(geoip_info.country.clone()),
                        ));
                        new_fields.push((
                            LogString::Owned(format!("{}.geo.country_iso_code", &field_name[..])),
                            SiemField::Text(geoip_info.country_iso.clone()),
                        ));
                    }
                    if geoip_info.isp.len() > 0 {
                        new_fields.push((
                            LogString::Owned(format!("{}.as.organization.name", &field_name[..])),
                            SiemField::Text(geoip_info.isp.clone()),
                        ));
                    }
                    if geoip_info.asn > 0 {
                        new_fields.push((
                            LogString::Owned(format!("{}.as.number", &field_name[..])),
                            SiemField::U64(geoip_info.asn as u64),
                        ));
                    }
                    if geoip_info.longitude != 0.0 && geoip_info.latitude != 0.0 {
                        new_fields.push((
                            LogString::Owned(format!("{}.geo.location.lon", &field_name[..])),
                            SiemField::F64(geoip_info.longitude as f64),
                        ));
                        new_fields.push((
                            LogString::Owned(format!("{}.geo.location.lat", &field_name[..])),
                            SiemField::F64(geoip_info.latitude as f64),
                        ));
                    }
                }
                None => continue,
            };
        }
        for (field_name, field_value) in new_fields {
            log.insert(field_name, field_value);
        }
        log
    }

    fn name(&self) -> &'static str {
        "GeoIpEnricher"
    }

    fn description(&self) -> &'static str {
        "Adds geo ip information to each IP field"
    }
}
