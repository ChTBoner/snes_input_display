pub mod controller {
    use bitvec::prelude::*;
    use serde::Deserialize;
    use serde_json;
    use std::{fs, path::Path};

    use crate::qusb2snes::usb2snes::SyncClient;


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

        pub fn pushed(&self, client: &mut SyncClient) -> Vec<&str> {
            let inputs = client.get_address(self.address, self.size);
            let bits = inputs.view_bits::<Msb0>();
            let mut inputs = Vec::new();

            if bits[self.button_layout.a] {
                inputs.push("a");
            };
            if bits[self.button_layout.x] {
                inputs.push("x");
            };
            if bits[self.button_layout.b] {
                inputs.push("b");
            };
            if bits[self.button_layout.y] {
                inputs.push("y");
            };
            if bits[self.button_layout.select] {
                inputs.push("select");
            };
            if bits[self.button_layout.start] {
                inputs.push("start");
            };
            if bits[self.button_layout.up] {
                inputs.push("up");
            };
            if bits[self.button_layout.down] {
                inputs.push("down");
            };
            if bits[self.button_layout.left] {
                inputs.push("left");
            };
            if bits[self.button_layout.right] {
                inputs.push("right");
            };
            if bits[self.button_layout.l] {
                inputs.push("l");
            };
            if bits[self.button_layout.r] {
                inputs.push("r");
            };
            inputs
        }
    }
}
