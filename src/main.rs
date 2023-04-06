mod qusb2snes;
mod controller;
use controller::controller::{Controller, ControllerEvents};
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
        let events = controller.pushed(&mut usb2snes);
        let mut input_string = String::new();
        for event in events {
            match event {
                ControllerEvents::A => { 
                    input_string.push_str("A ")
                },
                ControllerEvents::X => {
                    input_string.push_str("X ");
                },
                ControllerEvents::B => {
                    input_string.push_str("B ");
                },
                ControllerEvents::Y => {
                    input_string.push_str("Y ");
                },
                ControllerEvents::Select => {
                    input_string.push_str("Select ");
                },
                ControllerEvents::Start => {
                    input_string.push_str("Start ");
                },
                ControllerEvents::Up =>{
                    input_string.push_str("Up ");
                },
                ControllerEvents::Down => {
                    input_string.push_str("Down ");
                },
                ControllerEvents::Left => {
                    input_string.push_str("Left ");
                },
                ControllerEvents::Right => {
                    input_string.push_str("Right ");
                },
                ControllerEvents::L => {
                    input_string.push_str("L ");
                },
                ControllerEvents::R => {
                    input_string.push_str("R ");
                },
            }
            };
            println!("{}", input_string)
    }
}
