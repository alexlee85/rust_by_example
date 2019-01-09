extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;

use futures::Stream;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde_json::Value;
use tokio_core::reactor::Core;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeviceStatus {
    value: String,
    last_update_time: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Device {
    address: String,
    device_type: String,
    resource_sum: String,
    mac_address: String,
    status: Vec<DeviceStatus>,
}

fn main() {
    let json_str = r#"{
	    "address": "16113",
	    "deviceType": "PowerPanel",
	    "resourceSum": "2",
	    "macAddress": "00:12:4b:00:09:2d:89:9b",
	    "status": [
	        {
	            "value": "0",
	            "lastUpdateTime": 1470127427006
	        },
	        {
	            "value": "0",
	            "lastUpdateTime": 1470127427006
	        }
	    ]
	}"#;

    let data: Device = serde_json::from_str(json_str).unwrap();
    println!("{:?}", data);
    let time = std::time::Instant::now();
    for _ in 0..5000 {
        let data: Device = serde_json::from_str(json_str).unwrap();
        let _ = serde_json::to_string(&data).unwrap();
    }

    println!("{:?}", time.elapsed());

    let mut core = Core::new().unwrap();

    let connector = HttpsConnector::new(4).unwrap();

    let client = Client::builder().build::<_, hyper::Body>(connector);

    let uri = "https://crates.io/api/v1/crates/serde_derive"
        .parse()
        .unwrap();

    let response = core.run(client.get(uri)).unwrap();
    let body = core.run(response.into_body().concat2()).unwrap();
    let json_str = format!("{}", String::from_utf8_lossy(&body));

    let time = std::time::Instant::now();
    let data: Value = serde_json::from_str(json_str.as_str()).unwrap();
    println!("{:?}", data);
    println!("{:?}", time.elapsed());
}
