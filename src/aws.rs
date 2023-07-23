use serde::{Deserialize, Serialize};
use usiem::utilities::types::LogString;

use crate::err::TempResult;

pub async fn get_aws_ips() -> TempResult<AwsIpRanges> {
    let body = reqwest::get("https://ip-ranges.amazonaws.com/ip-ranges.json")
    .await?
    .text()
    .await?;
    let res: AwsIpRanges = usiem::serde_json::from_str(&body)?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AwsIpRanges {
    #[serde(rename = "syncToken")]
    pub sync_token: String,
    #[serde(rename = "createDate")]
    pub create_date: String,
    pub prefixes: Vec<AwsService>,
    pub ipv6_prefixes: Vec<Aws6Service>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AwsService {
    pub ip_prefix: String,
    pub region: String,
    pub service: String,
    pub network_border_group: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Aws6Service {
    pub ipv6_prefix: String,
    pub region: String,
    pub service: String,
    pub network_border_group: String,
}

pub fn static_service(service: String) -> LogString {
    match &service[..] {
        "AMAZON" => LogString::Borrowed("AMAZON"),
        "AMAZON_APPFLOW" => LogString::Borrowed("AMAZON_APPFLOW"),
        "AMAZON_CONNECT" => LogString::Borrowed("AMAZON_CONNECT"),
        "API_GATEWAY" => LogString::Borrowed("API_GATEWAY"),
        "CHIME_MEETINGS" => LogString::Borrowed("CHIME_MEETINGS"),
        "CHIME_VOICECONNECTOR" => LogString::Borrowed("CHIME_VOICECONNECTOR"),
        "CLOUD9" => LogString::Borrowed("CLOUD9"),
        "CLOUDFRONT" => LogString::Borrowed("CLOUDFRONT"),
        "CLOUDFRONT_ORIGIN_FACING" => LogString::Borrowed("CLOUDFRONT_ORIGIN_FACING"),
        "CODEBUILD" => LogString::Borrowed("CODEBUILD"),
        "DYNAMODB" => LogString::Borrowed("DYNAMODB"),
        "EBS" => LogString::Borrowed("EBS"),
        "EC2" => LogString::Borrowed("EC2"),
        "EC2_INSTANCE_CONNECT" => LogString::Borrowed("EC2_INSTANCE_CONNECT"),
        "GLOBALACCELERATOR" => LogString::Borrowed("GLOBALACCELERATOR"),
        "KINESIS_VIDEO_STREAMS" => LogString::Borrowed("KINESIS_VIDEO_STREAMS"),
        "ROUTE53" => LogString::Borrowed("ROUTE53"),
        "ROUTE53_HEALTHCHECKS" => LogString::Borrowed("ROUTE53_HEALTHCHECKS"),
        "ROUTE53_HEALTHCHECKS_PUBLISHING" => LogString::Borrowed("ROUTE53_HEALTHCHECKS_PUBLISHING"),
        "ROUTE53_RESOLVER" => LogString::Borrowed("ROUTE53_RESOLVER"),
        "S3" => LogString::Borrowed("S3"),
        "WORKSPACES_GATEWAYS" => LogString::Borrowed("WORKSPACES_GATEWAYS"),
        _ => LogString::Owned(service),
    }
}

pub fn static_region(region: String) -> LogString {
    match &region[..] {
        "ap-east-1" => LogString::Borrowed("AWS-ap-east-1"),
        "ap-northeast-1" => LogString::Borrowed("AWS-ap-northeast-1"),
        "ap-northeast-2" => LogString::Borrowed("AWS-ap-northeast-2"),
        "ap-northeast-3" => LogString::Borrowed("AWS-ap-northeast-3"),
        "ap-south-1" => LogString::Borrowed("AWS-ap-south-1"),
        "ap-southeast-1" => LogString::Borrowed("AWS-ap-southeast-1"),
        "ap-southeast-2" => LogString::Borrowed("AWS-ap-southeast-2"),
        "ca-central-1" => LogString::Borrowed("AWS-ca-central-1"),
        "cn-north-1" => LogString::Borrowed("AWS-cn-north-1"),
        "cn-northwest-1" => LogString::Borrowed("AWS-cn-northwest-1"),
        "eu-central-1" => LogString::Borrowed("AWS-eu-central-1"),
        "eu-central-2" => LogString::Borrowed("AWS-eu-central-2"),
        "eu-north-1" => LogString::Borrowed("AWS-eu-north-1"),
        "eu-south-1" => LogString::Borrowed("AWS-eu-south-1"),
        "eu-south-2" => LogString::Borrowed("AWS-eu-south-2"),
        "eu-west-1" => LogString::Borrowed("AWS-eu-west-1"),
        "eu-west-2" => LogString::Borrowed("AWS-eu-west-2"),
        "eu-west-3" => LogString::Borrowed("AWS-eu-west-3"),
        "me-central-1" => LogString::Borrowed("AWS-me-central-1"),
        "me-south-1" => LogString::Borrowed("AWS-me-south-1"),
        "sa-east-1" => LogString::Borrowed("AWS-sa-east-1"),
        "us-east-1" => LogString::Borrowed("AWS-us-east-1"),
        "us-east-2" => LogString::Borrowed("AWS-us-east-2"),
        "us-gov-east-1" => LogString::Borrowed("AWS-us-gov-east-1"),
        "us-gov-west-1" => LogString::Borrowed("AWS-us-gov-west-1"),
        "us-west-1" => LogString::Borrowed("AWS-us-west-1"),
        "us-west-2" => LogString::Borrowed("AWS-us-west-2"),
        "GLOBAL" => LogString::Borrowed("AWS-GLOBAL"),
        _ => LogString::Owned(region),
    }
}

#[tokio::test]
async fn test_aws() {
    let _res = get_aws_ips().await;
}
