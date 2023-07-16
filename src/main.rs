#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controllers;
mod skins;
use bevy::asset::AssetPath;
use bevy::prelude::*;
use bevy::transform::commands;
use bevy::window::WindowResolution;
use controllers::controller::Controller;

// use sdl2::event::Event;
// use sdl2::image::{InitFlag, LoadTexture};
// use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use rusb2snes::SyncClient;
use skins::skin::Skin;

use configuration::config::AppConfig;

fn main() -> Result<(), String> {
    let app_config = AppConfig::new().unwrap();
    
    let controller = Controller::new(&app_config.controller.input_config_path);

    let skin = Skin::new(&app_config.skin.skins_path, app_config.skin.skin_name);

    /* Connect to USB2SNES Server */
    
 
    App::new()
    .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 1.0, 0.7)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(skin.backgrounds[&app_config.skin.skin_theme].width, skin.backgrounds[&app_config.skin.skin_theme].height),
            title: "SNES Input Display".to_string(),
            ..default()
            }),
            ..default()
        },
    ))
    .add_systems(Startup, setup)
    .run();

   
    Ok(())
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
     /* Setup Configs */
    let mut usb2snes = SyncClient::connect().unwrap();

    usb2snes.set_name(String::from("Snes Input Viewer")).unwrap(); 
    let devices = usb2snes.list_device().unwrap();
 
    usb2snes.attach(&devices[0]).unwrap();
    let info = usb2snes.info().unwrap();
    println!("Attached to {} - {}", info.dev_type, info.version);

    let background_image = AssetPath::new(skin.backgrounds[&app_config.skin.skin_theme].image.to_owned(), None);
    
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load(background_image),
        ..default()
    });
}