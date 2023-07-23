use usiem::{
    prelude::{
        ip_net::IpNetSynDataset,
        task::{SiemTaskData, SiemTaskResult, TaskDefinition, TaskFireMode},
        SiemDatasetType, SiemError, SiemIp,
    },
    utilities::types::LogString,
};

use crate::{
    aws::{get_aws_ips, static_region, static_service},
    azure::{self, get_azure_ips},
    common::{parse_ip4_network, parse_ip6_network},
};

pub fn cloud_provider_definition() -> TaskDefinition {
    TaskDefinition::new(
        SiemTaskData::UPDATE_CLOUD_PROVIDER,
        LogString::Borrowed("CloudProvider"),
        LogString::Borrowed("Update cloud provider dataset with AWS and Azure"),
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
                        "Cloud service dataset is not supported by this SIEM implementation"
                    )))
                        }
                    },
                    None => {
                        return Err(SiemError::Task(format!(
                            "Cloud service dataset is not supported by this SIEM implementation"
                        )))
                    }
                };
            let cloud_provider: IpNetSynDataset =
                match datasets.get(&SiemDatasetType::IpCloudProvider) {
                    Some(v) => match v.clone().try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(SiemError::Task(format!(
                        "Cloud provider dataset is not supported by this SIEM implementation"
                    )))
                        }
                    },
                    None => {
                        return Err(SiemError::Task(format!(
                            "Cloud provider dataset is not supported by this SIEM implementation"
                        )))
                    }
                };

            Ok(Box::pin(async move {
                match get_aws_ips().await {
                    Ok(aws_ranges) => {
                        for service in aws_ranges.prefixes {
                            if let Some((ip, net)) = parse_ip4_network(&service.ip_prefix) {
                                if !service.region.is_empty() {
                                    cloud_provider.insert(
                                        SiemIp::V4(ip),
                                        net,
                                        static_region(service.region),
                                    );
                                }
                                if service.service != "AMAZON" && !service.service.is_empty() {
                                    cloud_service.insert(
                                        SiemIp::V4(ip),
                                        net,
                                        static_service(service.service),
                                    );
                                }
                            }
                        }
                        for service in aws_ranges.ipv6_prefixes {
                            if let Some((ip, net)) = parse_ip6_network(&service.ipv6_prefix) {
                                if !service.region.is_empty() {
                                    cloud_provider.insert(
                                        SiemIp::V6(ip),
                                        net,
                                        static_region(service.region),
                                    );
                                }
                                if service.service != "AMAZON" && !service.service.is_empty() {
                                    cloud_service.insert(
                                        SiemIp::V6(ip),
                                        net,
                                        static_service(service.service),
                                    );
                                }
                            }
                        }
                    }
                    Err(_) => {}
                };
                match get_azure_ips().await {
                    Ok(azure_ranges) => {
                        for service in azure_ranges.values {
                            for prefix in service.properties.address_prefixes {
                                match parse_ip4_network(&prefix) {
                                    Some((ip, net)) => {
                                        if !&service.properties.region.is_empty() {
                                            cloud_provider.insert(
                                                SiemIp::V4(ip),
                                                net,
                                                azure::static_region(&service.properties.region),
                                            );
                                        }
                                        if !&service.properties.system_service.is_empty() {
                                            cloud_service.insert(
                                                SiemIp::V4(ip),
                                                net,
                                                azure::static_service(
                                                    &service.properties.system_service,
                                                ),
                                            );
                                        }
                                    }
                                    None => match parse_ip6_network(&prefix) {
                                        Some((ip, net)) => {
                                            if !&service.properties.region.is_empty() {
                                                cloud_provider.insert(
                                                    SiemIp::V6(ip),
                                                    net,
                                                    azure::static_region(
                                                        &service.properties.region,
                                                    ),
                                                );
                                            }
                                            if !&service.properties.system_service.is_empty() {
                                                cloud_service.insert(
                                                    SiemIp::V6(ip),
                                                    net,
                                                    azure::static_service(
                                                        &service.properties.system_service,
                                                    ),
                                                );
                                            }
                                        }
                                        None => {}
                                    },
                                }
                            }
                        }
                    }
                    Err(_) => {}
                };

                SiemTaskResult {
                    data: Some(Ok(format!("Correctly updated IpCloudService and IpCloudProvider"))),
                    id: task.id,
                }
            }))
        },
    )
}
