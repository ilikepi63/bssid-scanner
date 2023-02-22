use std::{env, time::Duration};
use tokio::time;
use query::{get_bssid, get_hostname};
use request::{make_request};

pub mod query;
pub mod request;

// include tokio as the standard runtime, as it's probably the most preformant for our usecase(periodically retrieving data posting it.)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // get the url from the environment variables(can be compiled at build time)
    // we want to force the binary to panic here, so we know the
    let url: &'static str = env!("URL");

    // want it to panic here with a human readable error at startup.
    let hostname = get_hostname().await.expect("Could not retrieve hostname for the current machine");

    // create the reqwest client. 
    let client = reqwest::Client::new();

    // setup an interval each 10 seconds
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
    
        interval.tick().await;

        // get the bssid information from the environment
        // if it fails, we would just continue on to getting the next lot of information
        println!("IS getting here");
        if let Ok(information) = get_bssid().await {

            println!("Does get here");
            make_request(&hostname, url, information, &client).await;

        }
    }

}
