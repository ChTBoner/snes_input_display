pub mod controller {
    use bitvec::prelude::*;
    use serde::Deserialize;
    use serde_json;
    use std::{fs, path::Path};

    use crate::qusb2snes::usb2snes::SyncClient;

    pub enum Inputs {
        A,
        B,
        X,
        Y,
        Select,
        Start,
        Up,
        Down,
        Left,
        Right,
        L,
        R,
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

        pub fn pushed(&self, client: &mut SyncClient) -> Vec<Inputs> {
            let inputs = client.get_address(self.address, self.size);
            let bits = inputs.view_bits::<Msb0>();
            let mut controller_events = Vec::new();

            if bits[self.button_layout.a] {
                controller_events.push(Inputs::A);
            };
            if bits[self.button_layout.x] {
                controller_events.push(Inputs::X);
            };
            if bits[self.button_layout.b] {
                controller_events.push(Inputs::B);
            };
            if bits[self.button_layout.y] {
                controller_events.push(Inputs::Y);
            };
            if bits[self.button_layout.select] {
                controller_events.push(Inputs::Select);
            };
            if bits[self.button_layout.start] {
                controller_events.push(Inputs::Start);
            };
            if bits[self.button_layout.up] {
                controller_events.push(Inputs::Up);
            };
            if bits[self.button_layout.down] {
                controller_events.push(Inputs::Down);
            };
            if bits[self.button_layout.left] {
                controller_events.push(Inputs::Left);
            };
            if bits[self.button_layout.right] {
                controller_events.push(Inputs::Right);
            };
            if bits[self.button_layout.l] {
                controller_events.push(Inputs::L);
            };
            if bits[self.button_layout.r] {
                controller_events.push(Inputs::R);
            };
            controller_events
        }
    }
}
