use dagt_core::{Constraints, Draw, EventHandler, StateChanged, Widget, WidgetBuilder};
use dagt_fonts::Font;
use dagt_platform::desktop::event::GlobalEvent;
use dagt_primitives::glyph::Glyph;

#[derive(Clone)]
pub struct Text {
    text: String,
    constraints: Option<Constraints>,
    font: Font,
    font_size: f64,
    line_height: f64,
    space_size_em: f64,
    line_height_em: f64,
    font_size_em: f64,
    letter_spacing_em: f64,
    word_spacing_em: f64,
    line_spacing_em: f64,
    scale: f64,
}

impl Text {
    pub fn new() -> Text {
        let font = Font::load("/usr/share/fonts/TTF/JetBrainsMonoNerdFontMono-Bold.ttf");
        let font_size = 26.0; // pt
        let line_height = font_size * 96.0 / 72.0;

        let space_size_em = 0.333;
        let line_height_em = 1.3;
        let font_size_em = 1.0;
        let letter_spacing_em = 1.0;
        let word_spacing_em = 1.0;
        let line_spacing_em = 1.0;
        let scale = 1.0 / font.units_per_em() as f64;

        Text {
            text: String::new(),
            constraints: None,
            font,
            font_size,
            line_height,
            space_size_em,
            line_height_em,
            font_size_em,
            letter_spacing_em,
            word_spacing_em,
            line_spacing_em,
            scale,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_owned();
        self
    }

    fn calc_constr(mut self) -> Self {
        let scale = self.scale;
        let line_height = self.line_height;

        let mut letter_advance = 0.0;
        let mut word_advance = 0.0;
        let mut line_advance = 0.0;

        let mut max_x = 0;
        let mut max_y = 0;

        for (i, c) in self.text.chars().enumerate() {
            match c {
                ' ' => {
                    word_advance += self.space_size_em;
                }
                '\n' => {
                    line_advance += self.line_height_em;
                    word_advance = 0.0;
                    letter_advance = 0.0;
                }
                _ => {
                    let glyph = self.font.get_glyph(c);

                    let offset_x = glyph.min_x as f64 * scale;
                    let offset_y = glyph.min_y as f64 * scale;

                    if i != self.text.len() - 1 {
                        letter_advance += glyph.advance_width as f64 * scale;
                    }

                    let width = glyph.width as f64 * scale;
                    let height = glyph.height as f64 * scale;

                    let x_advance = (letter_advance * self.letter_spacing_em
                        + word_advance * self.word_spacing_em
                        + offset_x)
                        * self.font_size_em;
                    let y_advance =
                        (line_advance * self.line_spacing_em + offset_y) * self.font_size_em;

                    let height = (line_height * height) as i32;
                    let width = (line_height * width) as i32;

                    max_x = max_x.max((line_height * x_advance) as i32 + width);
                    max_y = max_y.max((line_height * y_advance) as i32 + height);
                }
            }
        }

        self.constraints = Some(Constraints {
            width: max_x,
            height: max_y,
            ..Default::default()
        });

        self
    }
}

impl WidgetBuilder<GlobalEvent> for Text {
    fn build(&self) -> Box<dyn dagt_core::Widget<GlobalEvent>> {
        Box::new(self.clone().calc_constr())
    }
}

impl StateChanged for Text {}

impl Widget<GlobalEvent> for Text {
    fn constraints(&self) -> dagt_core::Constraints {
        self.constraints.unwrap_or_default()
    }
}

impl EventHandler<GlobalEvent> for Text {}

impl Draw for Text {
    fn draw(&mut self, constraints: Constraints) -> bool {
        let scale = self.scale;
        let line_height = self.line_height;

        let mut letter_advance = 0.0;
        let mut word_advance = 0.0;
        let mut line_advance = 0.0;

        for (i, c) in self.text.chars().enumerate() {
            match c {
                ' ' => {
                    word_advance += self.space_size_em;
                }
                '\n' => {
                    line_advance += self.line_height_em;
                    word_advance = 0.0;
                    letter_advance = 0.0;
                }
                _ => {
                    let glyph = self.font.get_glyph(c).clone();

                    let offset_x = glyph.min_x as f64 * scale;
                    let offset_y = glyph.min_y as f64 * scale;

                    let width = glyph.width as f64 * scale;
                    let height = glyph.height as f64 * scale;
                    // println!("{c}: width = {width}; height = {height}");

                    let x_advance = (letter_advance * self.letter_spacing_em
                        + word_advance * self.word_spacing_em
                        + offset_x)
                        * self.font_size_em;
                    let y_advance =
                        (line_advance * self.line_spacing_em + offset_y) * self.font_size_em;

                    letter_advance += glyph.advance_width as f64 * scale;

                    // println!("{c}: x_advance = {x_advance}; y_advance = {y_advance}");

                    let constr = Constraints {
                        height: (line_height * height) as i32,
                        width: (line_height * width) as i32,
                        x: (line_height * x_advance) as i32 + constraints.x,
                        y: (line_height * y_advance) as i32 + constraints.y,
                        ..Default::default()
                    };

                    if c == 'T' {
                        println!("constr = {:#?}", constr);
                        println!("offset_x = {offset_x}");
                    }

                    // println!("{c}: width = {width}; height = {height}");

                    // Rect {
                    //     bg_color: Color::black(),
                    //     ..Default::default()
                    // }
                    // .draw(constr);

                    Glyph::new(glyph).draw(constr);
                }
            }
        }

        true
    }
}
