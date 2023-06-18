use pcap::{Device, Packet, Capture};
use serde_json;
use reqwest;
use chrono;
#[tokio::main]
async fn main() {
    // set device to monitor mode (if not already)

    let device_name = "wlan0";
    let output = std::process::Command::new("iw")
        .arg("dev")
        .arg(device_name)
        .arg("set")
        .arg("monitor")
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        println!("Device {} is now in monitor mode.", device_name);
    } else {
        println!("Failed to set monitor mode on device {}", device_name);
    }
    //get device
    //open device
    let  mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    

    // set filter for wifi scan packets
    cap.filter("type mgt subtype probe-req", true).unwrap();
    let mut packets: Vec<Packet> = Vec::new();
    // capture packets while time is less than 7pm
    // BUG: cannot borrow cap as mutable more than once at a time
    // how to fix?
    // also check if built with debug if debug then stop after 10 packets
    while chrono::Local::now().time() < chrono::NaiveTime::from_hms(15, 55, 0) {
        let packet = cap.next_packet().unwrap();
        println!("packet: {:?}", packet);
    }

    // convert pacekts to mac addresses
    // convert mac addresses to json
    let  mut macs: Vec<String> = Vec::new();


    let json = serde_json::to_string(&macs).unwrap();
    //send json to server
    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:3000")
        .body(json)
        .send()
        .await;
}
