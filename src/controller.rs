pub mod controller {
    use std::fs;
    use serde::Deserialize;
    use serde_json;
    use bitvec::prelude::*;
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
        pub r: usize
    }

    #[derive(Deserialize, Debug)]
    pub struct Controller {
        pub address: u32,
        pub size: usize,
        pub button_layout: ButtonLayout
    }

    impl Controller {
        pub fn new(path: String) -> Self {
            let data = fs::read_to_string(path).expect("Unable to read file");
            let c: Self = serde_json::from_str(&data).expect("Unable to parse");
            c
        }

        pub fn pushed(&self, client: &mut SyncClient ) {
            let inputs = client.get_address(self.address, self.size);
            let mut input_string = "".to_string();
            let bits = inputs.view_bits::<Msb0>();
            if bits[self.button_layout.a] {
                input_string.push_str("A ");
            };
            if bits[self.button_layout.x] {
                input_string.push_str("X ");
            };
            if bits[self.button_layout.b] {
                input_string.push_str("B ");
            };
            if bits[self.button_layout.y] {
                input_string.push_str("Y ");
            };
            if bits[self.button_layout.select] {
                input_string.push_str("Select ");
            };
            if bits[self.button_layout.start] {
                input_string.push_str("Start ");
            };
            if bits[self.button_layout.up] {
                input_string.push_str("Up ");
            };
            if bits[self.button_layout.down] {
                input_string.push_str("Down ");
            };
            if bits[self.button_layout.left] {
                input_string.push_str("Left ");
            };
            if bits[self.button_layout.right] {
                input_string.push_str("Right ");
            };
            if bits[self.button_layout.l] {
                input_string.push_str("L ");
            };
            if bits[self.button_layout.r] {
                input_string.push_str("R ");
            };
            println!("{}", input_string)
            }
    }
}

