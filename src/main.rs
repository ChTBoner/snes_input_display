#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controllers;
mod skins;
use controllers::controller::Controller;

use rusb2snes::SyncClient;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;
use sdl2::rect::Rect;
use skins::skin::Skin;
use std::collections::HashMap;
use std::error::Error;

use configuration::config::AppConfig;

fn main() -> Result<(), Box<dyn Error>> {
    /* Setup Configs */
    let app_config = AppConfig::new()?;

    let controller = Controller::new(&app_config.controller.input_config_path);

    let skin = Skin::new(&app_config.skin.skins_path, app_config.skin.skin_name);

    /* Connect to USB2SNES Server */
    let mut usb2snes = SyncClient::connect()?;

    usb2snes.set_name(String::from("Snes Input Viewer"))?;

    let devices = usb2snes.list_device()?;

    usb2snes.attach(&devices[0])?;
    let info = usb2snes.info()?;
    println!("Attached to {} - {}", info.dev_type, info.version);

    /*
    Set SDL2 context
    */

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let mut fps = FPSManager::new();
    fps.set_framerate(60)?;
    dbg!(&fps.get_framerate());
    let window_height = match app_config.skin.piano_roll {
        true => skin.backgrounds[&app_config.skin.skin_theme].height * 2,
        false => skin.backgrounds[&app_config.skin.skin_theme].height,
    };

    let window = video_subsystem
        .window(
            "SNES Input Display",
            skin.backgrounds[&app_config.skin.skin_theme].width,
            window_height,
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

    // calculate x for each button position if piano roll.
    let piano_roll_section_width = skin.backgrounds[&app_config.skin.skin_theme].width / 12;
    let piano_roll_extra_width = skin.backgrounds[&app_config.skin.skin_theme].width % 12;
    let piano_roll_inside_padding = 5;
    let piano_roll_rect_width = piano_roll_section_width - (piano_roll_inside_padding * 2);
    let piano_roll_left_padding = piano_roll_extra_width / 2;
    // let piano_roll_right_padding = piano_roll_extra_width % 2;
    // let piano_roll_color:
    let mut piano_roll_x_positions = HashMap::new();
    piano_roll_x_positions.insert("left", piano_roll_left_padding + 5);
    piano_roll_x_positions.insert(
        "up",
        piano_roll_x_positions.get("left").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "down",
        piano_roll_x_positions.get("up").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "right",
        piano_roll_x_positions.get("down").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "l",
        piano_roll_x_positions.get("right").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "select",
        piano_roll_x_positions.get("l").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "start",
        piano_roll_x_positions.get("select").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "r",
        piano_roll_x_positions.get("start").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "y",
        piano_roll_x_positions.get("r").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "b",
        piano_roll_x_positions.get("y").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "x",
        piano_roll_x_positions.get("b").unwrap().to_owned() + piano_roll_section_width,
    );
    piano_roll_x_positions.insert(
        "a",
        piano_roll_x_positions.get("x").unwrap().to_owned() + piano_roll_section_width,
    );

    let mut piano_roll_vecs: HashMap<&str, Vec<Rect>> = HashMap::new();
    piano_roll_vecs.insert("left", Vec::new());
    piano_roll_vecs.insert("up", Vec::new());
    piano_roll_vecs.insert("down", Vec::new());
    piano_roll_vecs.insert("right", Vec::new());
    piano_roll_vecs.insert("l", Vec::new());
    piano_roll_vecs.insert("select", Vec::new());
    piano_roll_vecs.insert("start", Vec::new());
    piano_roll_vecs.insert("r", Vec::new());
    piano_roll_vecs.insert("y", Vec::new());
    piano_roll_vecs.insert("b", Vec::new());
    piano_roll_vecs.insert("x", Vec::new());
    piano_roll_vecs.insert("a", Vec::new());

    'mainloop: loop {
        fps.delay();
        let events = controller.pushed(&mut usb2snes)?;
        canvas.clear();
        // skin background
        canvas.copy(
            &background_texture,
            None,
            Rect::new(
                0,
                0,
                skin.backgrounds[&app_config.skin.skin_theme].width,
                skin.backgrounds[&app_config.skin.skin_theme].height,
            ),
        )?;
        // if piano roll, move each Rect by one pixel
        if app_config.skin.piano_roll {
            for (button, rect_vec) in piano_roll_vecs.iter_mut() {
                for rect in rect_vec.iter_mut() {
                    // dbg!(&fps.get_frame_count());
                    rect.y += 1;
                    canvas.copy(&button_textures[button], None, rect.to_owned())?;
                }

                if !rect_vec.is_empty() && rect_vec[0].y > window_height as i32 {
                    rect_vec.remove(0);
                }
            }
        }

        // buttons presses
        for event in events {
            canvas.copy(&button_textures[event], None, skin.buttons[event].rect)?;

            if app_config.skin.piano_roll {
                let roll_vec = piano_roll_vecs.get_mut(event).unwrap();
                roll_vec.push(Rect::new(
                    piano_roll_x_positions.get(event).unwrap().to_owned() as i32,
                    skin.backgrounds[&app_config.skin.skin_theme]
                        .height
                        .to_owned() as i32,
                    piano_roll_rect_width.to_owned(),
                    1,
                ));
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
