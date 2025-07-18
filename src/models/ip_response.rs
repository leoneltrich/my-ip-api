use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct IpResponse {
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}