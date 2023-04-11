mod controllers;
mod qusb2snes;
mod skins;
use controllers::controller::{Controller, ControllerEvents};
use imageinfo::ImageInfo;
use qusb2snes::usb2snes::SyncClient;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::path::Path;

use skins::skin::Skin;

fn main() -> Result<(), String> {
    /* Setup Controller data */

    let controller_config_path = Path::new("./confs/SuperMetroid.json");
    // let controller_config_path = Path::new("./confs/ALTTP.json");
    let controller = Controller::new(controller_config_path);
    dbg!(&controller);

    /* Setup Skin data */
    // let selected_skin = "default".to_string();
    let selected_skin = "black".to_string();
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-skinny/skin.xml");
    let skin_config_path =
        Path::new("E:/Emu/ButtonMash/Skins/snes-super-famicom-squabbler/skin.xml");
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-sm/skin.xml");
    let skin = Skin::new(skin_config_path);
    dbg!(&skin);

    /* Connect to USB2SNES Server */
    let mut usb2snes = SyncClient::connect();
    println!("Connected to {}", usb2snes.app_version());

    usb2snes.set_name(String::from("Snes Input Viewer"));

    let devices = usb2snes.list_device();

    usb2snes.attach(&devices[0]);
    let info = usb2snes.info();
    println!("Attached to {} - {}", info.dev_type, info.version);

    /*
    Set SDL2 context
    */
    // get background image size
    let background_info = ImageInfo::from_file_path(&skin.backgrounds[&selected_skin]).unwrap();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            background_info.size.width as u32,
            background_info.size.height as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let selected_skin_path = &skin.backgrounds[&selected_skin];
    dbg!(selected_skin_path);

    let controller_texture = texture_creator.load_texture(selected_skin_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["a"].image);
    let button_texture_a = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["b"].image);
    let button_texture_b = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["x"].image);
    let button_texture_x = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["y"].image);
    let button_texture_y = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["select"].image);
    let button_texture_select = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["start"].image);
    let button_texture_start = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["r"].image);
    let button_texture_r = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["l"].image);
    let button_texture_l = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["up"].image);
    let button_texture_up = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["down"].image);
    let button_texture_down = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["left"].image);
    let button_texture_left = texture_creator.load_texture(&button_path)?;

    let button_path = Path::new(&skin.directory).join(&skin.buttons["right"].image);
    let button_texture_right = texture_creator.load_texture(&button_path)?;

    'mainloop: loop {
        let events = controller.pushed(&mut usb2snes);

        canvas.copy(&controller_texture, None, None)?;
        for event in events {
            match event {
                ControllerEvents::A => {
                    canvas.copy(&button_texture_a, None, skin.buttons["a"].rect)?;
                }
                ControllerEvents::X => {
                    canvas.copy(&button_texture_x, None, skin.buttons["x"].rect)?;
                }
                ControllerEvents::B => {
                    canvas.copy(&button_texture_b, None, skin.buttons["b"].rect)?;
                }
                ControllerEvents::Y => {
                    canvas.copy(&button_texture_y, None, skin.buttons["y"].rect)?;
                }
                ControllerEvents::Select => {
                    canvas.copy(&button_texture_select, None, skin.buttons["select"].rect)?;
                }
                ControllerEvents::Start => {
                    canvas.copy(&button_texture_start, None, skin.buttons["start"].rect)?;
                }
                ControllerEvents::Up => {
                    canvas.copy(&button_texture_up, None, skin.buttons["up"].rect)?;
                }
                ControllerEvents::Down => {
                    canvas.copy(&button_texture_down, None, skin.buttons["down"].rect)?;
                }
                ControllerEvents::Left => {
                    canvas.copy(&button_texture_left, None, skin.buttons["left"].rect)?;
                }
                ControllerEvents::Right => {
                    canvas.copy(&button_texture_right, None, skin.buttons["right"].rect)?;
                }
                ControllerEvents::L => {
                    canvas.copy(&button_texture_l, None, skin.buttons["l"].rect)?;
                }
                ControllerEvents::R => {
                    canvas.copy(&button_texture_r, None, skin.buttons["r"].rect)?;
                }
            }
        }
        canvas.present();

        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }
    Ok(())
}
