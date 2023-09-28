pub mod skin {
    use ggez::{
        graphics::{Image, Rect},
        Context,
    };
    use quick_xml::{
        events::{BytesStart, Event},
        reader::Reader,
    };
    use std::{collections::HashMap, error::Error, fs, io::Read, path::Path, path::PathBuf};

    use crate::controllers::controller::Pressed;

    // #[derive(Debug)]
    pub struct Skin {
        // pub metadata: HashMap<String, String>,
        pub background: Theme,
        pub buttons: HashMap<Pressed, Button>,
        pub directory: PathBuf,
        pub name: String,
        pub theme: String,
    }

    impl Skin {
        pub fn new(
            path: &Path,
            name: &String,
            theme: &String,
            ctx: &mut Context,
        ) -> Result<Skin, Box<dyn Error>> {
            let skin_filename = "skin.xml";
            let file_path = path.join(name).join(skin_filename);
            let directory = file_path.parent().unwrap().to_owned();

            let (backgrounds, buttons) = Self::get_layout(file_path, name, ctx)?;
            let background = Self::parse_backgrounds(backgrounds, theme).unwrap();
            Ok(Self {
                // metadata,
                background,
                buttons: Skin::parse_buttons(buttons),
                directory,
                name: name.to_owned(),
                theme: theme.to_owned(),
            })
        }

        fn get_layout(
            file_path: PathBuf,
            name: &str,
            ctx: &mut Context,
        ) -> Result<(Vec<Theme>, Vec<Button>), Box<dyn Error>> {
            let file = Self::load_file(&file_path);
            let mut reader = Reader::from_str(&file);
            let mut _metadata: HashMap<String, String> = HashMap::new();
            let mut backgrounds: Vec<Theme> = Vec::new();
            let mut buttons: Vec<Button> = Vec::new();

            loop {
                match reader.read_event() {
                    // Ok(Event::Start(t)) => _metadata = parse_attributes(t),
                    Ok(Event::Empty(t)) => match t.name().as_ref() {
                        b"background" => {
                            let bg = Theme::new(t, name, ctx)?;
                            backgrounds.push(bg);
                        }
                        b"button" => {
                            let bt = Button::new(t, name, ctx)?;
                            buttons.push(bt);
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

        fn load_file(path: &Path) -> String {
            let mut file = fs::File::open(path).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            text
        }

        fn parse_backgrounds(backgrounds_vec: Vec<Theme>, theme: &String) -> Option<Theme> {
            for background in backgrounds_vec {
                dbg!(&background);
                dbg!(&theme);
                if background.theme.eq(theme) {
                    return Some(background);
                }
            }
            None
        }

        fn parse_buttons(buttons_vec: Vec<Button>) -> HashMap<Pressed, Button> {
            let mut buttons = HashMap::new();
            for button in buttons_vec {
                buttons.insert(button.name, button);
            }
            buttons
        }
    }

    #[derive(Debug, Clone)]
    pub struct Theme {
        pub theme: String,
        pub image: Image,
        pub width: f32,
        pub height: f32,
    }

    impl Theme {
        fn new(t: BytesStart, dir: &str, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
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
        fn new(t: BytesStart, dir: &str, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
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

}
