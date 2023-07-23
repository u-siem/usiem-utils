use usiem::{
    serde::{Deserialize, Serialize},
    utilities::types::LogString,
};

use crate::err::TempResult;

pub async fn get_office365_ip() -> TempResult<Vec<Office365ServiceInfo>> {
    let body = reqwest::get("https://endpoints.office.com/endpoints/worldwide?clientrequestid=b10c5ed1-bad1-445f-b386-b919946339a7")
    .await?
    .text()
    .await?;
    let res:  Vec<Office365ServiceInfo> = usiem::serde_json::from_str(&body)?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Office365ServiceInfo {
    pub id: u32,
    #[serde(rename = "serviceArea")]
    pub service_area: String,
    #[serde(rename = "serviceAreaDisplayName")]
    pub service_area_display_name: String,
    #[serde(default)]
    pub urls: Vec<String>,
    #[serde(default)]
    pub ips: Vec<String>,
    #[serde(rename = "tcpPorts")]
    pub tcp_ports: Option<String>,
    #[serde(rename = "udpPorts")]
    pub udp_ports: Option<String>,
    #[serde(rename = "expressRoute")]
    pub express_route: bool,
    pub category: String,
    pub required: bool,
}

pub fn static_service(service: &str) -> LogString {
    match service {
        "Exchange" => LogString::Borrowed("Exchange"),
        "Skype" => LogString::Borrowed("Skype"),
        "SharePoint" => LogString::Borrowed("SharePoint"),
        "Common" => LogString::Borrowed("O365 Common"),
        _ => LogString::Owned(service.to_string()),
    }
}

#[tokio::test]
async fn test_o365() {
    let _res = get_office365_ip().await;
}
