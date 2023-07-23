use usiem::{
    prelude::{ip_net::IpNetSynDataset, LogEnrichment, SiemDatasetType, SiemField, SiemIp},
    utilities::types::LogString,
};

#[derive(Clone)]
pub struct CloudServiceEnricher {}

impl LogEnrichment for CloudServiceEnricher {
    fn enrich(
        &self,
        mut log: usiem::prelude::SiemLog,
        datasets: &usiem::prelude::holder::DatasetHolder,
    ) -> usiem::prelude::SiemLog {
        let cloud_service: &IpNetSynDataset = match datasets.get(&SiemDatasetType::IpCloudService) {
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
            match cloud_service.get(ip) {
                Some(cloud_info) => {
                    new_fields.push((
                        LogString::Owned(format!("{}.cloud.service.name", &field_name[..])),
                        SiemField::Text(cloud_info.clone()),
                    ));
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
        "CloudServiceEnricher"
    }

    fn description(&self) -> &'static str {
        "Adds cloud service information like O365, Azure or AWS to each IP field"
    }
}
