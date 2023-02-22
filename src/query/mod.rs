use tokio::process::Command;
use types::BssidPayload;

pub mod types;


// naive get hostname, in a more complex implementation, would do something more 
// platform independent as such: https://github.com/swsnr/gethostname.rs/blob/main/src/lib.rs
pub async fn get_hostname() -> anyhow::Result<String>{
    let output = Command::new("hostname").output().await?;

    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}

// native get_bssid as well, also would look at platform-independent solutions(using netsh on windows for instance )
pub async fn get_bssid() -> anyhow::Result<Vec<BssidPayload>>{

    //example output: 
    // IN-USE  BSSID              SSID                             MODE   CHAN  RATE        SIGNAL  BARS  SECURITY  
    //         18:7B:CB:42:EF:8E  FRITZ!Box 7530 MS                Infra  11    260 Mbit/s  100     ▂▄▆█  WPA2      
    // *       D1:12:C8:87:D7:F1  FRITZ!Box 7530 MS                Infra  6     130 Mbit/s  75      ▂▄▆_  WPA2      
    //         DD:72:2D:64:04:C9  example network                  Infra  11    130 Mbit/s  65      ▂▄▆_  WPA1 WPA2 
    //         2C:A1:F2:9E:97:2E  FRITZ!Box 7530 MS                Infra  11    130 Mbit/s  57      ▂▄▆_  WPA2      
    //         11:22:F5:89:71:9C  some_other_network               Infra  6     130 Mbit/s  55      ▂▄__  WPA2   

    let output = Command::new("nmcli device wifi")
                     .output().await?;

    let result: Vec<BssidPayload> = std::str::from_utf8(&output.stdout)?.to_string()
                    // initial split into lines
                     .split("\n")
                     // enumerate so that we can get the index
                     .enumerate()
                     // filter the first entry(header) out
                     .filter(|(i, _)| i != &0_usize)
                     // map each line to a bssid entry
                     .map(|(_, line)| {
                         let entries: Vec<&str> = line.split_whitespace().collect();
             
                         let bssid = entries[0].to_string();
                         let ssid = entries[1].to_string();
                         let channel = entries[3].to_string();
                         let rssi = entries[5].to_string();
                         let security = entries[6].to_string();
             
                         BssidPayload::new(
                             ssid,
                             bssid,
                             rssi,
                             channel,
                             security,
                         )
                     })
                     .collect();

    Ok(result)
}