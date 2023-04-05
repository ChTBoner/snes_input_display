mod qusb2snes;
mod controller;
use controller::controller::Controller;
use qusb2snes::usb2snes::SyncClient;




fn main() {
    let controller = Controller::new("./confs/SuperMetroid.json".to_string());
    // let wram_offset = 0xF50000;
    // let input_address = wram_offset + controller.address;
    // let input_address = 0xF5008B;
    // let address_size = 2;

    
    let mut usb2snes = SyncClient::connect();
    println!("Connected to {}", usb2snes.app_version());

    usb2snes.set_name(String::from("Snes Input Viewer"));

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);
    let info = usb2snes.info();
    println!("Attached to {} - {}", info.dev_type, info.version);

    loop {
        controller.pushed(&mut usb2snes);

    }

}