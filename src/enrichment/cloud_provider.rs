use usiem::{
    prelude::{ip_net::IpNetSynDataset, LogEnrichment, SiemDatasetType, SiemField, SiemIp},
    utilities::types::LogString,
};

#[derive(Clone)]
pub struct CloudProviderEnricher {}

impl LogEnrichment for CloudProviderEnricher {
    fn enrich(
        &self,
        mut log: usiem::prelude::SiemLog,
        datasets: &usiem::prelude::holder::DatasetHolder,
    ) -> usiem::prelude::SiemLog {
        let cloud_provider: &IpNetSynDataset = match datasets.get(&SiemDatasetType::IpCloudProvider)
        {
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
            match cloud_provider.get(ip) {
                Some(cloud_info) => {
                    new_fields.push((
                        LogString::Owned(format!("{}.cloud.provider", &field_name[..])),
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
        "CloudProviderEnricher"
    }

    fn description(&self) -> &'static str {
        "Adds cloud provider information like Google, Azure or AWS to each IP field"
    }
}
