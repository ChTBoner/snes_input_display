use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::{fs, path::Path};

use rusb2snes::SyncClient;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u16)]
pub enum Pressed {
    R = 0x0010,
    L = 0x0020,
    X = 0x0040,
    A = 0x0080,
    Right = 0x0100,
    Left = 0x0200,
    Down = 0x0400,
    Up = 0x0800,
    Start = 0x1000,
    Select = 0x2000,
    Y = 0x4000,
    B = 0x8000,
}

impl Pressed {
    /// Accepts a `u16` with a single bit set according to the SNES joypad register layout and
    /// returns `Option<Pressed>` where None represents no buttons pushed. Caller is responsible
    /// for ensuring that the value passed in is zero or a single, valid bit. Otherwise the
    /// function will panic.
    pub fn try_from_bit(bit: u16) -> Option<Self> {
        debug_assert!(bit.is_power_of_two() || bit == 0);
        match bit {
            0x0000 => None,
            0x0010 => Some(Pressed::R),
            0x0020 => Some(Pressed::L),
            0x0040 => Some(Pressed::X),
            0x0080 => Some(Pressed::A),
            0x0100 => Some(Pressed::Right),
            0x0200 => Some(Pressed::Left),
            0x0400 => Some(Pressed::Down),
            0x0800 => Some(Pressed::Up),
            0x1000 => Some(Pressed::Start),
            0x2000 => Some(Pressed::Select),
            0x4000 => Some(Pressed::Y),
            0x8000 => Some(Pressed::B),
            _ => panic!(
                "Called Pressed::try_from_bit with invalid input: {:#06x}",
                bit
            ),
        }
    }
}

/// A `u16` backed bitfield representing a controller state according to the SNES joypad register
/// layout.
#[derive(Debug, Copy, Clone, Default)]
#[repr(transparent)]
pub struct ButtonState(u16);

impl ButtonState {
    /// Construct a `ButtonState` from little-endian bytes. The low and high bytes correspond to
    /// the low and high bytes of the SNES joypad registers.
    pub fn from_le_bytes(bytes: [u8; 2]) -> Self {
        ButtonState(u16::from_le_bytes(bytes))
    }

    /// Provides an iterator over the buttons pressed in this `ButtonState` which returns
    /// `Option<Pressed>`.
    pub fn iter(&self) -> ButtonsIter {
        ButtonsIter {
            bitfield: self.0,
            cursor_offset: 0,
        }
    }
}

/// An iterator over the buttons currently pressed in a given ButtonState. Iterates from the
/// highest bit to the lowest according to the SNES joypad register layout.
pub struct ButtonsIter {
    bitfield: u16,
    cursor_offset: u16,
}

impl ButtonsIter {
    const BIT_CURSOR: u16 = 0x8000;
}

impl Iterator for ButtonsIter {
    type Item = Pressed;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match Self::BIT_CURSOR.checked_shr(self.cursor_offset as u32) {
                None => break None,
                Some(bitmask) => {
                    self.cursor_offset = self.cursor_offset.saturating_add(1);
                    match (self.bitfield & bitmask) > 0 {
                        true => break Pressed::try_from_bit(bitmask),
                        false => continue,
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Controller {
    #[serde(deserialize_with = "hex_to_u32")]
    pub address_low: u32,
    #[serde(deserialize_with = "hex_to_u32")]
    pub address_high: u32,
}

impl Controller {
    pub fn new(config_path: &Path) -> Self {
        let config_data = fs::read_to_string(config_path).expect("Unable to config file");
        serde_json::from_str(&config_data).expect("Unable to parse")
    }

    pub fn pushed(&self, client: &mut SyncClient) -> Result<ButtonState, Box<dyn Error>> {
        let base_address = std::cmp::min(self.address_low, self.address_high);
        let offset_low = self.address_low.saturating_sub(base_address) as usize;
        let offset_high = self.address_high.saturating_sub(base_address) as usize;
        let read_length = offset_low.abs_diff(offset_high).saturating_add(1);
        debug_assert!((2..256).contains(&read_length));
        let input_bytes = client.get_address(base_address, read_length)?;
        let button_state =
            ButtonState::from_le_bytes([input_bytes[offset_low], input_bytes[offset_high]]);

        Ok(button_state)
    }
}

/// Serialization function for converting a 24-bit hex address string into `u32`.
fn hex_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let hex_address = String::deserialize(deserializer)?;
    u32::from_str_radix(&hex_address, 16).map_err(Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_buttons_iter() {
        let mut some_buttons_iter = ButtonState::from_le_bytes([0xA0, 0x03]).iter();
        assert_eq!(Some(Pressed::Left), some_buttons_iter.next());
        assert_eq!(Some(Pressed::Right), some_buttons_iter.next());
        assert_eq!(Some(Pressed::A), some_buttons_iter.next());
        assert_eq!(Some(Pressed::L), some_buttons_iter.next());
        assert_eq!(None, some_buttons_iter.next());

        let mut no_buttons_iter = ButtonState::from_le_bytes([0x00, 0x00]).iter();
        assert_eq!(None, no_buttons_iter.next());
    }
}
