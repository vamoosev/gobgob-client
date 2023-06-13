use pcap::{Device,Capture,Packet};
use serde_json;
use reqwest;
use chrono;
#[tokio::main]
async fn main() {
    //get device
    //open device
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    //create vector to store packets
    let mut packets: Vec<Packet> = Vec::new();
    // capture packets while time is less than 7pm
    while chrono::Local::now().time() < chrono::NaiveTime::from_hms(19, 0, 0) {
        //capture packet
        if let Ok(packet) = cap.next_packet() {
            // add packet to vector
            packets.push(packet);
        }
    }

    // convert pacekts to mac addresses
    // convert mac addresses to json
    let  mut macs: Vec<String> = Vec::new();
    for packet in packets.to_owned() {
        let mac = format!("{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", packet.data[6], packet.data[7], packet.data[8], packet.data[9], packet.data[10], packet.data[11]);
        macs.push(mac);
    }




    let json = serde_json::to_string(&macs).unwrap();
    //send json to server
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3000")
        .body(json)
        .send()
        .await;
}
