//! Module for determining the public IP address of the current machine.

#[derive(Debug, Serialize)]
pub struct PublicIpPair {
    pub ipv4: String,
    pub ipv6: String,
}

impl PublicIpPair {
    /// Construct a new `PublicIpPair`
    pub async fn new(ipv4_override: Option<String>, ipv6_override: Option<String>) -> Self {
        Self {
            ipv4: ipv4_override.unwrap_or(
                public_ip::addr_v4()
                    .await
                    .map(|a| a.to_string())
                    .unwrap_or("N/A".to_string()),
            ),
            ipv6: ipv6_override.unwrap_or(
                public_ip::addr_v6()
                    .await
                    .map(|a| a.to_string())
                    .unwrap_or("N/A".to_string()),
            ),
        }
    }
}
