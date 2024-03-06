#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod configuration;
mod controller;
mod skins;
use controller::{ButtonState, Controller};

use ggez::{
    conf, event,
    graphics::{self, DrawParam},
    Context, ContextBuilder, GameResult,
};
use rusb2snes::SyncClient;
use skins::Skin;
use std::error::Error;

use configuration::AppConfig;

const APP_NAME: &str = "Snes Input Display";

enum AppState {
    Menu,
    InputViewer,
}

struct InputViewer {
    // config: AppConfig,
    state: AppState,
    controller: Controller,
    skin: Skin,
    client: SyncClient,
    events: ButtonState,
}

impl InputViewer {
    fn new(ctx: &mut Context, config: AppConfig) -> Result<Self, Box<dyn Error>> {
        dbg!(&config.controller.input_config_path);
        let controller = Controller::new(&config.controller);

        let skin = Skin::new(
            &config.skin.skins_path,
            &config.skin.skin_name,
            &config.skin.skin_theme,
            ctx,
        )?;

        /* Connect to USB2SNES Server */
        let mut client = SyncClient::connect()?;

        client.set_name(String::from(APP_NAME))?;

        let devices = client.list_device()?;
        client.attach(&devices[0])?;
        let game_name = client.get_address(0x7FC0, 16)?;
        for i in game_name {
            let num = format!("{}", char::from_u32(i as u32).unwrap());
            println!("{}", num);
        }
        let _info = client.info()?;

        // Set the window size
        ctx.gfx.set_mode(conf::WindowMode {
            width: skin.background.image.width() as f32,
            height: skin.background.height,
            resizable: true,
            ..Default::default()
        })?;

        Ok(Self {
            // config,
            state: AppState::Menu,
            controller,
            skin,
            client,
            events: ButtonState::default(),
        })
    }
}

impl event::EventHandler for InputViewer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        // const DESIRED_FPS: u32 = 60;
        self.events = self.controller.pushed(&mut self.client).unwrap();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);
        match self.state {
            AppState::Menu => {
                self.state = AppState::InputViewer;
            }
            AppState::InputViewer => {
                // Draw background
                canvas.draw(&self.skin.background.image, DrawParam::new());

                // Draw inputs
                self.events.iter().for_each(|event| {
                    let button_image = &self.skin.buttons[event].image;
                    canvas.draw(
                        button_image,
                        DrawParam::default().dest(self.skin.buttons[event].rect.point()),
                    );
                });
            }
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
