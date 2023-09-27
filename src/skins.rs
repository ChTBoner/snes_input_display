pub mod skin {
    use ggez::{
        graphics::{Image, Rect, Canvas},
        Context,
    };
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::reader::Reader;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;
    use std::io::Read;
    use std::path::Path;
    use std::path::PathBuf;

    use crate::controllers::controller::Buttons;

    // #[derive(Debug)]
    pub struct Skin {
        // pub metadata: HashMap<String, String>,
        pub background: Background,
        pub buttons: HashMap<Buttons, Button>,
        pub directory: PathBuf,
        pub name: String,
        pub theme: String,
        pub piano_roll: PianoRoll,
    }

    impl Skin {
        pub fn new(
            path: &Path,
            name: &String,
            theme: &String,
            ctx: &mut Context,
        ) -> Result<Skin, Box<dyn Error>> {
            let skin_filename = "skin.xml";
            let file_path = path.join(&name).join(skin_filename);
            let directory = file_path.parent().unwrap().to_owned();

            let (backgrounds, buttons) = Self::get_layout(file_path, name, ctx)?;
            let background = Self::parse_backgrounds(backgrounds, theme).unwrap();
            Ok(Self {
                // metadata,
                piano_roll: PianoRoll::new(&background),
                background,
                buttons: Skin::parse_buttons(buttons),
                directory,
                name: name.to_owned(),
                theme: theme.to_owned(),
            })
        }

        fn get_layout(
            file_path: PathBuf,
            name: &String,
            ctx: &mut Context,
        ) -> Result<(Vec<Background>, Vec<Button>), Box<dyn Error>> {
            let file = Self::load_file(&file_path);
            let mut reader = Reader::from_str(&file);
            let mut _metadata: HashMap<String, String> = HashMap::new();
            let mut backgrounds: Vec<Background> = Vec::new();
            let mut buttons: Vec<Button> = Vec::new();

            loop {
                match reader.read_event() {
                    // Ok(Event::Start(t)) => _metadata = parse_attributes(t),
                    Ok(Event::Empty(t)) => match t.name().as_ref() {
                        b"background" => {
                            let bg = Background::new(t, &name, ctx)?;
                            backgrounds.push(bg);
                        }
                        b"button" => {
                            let bt = Button::new(t, &name, ctx)?;
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

        fn parse_backgrounds(
            backgrounds_vec: Vec<Background>,
            theme: &String,
        ) -> Option<Background> {
            for background in backgrounds_vec {
                dbg!(&background);
                dbg!(&theme);
                if background.theme.eq(theme) {
                    return Some(background);
                }
            }
            None
        }

        fn parse_buttons(buttons_vec: Vec<Button>) -> HashMap<Buttons, Button> {
            let mut buttons = HashMap::new();
            for button in buttons_vec {
                buttons.insert(button.name, button);
            }
            buttons
        }
    }

    #[derive(Debug, Clone)]
    pub struct Background {
        pub theme: String,
        pub image: Image,
        pub width: f32,
        pub height: f32,
    }

    impl Background {
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
        pub name: Buttons,
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

    pub struct PianoRoll {
        // width of section reserved for each button
        section_width: f32,
        // extra space left after division: modulo of section_width calculation
        extra_width: f32,
        rect_width: f32,
        // padding inside each section
        inside_padding: f32,
        left_padding: f32,
        // hashmap of all positions
        pub x_positions: HashMap<Buttons, PianoRollRect>,
    }

    impl PianoRoll {
        pub fn new(background: &Background) -> Self {
            let section_width = &background.width / 12.0;
            let extra_width = &background.width % 12.0;
            let inside_padding = 5.0;
            let rect_width = section_width - (inside_padding * 2.0);
            let left_padding = extra_width / 2.0;

            let mut x_positions = HashMap::new();
            x_positions.insert(
                Buttons::Left,
                PianoRollRect::new(left_padding + inside_padding),
            );
            x_positions.insert(
                Buttons::Up,
                PianoRollRect::new(x_positions[&Buttons::Left].x + section_width),
            );
            x_positions.insert(
                Buttons::Down,
                PianoRollRect::new(x_positions[&Buttons::Up].x + section_width),
            );
            x_positions.insert(
                Buttons::Right,
                PianoRollRect::new(x_positions[&Buttons::Down].x + section_width),
            );
            x_positions.insert(
                Buttons::L,
                PianoRollRect::new(x_positions[&Buttons::Right].x + section_width),
            );
            x_positions.insert(
                Buttons::Select,
                PianoRollRect::new(x_positions[&Buttons::L].x + section_width),
            );
            x_positions.insert(
                Buttons::Start,
                PianoRollRect::new(x_positions[&Buttons::Select].x + section_width),
            );
            x_positions.insert(
                Buttons::R,
                PianoRollRect::new(x_positions[&Buttons::Start].x + section_width),
            );
            x_positions.insert(
                Buttons::Y,
                PianoRollRect::new(x_positions[&Buttons::R].x + section_width),
            );
            x_positions.insert(
                Buttons::B,
                PianoRollRect::new(x_positions[&Buttons::Y].x + section_width),
            );
            x_positions.insert(
                Buttons::X,
                PianoRollRect::new(x_positions[&Buttons::B].x + section_width),
            );
            x_positions.insert(
                Buttons::A,
                PianoRollRect::new(x_positions[&Buttons::X].x + section_width),
            );

            Self {
                section_width,
                extra_width,
                inside_padding,
                rect_width,
                left_padding,
                x_positions,
            }
        }

        pub fn update(&mut self, (_, window_height): (f32, f32), events: &Vec<Buttons>) {
            for (_, position) in self.x_positions.iter_mut() {
                position.update(&window_height);
            }

            for event in events.into_iter() {
                let piano_roll_rect = self.x_positions.get_mut(event).unwrap();
                piano_roll_rect.add(&window_height, &self.rect_width)
            }
        }

        // pub fn display(self, canvas: &Canvas) {
        //     for (button, rollrects) in self.x_positions.iter() {
        //         for rect in rollrects.positions {
        //             canvas.draw(drawable, param)
        //         }
        //     }
        // }
    }

    pub struct PianoRollRect {
        x: f32,
        pub positions: Vec<Rect>,
    }

    impl PianoRollRect {
        pub fn new(x: f32) -> Self {
            Self {
                x,
                positions: Vec::new(),
            }
        }

        pub fn add(&mut self, window_height: &f32, rect_width: &f32) {
            self.positions.push(Rect {
                x: self.x,
                y: *window_height / 2.0,
                w: *rect_width,
                h: 1.0,
            })
        }

        pub fn update(&mut self, window_height: &f32) {
            for rect in self.positions.iter_mut() {
                rect.y += 1.0;
            }

            // remove Rect from Vector if y position is larger than window height
            if !self.positions.is_empty() && self.positions[0].y > *window_height {
                self.positions.remove(0);
            }
            dbg!(&self.positions);
        }

        // pub fn Display {}
    }
}
