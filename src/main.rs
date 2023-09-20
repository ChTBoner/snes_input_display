#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controllers;
mod skins;
// mod viewer;
use controllers::controller::{Controller, Buttons};

use ggez::{
    conf, event,
    graphics::{self, Color, GraphicsContext},
    Context, ContextBuilder, GameResult,
};
use rusb2snes::SyncClient;
use skins::skin::Skin;
// use std::collections::HashMap;
use std::error::Error;
// use viewer::InputViewer;

use configuration::config::AppConfig;

const APP_NAME: &'static str = "Snes Input Display";

struct InputViewer {
    config: AppConfig,
    controller: Controller,
    skin: Skin,
    client: SyncClient,
    events: Vec<Buttons>

}

impl InputViewer {
    fn new(ctx: &mut Context, config: AppConfig) -> Result<Self, Box<dyn Error>> {
        let controller = Controller::new(&config.controller.input_config_path);

        let skin = Skin::new(&config.skin.skins_path, &config.skin.skin_name, ctx)?;

        /* Connect to USB2SNES Server */
        let mut client = SyncClient::connect()?;

        client.set_name(String::from(APP_NAME))?;

        let devices = client.list_device()?;

        client.attach(&devices[0])?;
        let info = client.info()?;
        println!("Attached to {} - {}", info.dev_type, info.version);

        let window_height = match config.skin.piano_roll {
            true => skin.backgrounds[&config.skin.skin_theme].height * 2.0,
            false => skin.backgrounds[&config.skin.skin_theme].height,
        };

        // Set the window size
        ctx.gfx.set_mode(conf::WindowMode {
            width: skin.backgrounds[&config.skin.skin_theme].image.width() as f32,
            height: window_height,
            resizable: true,
            ..Default::default()
        })?;

        Ok(Self {
            config,
            controller,
            skin,
            client,
            events: Vec::new(),

        })
    }
}

impl event::EventHandler for InputViewer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        const DESIRED_FPS: u32 = 60;
        self.events = self.controller.pushed(&mut self.client).unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        // Draw background

        canvas.draw(&self.skin.backgrounds[&self.config.skin.skin_theme].image, graphics::DrawParam::new());

        // Draw inputs
        for event in self.events.iter() {
            // let _x = self.skin.buttons[event].image;
            dbg!(&event);
            dbg!(&self.skin.buttons[&event].rect.x);

            canvas.draw(&self.skin.buttons[&event].image, graphics::DrawParam::default().dest_rect(self.skin.buttons[&event].rect))
        }
        canvas.finish(ctx)
    }
}
fn main() -> Result<GameResult, Box<dyn Error>> {
    /* Setup Configs */
    let app_config = AppConfig::new()?;

    let (mut ctx, event_loop) = ContextBuilder::new(APP_NAME, "ChTBoner")
        .add_resource_path(&app_config.skin.skins_path)
        .window_setup(conf::WindowSetup::default().title(APP_NAME))
        .build()
        .expect("aieee, could not create ggez context!");

    let input_viewer = InputViewer::new(&mut ctx, app_config)?;
    event::run(ctx, event_loop, input_viewer);
}
