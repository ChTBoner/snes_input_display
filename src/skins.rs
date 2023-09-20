pub mod skin {
    // use imageinfo::ImageInfo;
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    // use sdl2::rect::Rect;
    use ggez::{
        graphics::{Image, Rect},
        Context,
    };
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;

    use crate::controllers::controller::Buttons;

    // #[derive(Debug)]
    pub struct Skin {
        pub metadata: HashMap<String, String>,
        pub backgrounds: HashMap<String, Background>,
        pub buttons: HashMap<Buttons, Button>,
        pub directory: PathBuf,
    }

    impl Skin {
        pub fn new(
            skins_path: &Path,
            skin_name: &String,
            ctx: &mut Context,
        ) -> Result<Skin, Box<dyn Error>> {
            let skin_filename = "skin.xml";
            let file_path = skins_path.join(&skin_name).join(skin_filename);
            let file = Skin::load_file(&file_path);

            let mut reader = Reader::from_str(&file);
            let mut backgrounds: Vec<Background> = Vec::new();
            let mut buttons: Vec<Button> = Vec::new();
            let mut metadata: HashMap<String, String> = HashMap::new();
            let directory = file_path.parent().unwrap().to_owned();

            loop {
                match reader.read_event() {
                    Ok(Event::Start(t)) => metadata = parse_attributes(t),
                    Ok(Event::Empty(t)) => match t.name().as_ref() {
                        b"background" => {
                            let bg = Background::new(t, &skin_name, ctx)?;
                            backgrounds.push(bg);
                        }
                        b"button" => {
                            let bt = Button::new(t, &skin_name, ctx)?;
                            buttons.push(bt);
                        }
                        _ => {}
                    },
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
            }
            Ok(Self {
                metadata,
                backgrounds: Skin::parse_backgrounds(backgrounds),
                buttons: Skin::parse_buttons(buttons),
                directory,
            })
        }

        fn load_file(path: &Path) -> String {
            let mut file = fs::File::open(path).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            text
        }

        fn parse_backgrounds(backgrounds_vec: Vec<Background>) -> HashMap<String, Background> {
            let mut backgrounds = HashMap::new();
            for background in backgrounds_vec {
                backgrounds.insert(background.name.to_lowercase(), background);
            }
            backgrounds
        }

        fn parse_buttons(buttons_vec: Vec<Button>) -> HashMap<Buttons, Button> {
            let mut buttons = HashMap::new();
            for button in buttons_vec {
                buttons.insert(button.name, button);
            }
            buttons
        }
    }

    // #[derive(Debug)]
    pub struct Background {
        pub name: String,
        pub image: Image,
        pub width: f32,
        pub height: f32,
    }

    impl Background {
        fn new(t: BytesStart, dir: &str, ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
            let attributes = parse_attributes(t);
            let image_path = Path::new("/").join(dir).join(&attributes["image"]);
            let image = Image::from_path(ctx, image_path)?;
            // let image_info = ImageInfo::from_file_path(&image_path)?;
            let width = image.width() as f32;
            let height = image.height() as f32;
            // let image_path = Path::new("/").join(&attributes["image"]);
            // let image_info = ImageInfo::from_file_path(&image_path)?;
            Ok(Self {
                name: attributes["name"].to_owned(),
                image,
                width,
                height,
            })
        }
    }

    #[derive(Debug)]
    pub struct Button {
        pub name: Buttons,
        pub image: Image,
        // pub x: f32,
        // pub y: f32,
        // pub width: f32,
        // pub height: f32,
        pub rect: Rect,
        // pub texture: Texture<'a>
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
                "a" => Buttons::A,
                "b" => Buttons::B,
                "x" => Buttons::X,
                "y" => Buttons::Y,
                "select" => Buttons::Select,
                "start" => Buttons::Start,
                "l" => Buttons::L,
                "r" => Buttons::R,
                "up" => Buttons::Up,
                "down" => Buttons::Down,
                "left" => Buttons::Left,
                "right" => Buttons::Right,
                _ => panic!()
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

    // pub struct RollPress {
    //     pub x: i32,
    //     y: i32,
    //     pub height: u32,
    //     pub width: u32,

    // }

    // impl RollPress {
    //     pub fn new(x: i32, y: i32, width: u32) -> Self {
    //         Self { x, y, height: 1, width }
    //     }
    // }
}
