use ggez::{
    graphics::{Image, Rect},
    Context,
};
use quick_xml::{
    events::{BytesStart, Event},
    reader::Reader,
};
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    ffi::{OsStr, OsString},
    fmt, fs, io,
    io::Read,
    path::{Path, PathBuf},
};

use crate::controller::Pressed;
use walkdir::WalkDir;

#[derive(Debug)]
struct NotASnesSkin(String);

impl fmt::Display for NotASnesSkin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for NotASnesSkin {}

type LayoutResult = Result<(Vec<Theme>, BTreeMap<Pressed, Button>), Box<dyn Error>>;

// #[derive(Debug)]
pub struct Skin {
    // pub metadata: HashMap<String, String>,
    pub background: Theme,
    pub buttons: Box<ButtonsMap>,
    pub directory: PathBuf,
    pub name: String,
    pub theme: String,
}

impl Skin {
    pub fn new(
        path: &Path,
        name: &String,
        theme: Option<&String>,
        ctx: &mut Context,
    ) -> Result<Skin, Box<dyn Error>> {
        let skin_filename = "skin.xml";
        let file_path = path.join(name).join(skin_filename);
        let directory = file_path.parent().unwrap().to_owned();

        let (backgrounds, buttons) = Skin::get_layout(&file_path, name, ctx)?;

        let background = Skin::parse_backgrounds(backgrounds, theme).unwrap();
        let theme_name = background.theme.to_owned();
        Ok(Self {
            // metadata,
            background,
            buttons: buttons_map_to_array(buttons),
            directory,
            name: name.to_owned(),
            theme: theme_name,
        })
    }

    pub fn list_available_skins(
        path: &PathBuf,
        ctx: &mut Context,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut available_skins: Vec<String> = Vec::new();

        for entry in WalkDir::new(path).into_iter() {
            let entry = entry?;
            if entry.file_name() == "skin.xml" {
                let skin_name = entry
                    .path()
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();

                match Skin::get_layout(entry.path(), &skin_name, ctx) {
                    Ok(_) => available_skins.push(skin_name.into()),
                    Err(e) => println!("Not a Snes Skin: {e}"),
                };
            }
        }
        available_skins.sort();

        Ok(available_skins)
    }

    fn get_layout(file_path: &Path, name: &String, ctx: &mut Context) -> LayoutResult {
        let file = load_file(file_path);
        let mut reader = Reader::from_str(&file);
        let mut backgrounds: Vec<Theme> = Vec::new();
        let mut buttons: BTreeMap<Pressed, Button> = BTreeMap::new();

        loop {
            match reader.read_event() {
                // check if it's a snes skin type
                Ok(Event::Start(t)) => {
                    if t.name().as_ref() == b"skin" {
                        let metadata: HashMap<String, String> = parse_attributes(t);
                        if metadata.get("type").unwrap() != &"snes".to_string() {
                            return Err(Box::new(NotASnesSkin("Oops".into())));
                        }
                    }
                }
                Ok(Event::Empty(t)) => match t.name().as_ref() {
                    b"background" => {
                        let bg = Theme::new(t, name, ctx)?;
                        backgrounds.push(bg);
                    }
                    b"button" => {
                        let bt = Button::new(t, name, ctx)?;
                        buttons.insert(bt.name, bt);
                    }
                    _ => {}
                },
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                _ => (),
            }
        }

        Ok((backgrounds, buttons))
    }

    fn parse_backgrounds(backgrounds_vec: Vec<Theme>, theme: Option<&String>) -> Option<Theme> {
        match theme {
            Some(t) => backgrounds_vec
                .into_iter()
                .find(|background| background.theme.eq(t)),
            // return first theme available
            None => Some(backgrounds_vec[0].to_owned()),
        }
    }
}

fn parse_attributes(t: BytesStart) -> HashMap<String, String> {
    let mut attributes_map = HashMap::new();
    let attributes = t.attributes().map(|a| a.unwrap());
    for attribute in attributes {
        let value = attribute.unescape_value().unwrap().into_owned();
        let mut key = String::new();
        attribute
            .key
            .local_name()
            .into_inner()
            .read_to_string(&mut key)
            .unwrap();

        attributes_map.insert(key, value);
    }
    attributes_map
}

fn load_file(path: &Path) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

/// Produces an boxed array indexable by `Pressed` that maps a single button press to an
/// initialized `Button`.
fn buttons_map_to_array(mut buttons_map: BTreeMap<Pressed, Button>) -> Box<ButtonsMap> {
    debug_assert!(buttons_map.len() >= 12);

    Box::new(ButtonsMap([
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
        buttons_map.pop_first().unwrap().1,
    ]))
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub theme: String,
    pub image: Image,
    pub width: f32,
    pub height: f32,
}

impl Theme {
    fn new(t: BytesStart, dir: &String, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
        let attributes = parse_attributes(t);
        let image_path = Path::new("/").join(dir).join(&attributes["image"]);
        let image = Image::from_path(ctx, image_path)?;
        let width = image.width() as f32;
        let height = image.height() as f32;

        Ok(Self {
            theme: attributes["name"].to_owned().to_lowercase(),
            image,
            width,
            height,
        })
    }
}

#[derive(Debug)]
pub struct Button {
    pub name: Pressed,
    pub image: Image,
    pub rect: Rect,
}

impl Button {
    fn new(t: BytesStart, dir: &String, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
        let attributes = parse_attributes(t);
        let x = attributes["x"].parse::<f32>().unwrap();
        let y = attributes["y"].parse::<f32>().unwrap();
        let image_path = Path::new("/").join(dir).join(&attributes["image"]);

        let image = Image::from_path(ctx, image_path)?;
        // let image_info = ImageInfo::from_file_path(&image_path)?;
        let width = image.width() as f32;
        let height = image.height() as f32;

        let name = match attributes["name"].as_str() {
            "a" => Pressed::A,
            "b" => Pressed::B,
            "x" => Pressed::X,
            "y" => Pressed::Y,
            "select" => Pressed::Select,
            "start" => Pressed::Start,
            "l" => Pressed::L,
            "r" => Pressed::R,
            "up" => Pressed::Up,
            "down" => Pressed::Down,
            "left" => Pressed::Left,
            "right" => Pressed::Right,
            _ => panic!(),
        };

        Ok(Self {
            name,
            image,
            rect: Rect::new(x, y, width, height),
        })
    }
}

/// A wrapper over an array `[Button; 12]` indexable by `Pressed`. The array is internally ordered
/// by a button's bit ascending from lowest bit to highest.
#[derive(Debug)]
pub struct ButtonsMap([Button; 12]);

impl std::ops::Index<Pressed> for ButtonsMap {
    type Output = Button;

    fn index(&self, pressed: Pressed) -> &Self::Output {
        let index = match pressed {
            Pressed::R => 0,
            Pressed::L => 1,
            Pressed::X => 2,
            Pressed::A => 3,
            Pressed::Right => 4,
            Pressed::Left => 5,
            Pressed::Down => 6,
            Pressed::Up => 7,
            Pressed::Start => 8,
            Pressed::Select => 9,
            Pressed::Y => 10,
            Pressed::B => 11,
        };

        &self.0[index]
    }
}

impl std::ops::IndexMut<Pressed> for ButtonsMap {
    fn index_mut(&mut self, pressed: Pressed) -> &mut Self::Output {
        let index = match pressed {
            Pressed::R => 0,
            Pressed::L => 1,
            Pressed::X => 2,
            Pressed::A => 3,
            Pressed::Right => 4,
            Pressed::Left => 5,
            Pressed::Down => 6,
            Pressed::Up => 7,
            Pressed::Start => 8,
            Pressed::Select => 9,
            Pressed::Y => 10,
            Pressed::B => 11,
        };

        &mut self.0[index]
    }
}
