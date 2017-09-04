extern crate rustc_serialize;

use rustc_serialize::json;

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

    for _ in 0..100000 {
	    let data = json::Json::from_str(json_str).unwrap();
	    let _ = json::encode(&data).unwrap();
    }
	
}