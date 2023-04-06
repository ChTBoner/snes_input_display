pub mod controller {
    use std::fs;
    use serde::Deserialize;
    use serde_json;
    use bitvec::prelude::*;
    use crate::qusb2snes::usb2snes::SyncClient;

    pub enum ControllerEvents {
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

        pub fn pushed(&self, client: &mut SyncClient ) -> Vec<ControllerEvents> {
            let inputs = client.get_address(self.address, self.size);
            let mut input_string = "".to_string();
            let bits = inputs.view_bits::<Msb0>();
            let mut controller_events = Vec::new();

            if bits[self.button_layout.a] {
                controller_events.push(ControllerEvents::A);
            };
            if bits[self.button_layout.x] {
                controller_events.push(ControllerEvents::X);
            };
            if bits[self.button_layout.b] {
                controller_events.push(ControllerEvents::B);
            };
            if bits[self.button_layout.y] {
                controller_events.push(ControllerEvents::Y);
            };
            if bits[self.button_layout.select] {
                controller_events.push(ControllerEvents::Select);
            };
            if bits[self.button_layout.start] {
                controller_events.push(ControllerEvents::Start);
            };
            if bits[self.button_layout.up] {
                controller_events.push(ControllerEvents::Up);
            };
            if bits[self.button_layout.down] {
                controller_events.push(ControllerEvents::Down);
            };
            if bits[self.button_layout.left] {
                controller_events.push(ControllerEvents::Left);
            };
            if bits[self.button_layout.right] {
                controller_events.push(ControllerEvents::Right);
            };
            if bits[self.button_layout.l] {
                controller_events.push(ControllerEvents::L);
            };
            if bits[self.button_layout.r] {
                controller_events.push(ControllerEvents::R);
            };
            controller_events
        }
    }
}

