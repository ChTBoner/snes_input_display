mod controller;
mod qusb2snes;
mod skin;
use controller::controller::{Controller, ControllerEvents};
use qusb2snes::usb2snes::SyncClient;
use std::fmt::format;
use std::path::Path;
use imageinfo::{ImageSize, ImageInfo};


use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;

use crate::skin::skin::Skin;

fn main() -> Result<(), String> {
    /* Setup Controller data */

    // let controller_config_path = Path::new("./confs/SuperMetroid.json");
    let controller_config_path = Path::new("./confs/ALTTP.json");
    let controller = Controller::new(&controller_config_path);
    // dbg!(&controller);
    
    /* Setup Skin data */
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-skinny/skin.xml");
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-super-famicom-squabbler/skin.xml");
    let selected_skin = "default".to_string();
    let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-sm/skin.xml");
    let skin = Skin::new(skin_config_path);
    let selected_skin_path = Path::new(&skin.directory).join(&skin.backgrounds[&selected_skin]);
    dbg!(selected_skin_path.as_path());

    /* Connect to USB2SNES Server */
/*     let mut usb2snes = SyncClient::connect();
    println!("Connected to {}", usb2snes.app_version());

    usb2snes.set_name(String::from("Snes Input Viewer"));

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);
    let info = usb2snes.info();
    println!("Attached to {} - {}", info.dev_type, info.version); */

    /* 
        Set SDL2 context 
    */
    // get background image size
    let background_info = ImageInfo::from_file_path(selected_skin_path.as_path()).unwrap();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust-sdl2 demo: Video", background_info.size.width as u32, background_info.size.height as u32)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    // canvas.present();
    let _texture = texture_creator.load_texture(&selected_skin_path)?;
    'mainloop: loop {
        canvas.clear();
    
        // canvas.copy(&texture, None, None)?;
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Space), ..
                } => {
                    let button_path = Path::new(&skin.directory).join(&skin.buttons["a"].image);
                    let button_texture = texture_creator.load_texture(&button_path)?;
                    println!("SPACE");
                    canvas.copy(&button_texture, None, None)?;

                },
                _ => {}
            }
        }
        canvas.present();
    }
    /* 'mainloop: loop {
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
    }*/
    Ok(())
}
