use chrono::{TimeZone, Utc};
use reqwest::{blocking, get};
use rsntp::SntpClient;

use chrono::prelude::*;



//source: https://docs.rs/chrono/latest/chrono/offset/struct.Utc.html
fn main(){

    let mut server_address = "3.be.pool.ntp.org".to_string();
    let fetch_default_ntp= "https://raw.githubusercontent.com/EloiStree/IP/main/IIDWS/DEFAULT_NTP.txt";
    

    // Make the HTTP GET request
    let response = blocking::get(fetch_default_ntp).unwrap();

    // Ensure the request was successful
    if response.status().is_success() {
        // Get the text content of the response
        let web_content = response.text();
        let unwrapped_content = web_content.unwrap().clone();
        server_address = unwrapped_content.trim().to_string();
    } 

    print!("Server address: {:?}", server_address)
;

    let client = SntpClient::new();
    let result = client.synchronize(server_address).unwrap();
    
    let utc_time = result
      .datetime().unix_timestamp().unwrap();
    let utc_time_tick:u128 = (utc_time) .as_nanos();
    
    println!("UTC time is: {:?}", utc_time);
    let now = Utc::now();   

    // Convert to nanoseconds since the Unix epoch
    let unix_epoch: chrono::offset::LocalResult<DateTime<Utc>> = Utc.with_ymd_and_hms(1970, 1, 1,0, 0, 0);
    let duration_since_epoch = now.signed_duration_since(unix_epoch.unwrap());
    let ticks = duration_since_epoch.num_nanoseconds().unwrap() as u128;

    // Print the timestamp in ticks (nanoseconds)
    let unix_timestamp = now.timestamp();
    // Print the timestamp
    println!("Current timestamp: {}", unix_timestamp);
    println!("Current timestamp in ticks: {}", ticks);
    println!("NTP:{:?}", utc_time_tick);
    println!("TIk:{:?}", ticks);
    let diff_nano_seconds = utc_time_tick - ticks;
    println!("Diff:{:?}", diff_nano_seconds);
    let diff_seconds = diff_nano_seconds / 1_000_000_000;
    println!("Diff seconds:{:?}", diff_seconds);
    let diff_milliseconds= diff_nano_seconds / 1_000_000;
    println!("Diff milliseconds:{:?}", diff_milliseconds);


}