use usiem::{
    events::tags::BLOCKED_IP,
    prelude::{
        holder::DatasetHolder, ip_map::IpMapSynDataset, ip_set::IpSetSynDataset,
        text_map::TextMapSynDataset, text_map_list::TextMapListSynDataset,
        text_set::TextSetSynDataset, LogEnrichment, SiemDatasetType, SiemField, SiemIp, SiemLog,
    },
    utilities::types::LogString,
};

#[derive(Clone)]
pub struct BasicIPEnricher {}

impl LogEnrichment for BasicIPEnricher {
    fn enrich(&self, mut log: SiemLog, datasets: &DatasetHolder) -> SiemLog {
        if let Some(fields) = enrich_block_ip(&mut log, datasets) {
            for (name, value) in fields {
                log.insert(name, value);
            }
        }
        if let Some(fields) = enrich_block_domain(&mut log, datasets) {
            for (name, value) in fields {
                log.insert(name, value);
            }
        }
        if let Some(fields) = enrich_mac_ip(&mut log, datasets) {
            for (name, value) in fields {
                log.insert(name, value);
            }
        }
        if let Some(fields) = enrich_asset_with_tags(&mut log, datasets) {
            for (name, value) in fields {
                log.insert(name, value);
            }
        }
        if let Some(fields) = enrich_asset_with_vulnerabilities(&mut log, datasets) {
            for (name, value) in fields {
                log.insert(name, value);
            }
        }
        log
    }

    fn name(&self) -> &'static str {
        "BasicIPEnricher"
    }

    fn description(&self) -> &'static str {
        "Enrich all IP fields. Checks if the IP is in the block list, adds mac and hostname information to the IP..."
    }
}

fn enrich_block_ip(
    log: &mut SiemLog,
    datasets: &DatasetHolder,
) -> Option<Vec<(LogString, SiemField)>> {
    let ip_info: &IpSetSynDataset = datasets.get(&SiemDatasetType::BlockIp)?.try_into().ok()?;
    let mut new_fields = Vec::with_capacity(32);
    let mut block_list = false;
    for (field_name, ip_field) in log.ip_fields() {
        let ip: &SiemIp = match ip_field.try_into() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if ip_info.contains(ip) {
            new_fields.push((
                LogString::Owned(format!("{}.tags", &field_name[..])),
                SiemField::Array(vec![LogString::Borrowed(BLOCKED_IP)]),
            ));
            block_list = true;
        }
    }
    if block_list {
        log.add_tag(BLOCKED_IP);
    }
    Some(new_fields)
}

fn enrich_mac_ip(
    log: &mut SiemLog,
    datasets: &DatasetHolder,
) -> Option<Vec<(LogString, SiemField)>> {
    let mac_info: &IpMapSynDataset = datasets.get(&SiemDatasetType::IpMac)?.try_into().ok()?;
    let host_info: &TextMapSynDataset = datasets.get(&SiemDatasetType::MacHost)?.try_into().ok()?;
    let mut new_fields = Vec::with_capacity(32);

    for (field_name, ip_field) in log.ip_fields() {
        let ip: &SiemIp = match ip_field.try_into() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let mac_addr = match mac_info.get(ip) {
            Some(v) => v,
            None => continue,
        };
        let host = match host_info.get(&mac_addr[..]) {
            Some(v) => v,
            None => continue,
        };
        let (field_host, field_mac) = match field_name.find(".ip") {
            Some(v) => {
                if v > 0 {
                    (
                        format!("{}.domain", &field_name[..v]),
                        format!("{}.mac", &field_name[..v]),
                    )
                } else {
                    (
                        format!("{}.domain", &field_name[..]),
                        format!("{}.mac", &field_name[..]),
                    )
                }
            }
            None => (
                format!("{}.domain", &field_name[..]),
                format!("{}.mac", &field_name[..]),
            ),
        };
        new_fields.push((LogString::Owned(field_host), SiemField::Text(host.clone())));
        new_fields.push((LogString::Owned(field_mac), SiemField::Text(host.clone())));
    }
    Some(new_fields)
}

fn enrich_asset_with_tags(
    log: &mut SiemLog,
    datasets: &DatasetHolder,
) -> Option<Vec<(LogString, SiemField)>> {
    let assets_info: &TextMapListSynDataset =
        datasets.get(&SiemDatasetType::AssetTag)?.try_into().ok()?;

    let mut new_fields = Vec::with_capacity(32);
    if let Some(source_host) = log.field("source.domain") {
        if let SiemField::Text(hostname) = source_host {
            if let Some(asset_tag) = assets_info.get(&hostname[..]) {
                new_fields.push((
                    LogString::Owned(format!("source.tags")),
                    SiemField::Array(asset_tag.clone()),
                ));
            }
        }
    }
    if let Some(source_host) = log.field("destination.domain") {
        if let SiemField::Text(hostname) = source_host {
            if let Some(asset_tag) = assets_info.get(&hostname[..]) {
                new_fields.push((
                    LogString::Owned(format!("destination.tags")),
                    SiemField::Array(asset_tag.clone()),
                ));
            }
        }
    }
    Some(new_fields)
}

fn enrich_asset_with_vulnerabilities(
    log: &mut SiemLog,
    datasets: &DatasetHolder,
) -> Option<Vec<(LogString, SiemField)>> {
    let assets_info: &TextMapListSynDataset = datasets
        .get(&SiemDatasetType::HostVulnerable)?
        .try_into()
        .ok()?;

    let mut new_fields = Vec::with_capacity(32);

    if let Some(source_host) = log.field("source.domain") {
        if let SiemField::Text(hostname) = source_host {
            if let Some(vulnerabilities) = assets_info.get(&hostname[..]) {
                new_fields.push((
                    LogString::Owned(format!("source.vulnerability.ids")),
                    SiemField::Array(vulnerabilities.clone()),
                ));
            }
        }
    }
    if let Some(source_host) = log.field("destination.domain") {
        if let SiemField::Text(hostname) = source_host {
            if let Some(vulnerabilities) = assets_info.get(&hostname[..]) {
                new_fields.push((
                    LogString::Owned(format!("destination.vulnerability.ids")),
                    SiemField::Array(vulnerabilities.clone()),
                ));
            }
        }
    }
    Some(new_fields)
}

fn enrich_block_domain(
    log: &mut SiemLog,
    datasets: &DatasetHolder,
) -> Option<Vec<(LogString, SiemField)>> {
    let block_domain: &TextSetSynDataset = datasets
        .get(&SiemDatasetType::BlockDomain)?
        .try_into()
        .ok()?;

    let mut new_fields = Vec::with_capacity(32);
    let mut block_list = false;
    if let Some(source_host) = log.field("source.domain") {
        if let SiemField::Text(hostname) = source_host {
            if block_domain.contains(hostname) {
                new_fields.push((
                    LogString::Owned(format!("source.tags")),
                    SiemField::Array(vec![LogString::Borrowed(BLOCKED_IP)]),
                ));
                block_list = true;
            }
        }
    }
    if let Some(source_host) = log.field("destination.domain") {
        if let SiemField::Text(hostname) = source_host {
            if block_domain.contains(hostname) {
                new_fields.push((
                    LogString::Owned(format!("destination.tags")),
                    SiemField::Array(vec![LogString::Borrowed(BLOCKED_IP)]),
                ));
                block_list = true;
            }
        }
    }
    if block_list {
        log.add_tag(BLOCKED_IP);
    }
    Some(new_fields)
}
