pub mod skin {
    use quick_xml::events::BytesStart;
    use quick_xml::events::Event;
    use quick_xml::reader::Reader;
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    #[derive(Debug)]
    pub struct Skin {
        pub name: String,
        pub author: String,
        pub console: String,
        pub backgrounds: HashMap<String, String>,
        pub buttons: HashMap<String, Button>,
    }

    impl Skin {
        pub fn new(file_path: &Path) -> Skin {
            let file = Skin::load_file(file_path);

            let mut reader = Reader::from_str(&file);

            let mut backgrounds: Vec<Background> = Vec::new();
            let mut buttons: Vec<Button> = Vec::new();
            let mut skin = Skin::empty();

            loop {
                // dbg!(&reader.read_event());
                match reader.read_event() {
                    Ok(Event::Start(t)) => skin.update(t),
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
            skin.parse_backgrounds(backgrounds);
            skin.parse_buttons(buttons);
            skin
        }

        fn load_file(path: &Path) -> String {
            let mut file = fs::File::open(path).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            text
        }

        fn empty() -> Self {
            Self {
                name: String::new(),
                console: String::new(),
                author: String::new(),
                backgrounds: HashMap::new(),
                buttons: HashMap::new(),
            }
        }

        fn update(&mut self, t: BytesStart) {
            let attributes = parse_attributes(t);
            self.name = attributes["name"].to_owned();
            self.author = attributes["author"].to_owned();
            self.console = attributes["type"].to_owned();
        }

        fn parse_backgrounds(&mut self, backgrounds: Vec<Background>) {
            for background in backgrounds {
                self.backgrounds
                    .insert(background.name.to_lowercase(), background.image);
            }
        }

        fn parse_buttons(&mut self, buttons: Vec<Button>) {
            for button in buttons {
                self.buttons.insert(button.name.to_owned(), button);
            }
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
