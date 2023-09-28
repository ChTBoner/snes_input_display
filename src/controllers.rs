pub mod controller {
    use bitvec::prelude::*;
    use serde::Deserialize;
    use serde_json;
    use std::{fs, path::Path};
    use tungstenite::error::Error;

    use rusb2snes::SyncClient;

    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
    pub enum Pressed {
        A,
        B,
        X,
        Y,
        L,
        R,
        Select,
        Start,
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Deserialize, Debug)]
    pub struct ButtonLayout {
        pub b: usize,
        pub y: usize,
        pub select: usize,
        pub start: usize,
        pub up: usize,
        pub down: usize,
        pub left: usize,
        pub right: usize,
        pub a: usize,
        pub x: usize,
        pub l: usize,
        pub r: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Controller {
        pub address: u32,
        pub size: usize,
        pub button_layout: ButtonLayout,
    }

    impl Controller {
        pub fn new(config_path: &Path) -> Self {
            let config_data = fs::read_to_string(config_path).expect("Unable to config file");
            serde_json::from_str(&config_data).expect("Unable to parse")
        }

        pub fn pushed(&self, client: &mut SyncClient) -> Result<Vec<Pressed>, Error> {
            let inputs = client.get_address(self.address, self.size)?;
            let bits = inputs.view_bits::<Msb0>();
            let mut inputs: Vec<Pressed> = Vec::new();

            if bits[self.button_layout.a] {
                inputs.push(Pressed::A);
            };
            if bits[self.button_layout.x] {
                inputs.push(Pressed::X);
            };
            if bits[self.button_layout.b] {
                inputs.push(Pressed::B);
            };
            if bits[self.button_layout.y] {
                inputs.push(Pressed::Y);
            };
            if bits[self.button_layout.select] {
                inputs.push(Pressed::Select);
            };
            if bits[self.button_layout.start] {
                inputs.push(Pressed::Start);
            };
            if bits[self.button_layout.up] {
                inputs.push(Pressed::Up);
            };
            if bits[self.button_layout.down] {
                inputs.push(Pressed::Down);
            };
            if bits[self.button_layout.left] {
                inputs.push(Pressed::Left);
            };
            if bits[self.button_layout.right] {
                inputs.push(Pressed::Right);
            };
            if bits[self.button_layout.l] {
                inputs.push(Pressed::L);
            };
            if bits[self.button_layout.r] {
                inputs.push(Pressed::R);
            };
            Ok(inputs)
        }
    }
}
