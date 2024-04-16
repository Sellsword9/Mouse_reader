extern crate hidapi;

use hidapi::HidApi;

fn main() {
    let api = HidApi::new().unwrap();
    let vendor_id = 0x062a;
    let product_id = 0x4102;
    let device = api.open(vendor_id, product_id).unwrap();
    let report = [0x00, 0xff];
    let mut response: [u8; 8];
    loop {
        device.write(&report).unwrap();
        response = [0u8; 8];
        device.read_timeout(&mut response, 10000).unwrap(); // Aumentado a 3000 milisegundos
        println!("Datos leídos: {:?}", response);
    }
}
