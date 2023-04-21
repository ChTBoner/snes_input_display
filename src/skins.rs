pub mod skin {
    use imageinfo::ImageInfo;
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    use sdl2::rect::Rect;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;

    // #[derive(Debug)]
    pub struct Skin {
        pub metadata: HashMap<String, String>,
        pub backgrounds: HashMap<String, Background>,
        pub buttons: HashMap<String, Button>,
        pub directory: PathBuf,
    }

    impl Skin {
        pub fn new(skins_path: &Path, skin: String) -> Skin {
            let skin_filename = "skin.xml";
            let file_path = skins_path.join(&skin).join(skin_filename);
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
                            let bg = Background::new(t, &directory);
                            backgrounds.push(bg);
                        }
                        b"button" => {
                            let bt = Button::new(t, &directory);
                            buttons.push(bt);
                        }
                        _ => {}
                    },
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
            }
            Self {
                metadata,
                backgrounds: Skin::parse_backgrounds(backgrounds),
                buttons: Skin::parse_buttons(buttons),
                directory,
            }
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

        fn parse_buttons(buttons_vec: Vec<Button>) -> HashMap<String, Button> {
            let mut buttons = HashMap::new();
            for button in buttons_vec {
                buttons.insert(button.name.to_owned(), button);
            }
            buttons
        }
    }

    // #[derive(Debug)]
    pub struct Background {
        pub name: String,
        pub image: PathBuf,
        pub width: u32,
        pub height: u32,
    }

    impl Background {
        fn new(t: BytesStart, dir: &PathBuf) -> Self {
            let attributes = parse_attributes(t);
            let image = Path::new(&dir).join(&attributes["image"]);
            let image_info = ImageInfo::from_file_path(&image).unwrap();
            Self {
                name: attributes["name"].to_owned(),
                image,
                width: image_info.size.width as u32,
                height: image_info.size.height as u32,
            }
        }
    }

    #[derive(Debug)]
    pub struct Button {
        pub name: String,
        pub image: PathBuf,
        pub x: i32,
        pub y: i32,
        pub width: u32,
        pub height: u32,
        pub rect: Rect,
        // pub texture: Texture<'a>
    }

    impl Button {
        fn new(t: BytesStart, dir: &PathBuf) -> Self {
            let attributes = parse_attributes(t);
            let image = Path::new(&dir).join(&attributes["image"]);
            let image_info = ImageInfo::from_file_path(&image).unwrap();
            let x = attributes["x"].parse::<i32>().unwrap();
            let y = attributes["y"].parse::<i32>().unwrap();
            let width = image_info.size.width as u32;
            let height = image_info.size.height as u32;
            Self {
                name: attributes["name"].to_owned(),
                image,
                x,
                y,
                width,
                height,
                rect: Rect::new(x, y, width, height),
                // texture: T
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
}
