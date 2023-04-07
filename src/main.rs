mod controller;
mod qusb2snes;
mod skin;
use controller::controller::{Controller, ControllerEvents};
use qusb2snes::usb2snes::SyncClient;
use std::path::Path;

use crate::skin::skin::Skin;

fn main() {
    // let controller_config_path = Path::new("./confs/SuperMetroid.json");
    let controller_config_path = Path::new("./confs/ALTTP.json");
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-skinny/skin.xml");
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-super-famicom-squabbler/skin.xml");
    let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-sm/skin.xml");
    let controller = Controller::new(&controller_config_path);
    let skin = Skin::new(skin_config_path);

    let mut usb2snes = SyncClient::connect();
    println!("Connected to {}", usb2snes.app_version());

    usb2snes.set_name(String::from("Snes Input Viewer"));

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);
    let info = usb2snes.info();
    println!("Attached to {} - {}", info.dev_type, info.version);
    dbg!(&controller);
    dbg!(&skin);
    loop {
        let events = controller.pushed(&mut usb2snes);
        let mut input_string = String::new();
        for event in events {
            match event {
                ControllerEvents::A => {
                    input_string.push_str(&skin.buttons["a"].image);
                }
                ControllerEvents::X => {
                    input_string.push_str(&skin.buttons["x"].image);
                }
                ControllerEvents::B => {
                    input_string.push_str(&skin.buttons["b"].image);
                }
                ControllerEvents::Y => {
                    input_string.push_str(&skin.buttons["y"].image);
                }
                ControllerEvents::Select => {
                    input_string.push_str(&skin.buttons["select"].image);
                }
                ControllerEvents::Start => {
                    input_string.push_str(&skin.buttons["start"].image);
                }
                ControllerEvents::Up => {
                    input_string.push_str(&skin.buttons["up"].image);
                }
                ControllerEvents::Down => {
                    input_string.push_str(&skin.buttons["down"].image);
                }
                ControllerEvents::Left => {
                    input_string.push_str(&skin.buttons["left"].image);
                }
                ControllerEvents::Right => {
                    input_string.push_str(&skin.buttons["right"].image);
                }
                ControllerEvents::L => {
                    input_string.push_str(&skin.buttons["l"].image);
                }
                ControllerEvents::R => {
                    input_string.push_str(&skin.buttons["r"].image);
                }
            }
        }
        println!("{}", input_string)
    }
}
