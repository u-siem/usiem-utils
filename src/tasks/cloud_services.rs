use usiem::{
    prelude::{
        ip_net::IpNetSynDataset,
        task::{SiemTaskData, SiemTaskResult, TaskDefinition, TaskFireMode},
        SiemDatasetType, SiemError, SiemIp,
    },
    utilities::{
        ip_utils::{ipv4_from_str, ipv6_from_str},
        types::LogString,
    },
};

use crate::{
    err::TempErr,
    o365,
};

pub fn cloud_service_definition() -> TaskDefinition {
    TaskDefinition::new(
        SiemTaskData::UPDATE_GEOIP,
        LogString::Borrowed("CloudService"),
        LogString::Borrowed("Update cloud service dataset with O365 IPs"),
        usiem::prelude::UserRole::Administrator,
        TaskFireMode::Repetitive(86400000),
        600_000,
        |task, datasets| {
            let cloud_service: IpNetSynDataset =
                match datasets.get(&SiemDatasetType::IpCloudService) {
                    Some(v) => match v.clone().try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(SiemError::Task(format!(
                                "IpCloudService dataset is not supported by this SIEM implementation"
                            )))
                        }
                    },
                    None => {
                        return Err(SiemError::Task(format!(
                            "IpCloudService is not supported by this SIEM implementation"
                        )))
                    }
                };

            Ok(Box::pin(async move {
                let _o365_res = process_o365(&cloud_service);

                SiemTaskResult {
                    data: Some(Ok(format!("Correctly updated CloudService"))),
                    id: task.id,
                }
            }))
        },
    )
}

pub async fn process_o365(dataset: &IpNetSynDataset) -> Result<(), TempErr> {
    let res = o365::get_office365_ip().await;
    let res = match res {
        Ok(v) => v,
        Err(err) => return Err(err),
    };
    for service in res {
        for text in &service.ips {
            let (ip, net) = match text.rfind("/") {
                Some(pos) => {
                    let ip = &text[..pos];
                    let net = &text[pos + 1..];
                    let net = match net.parse::<u8>() {
                        Ok(net) => net,
                        Err(_) => continue,
                    };
                    let ip = if ip.contains(":") {
                        match ipv4_from_str(ip) {
                            Ok(v) => SiemIp::V4(v),
                            Err(_) => continue,
                        }
                    } else {
                        match ipv6_from_str(ip) {
                            Ok(v) => SiemIp::V6(v),
                            Err(_) => continue,
                        }
                    };
                    (ip, net)
                }
                None => continue,
            };
            dataset.insert(ip, net, o365::static_service(&service.service_area));
        }
    }
    Ok(())
}
