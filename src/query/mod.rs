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
pub async fn get_bssid() -> anyhow::Result<BssidPayload>{
    let output = Command::new("ls")
                     .arg("-l")
                     .output().await?;

    // let lines =  std::str::from_utf8(&output.stdout)?.split("\n");
                 
    // for line in lines {
    //     println!("{:?}",line);
    // }

    Ok(BssidPayload::new(String::from(String::from("123")) , String::from("123"), String::from("123"), String::from("123"), String::from("123")))
                 
}