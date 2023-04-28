#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controllers;
mod skins;
use controllers::controller::Controller;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use rusb2snes::SyncClient;
use skins::skin::Skin;

use configuration::config::AppConfig;

fn main() -> Result<(), String> {
    /* Setup Configs */
    let app_config = AppConfig::new().unwrap();
    
    let controller = Controller::new(&app_config.controller.input_config_path);

    let skin = Skin::new(&app_config.skin.skins_path, app_config.skin.skin_name);

    /* Connect to USB2SNES Server */
    let mut usb2snes = SyncClient::connect().unwrap();


    usb2snes.set_name(String::from("S
    $nes Input Viewer")).unwrap();

    let devices = usb2snes.list_device().unwrap();

    usb2snes.attach(&devices[0]).unwrap();
    let info = usb2snes.info().unwrap();
    println!("Attached to {} - {}", info.dev_type, info.version);

    /*
    Set SDL2 context
    */

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(
            "SNES Input Display",
            skin.backgrounds[&app_config.skin.skin_theme].width,
            skin.backgrounds[&app_config.skin.skin_theme].height,
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
    // skin.load_textures(&canvas);
    let background_texture =
        texture_creator.load_texture(&skin.backgrounds[&app_config.skin.skin_theme].image)?;

    let mut button_textures = HashMap::new();

    button_textures.insert("a", texture_creator.load_texture(&skin.buttons["a"].image)?);
    button_textures.insert("b", texture_creator.load_texture(&skin.buttons["b"].image)?);
    button_textures.insert("x", texture_creator.load_texture(&skin.buttons["x"].image)?);
    button_textures.insert("y", texture_creator.load_texture(&skin.buttons["y"].image)?);
    button_textures.insert(
        "select",
        texture_creator.load_texture(&skin.buttons["select"].image)?,
    );
    button_textures.insert(
        "start",
        texture_creator.load_texture(&skin.buttons["start"].image)?,
    );
    button_textures.insert("r", texture_creator.load_texture(&skin.buttons["r"].image)?);
    button_textures.insert("l", texture_creator.load_texture(&skin.buttons["l"].image)?);
    button_textures.insert(
        "up",
        texture_creator.load_texture(&skin.buttons["up"].image)?,
    );
    button_textures.insert(
        "down",
        texture_creator.load_texture(&skin.buttons["down"].image)?,
    );
    button_textures.insert(
        "left",
        texture_creator.load_texture(&skin.buttons["left"].image)?,
    );
    button_textures.insert(
        "right",
        texture_creator.load_texture(&skin.buttons["right"].image)?,
    );

    'mainloop: loop {
        let events = controller.pushed(&mut usb2snes).unwrap();

        canvas.copy(&background_texture, None, None)?;
        for event in events {
            canvas.copy(&button_textures[event], None, skin.buttons[event].rect)?;
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
