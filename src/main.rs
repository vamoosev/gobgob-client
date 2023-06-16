use pcap::{Device,Packet};
use serde_json;
use reqwest;
use chrono;
#[tokio::main]
async fn main() {
    //get device
    //open device
    let  mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    //crete vector to store packets
    let mut packets: Vec<Packet> = Vec::new();
    // capture packets while time is less than 7pm
    // BUG: cannot borrow cap as mutable more than once at a time
    // how to fix?
    while let Ok(packet) = cap.next_packet() {
        let time = chrono::Local::now();
         if chrono::NaiveTime::from_hms(19, 0, 0) > time.time(){
            packets.push(packet.clone());
        } else {
            break;
        }
    }


    // convert pacekts to mac addresses
    // convert mac addresses to json
    let  mut macs: Vec<String> = Vec::new();
    for packet in packets {
        let mac = format!("{:x}:{:x}:{:x}:{:x}:{:x}:{:x}", packet.data[6], packet.data[7], packet.data[8], packet.data[9], packet.data[10], packet.data[11]);
        macs.push(mac);
    }




    let json = serde_json::to_string(&macs).unwrap();
    //send json to server
    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:3000")
        .body(json)
        .send()
        .await;
}
