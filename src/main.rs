mod controllers;
mod qusb2snes;
mod skins;
use controllers::controller::{Controller, Inputs};
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

    /* Setup Skin data */
    // let selected_skin = "default".to_string();
    let skins_path = Path::new("/home/thibault/Documents/perso/squabbler-retrospy-nintendospy-skins/skins");
    let selected_skin = "snes-super-famicom-squabbler".to_string();
    let selected_skin_theme = "black".to_string();
    
    // let skin_config_path = Path::new("E:/Emu/ButtonMash/Skins/snes-sm/skin.xml");
    let skin = Skin::new(skins_path, selected_skin);

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
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(
            "rust-sdl2 demo: Video",
            skin.backgrounds[&selected_skin_theme].width,
            skin.backgrounds[&selected_skin_theme].height,
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

    let background_texture = texture_creator.load_texture(&skin.backgrounds[&selected_skin_theme].image)?;
    let button_texture_a = texture_creator.load_texture(&skin.buttons["a"].image)?;
    let button_texture_b = texture_creator.load_texture(&skin.buttons["b"].image)?;
    let button_texture_x = texture_creator.load_texture(&skin.buttons["x"].image)?;
    let button_texture_y = texture_creator.load_texture(&skin.buttons["y"].image)?;
    let button_texture_select = texture_creator.load_texture(&skin.buttons["select"].image)?;
    let button_texture_start = texture_creator.load_texture(&skin.buttons["start"].image)?;
    let button_texture_r = texture_creator.load_texture(&skin.buttons["r"].image)?;
    let button_texture_l = texture_creator.load_texture(&skin.buttons["l"].image)?;
    let button_texture_up = texture_creator.load_texture(&skin.buttons["up"].image)?;
    let button_texture_down = texture_creator.load_texture(&skin.buttons["down"].image)?;
    let button_texture_left = texture_creator.load_texture(&skin.buttons["left"].image)?;
    let button_texture_right = texture_creator.load_texture(&skin.buttons["right"].image)?;

    'mainloop: loop {
        let events = controller.pushed(&mut usb2snes);

        canvas.copy(&background_texture, None, None)?;
        for event in events {
            match event {
                Inputs::A => {
                    canvas.copy(&button_texture_a, None, skin.buttons["a"].rect)?
                }
                Inputs::X => {
                    canvas.copy(&button_texture_x, None, skin.buttons["x"].rect)?;
                }
                Inputs::B => {
                    canvas.copy(&button_texture_b, None, skin.buttons["b"].rect)?;
                }
                Inputs::Y => {
                    canvas.copy(&button_texture_y, None, skin.buttons["y"].rect)?;
                }
                Inputs::Select => {
                    canvas.copy(&button_texture_select, None, skin.buttons["select"].rect)?;
                }
                Inputs::Start => {
                    canvas.copy(&button_texture_start, None, skin.buttons["start"].rect)?;
                }
                Inputs::Up => {
                    canvas.copy(&button_texture_up, None, skin.buttons["up"].rect)?;
                }
                Inputs::Down => {
                    canvas.copy(&button_texture_down, None, skin.buttons["down"].rect)?;
                }
                Inputs::Left => {
                    canvas.copy(&button_texture_left, None, skin.buttons["left"].rect)?;
                }
                Inputs::Right => {
                    canvas.copy(&button_texture_right, None, skin.buttons["right"].rect)?;
                }
                Inputs::L => {
                    canvas.copy(&button_texture_l, None, skin.buttons["l"].rect)?;
                }
                Inputs::R => {
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
