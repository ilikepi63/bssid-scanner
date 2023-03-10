use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BssidPayload {
    ssid: String,
    bssid: String,
    rssi: String,
    channel: String,
    security: String,
    #[serde(rename = "localTime")]
    local_time: String,
}

impl BssidPayload {
    pub fn new(
        ssid: String,
        bssid: String,
        rssi: String,
        channel: String,
        security: String,
    ) -> Self {
        BssidPayload {
            ssid,
            bssid,
            rssi,
            channel,
            security,
            local_time: Utc::now().to_string(),
        }
    }
}

// Another option for converting the line items in the output to Payloads
// impl TryFrom<String> for BssidPayload {
//     type Error = anyhow::Error;

//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         Ok(BssidPayload::new(String::from(String::from("123")) , String::from("123"), String::from("123"), String::from("123"), String::from("123")))
//     }
// }
