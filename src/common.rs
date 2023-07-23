use usiem::utilities::ip_utils::{ipv4_from_str, ipv6_from_str};

pub(crate) fn parse_ip4_network(ip_net: &str) -> Option<(u32, u8)> {
    let pos = ip_net.find('/')?;
    let ip = &ip_net[..pos];
    let net = ip_net[pos + 1..].parse::<u8>().ok()?;
    let ip = ipv4_from_str(ip).ok()?;
    Some((ip, net))
}
pub(crate) fn parse_ip6_network(ip_net: &str) -> Option<(u128, u8)> {
    let pos = ip_net.find('/')?;
    let ip = &ip_net[..pos];
    let net = ip_net[pos + 1..].parse::<u8>().ok()?;
    let ip = ipv6_from_str(ip).ok()?;
    Some((ip, net))
}
