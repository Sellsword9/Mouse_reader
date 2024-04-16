extern crate hidapi;

use hidapi::HidApi;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let api = HidApi::new().unwrap();
    let vendor_id = 0x062a; // use lsusb to get the vendor_id and product_id
    let product_id = 0x4102; // use lsusb to get the vendor_id and product_id
    let device = api.open(vendor_id, product_id).unwrap();
    let report = [0x00, 0x01];
    let mut response: [u8; 8];
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("info.txt")
        .unwrap();
    loop {
        device.write(&report).unwrap();
        response = [0u8; 8];
        device.read_timeout(&mut response, 10000).unwrap();
        println!("Data: {:?}", response);
        if let Err(e) = writeln!(file, "Datos leídos: {:?}", response) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
