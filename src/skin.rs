pub mod skin {
    use quick_xml::events::BytesStart;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;

    #[derive(Debug)]
    pub struct Skin {
        pub metadata: HashMap<String, String>,
        pub backgrounds: HashMap<String, String>,
        pub buttons: HashMap<String, Button>,
        pub directory: PathBuf,
    }

    impl Skin {
        pub fn new(file_path: &Path) -> Skin {
            let file = Skin::load_file(file_path);

            let mut reader = Reader::from_str(&file);
            let mut backgrounds: Vec<Background> = Vec::new();
            let mut buttons: Vec<Button> = Vec::new();
            let mut metadata: HashMap<String, String> = HashMap::new();

            loop {
                // dbg!(&reader.read_event());
                match reader.read_event() {
                    Ok(Event::Start(t)) => metadata = parse_attributes(t),
                    Ok(Event::Empty(t)) => match t.name().as_ref() {
                        b"background" => {
                            let bg = Background::new(t);
                            backgrounds.push(bg);
                        }
                        b"button" => {
                            let bt = Button::new(t);
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
                metadata: metadata,
                backgrounds: Skin::parse_backgrounds(backgrounds),
                buttons: Skin::parse_buttons(buttons),
                directory: file_path.parent().unwrap().to_owned(),
            }
        }

        fn load_file(path: &Path) -> String {
            let mut file = fs::File::open(path).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            text
        }

        fn parse_backgrounds(backgrounds_vec: Vec<Background>) -> HashMap<String, String> {
            let mut backgrounds = HashMap::new();
            for background in backgrounds_vec {
                backgrounds.insert(background.name.to_lowercase(), background.image);
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

    #[derive(Debug)]
    struct Background {
        name: String,
        image: String,
    }

    impl Background {
        fn new(t: BytesStart) -> Self {
            let attributes = parse_attributes(t);
            Self {
                name: attributes["name"].to_owned(),
                image: attributes["image"].to_owned(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Button {
        pub name: String,
        pub image: String,
        pub x: u32,
        pub y: u32,
    }

    impl Button {
        fn new(t: BytesStart) -> Self {
            let attributes = parse_attributes(t);
            Self {
                name: attributes["name"].to_owned(),
                image: attributes["image"].to_owned(),
                x: attributes["x"].parse::<u32>().unwrap(),
                y: attributes["y"].parse::<u32>().unwrap(),
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
