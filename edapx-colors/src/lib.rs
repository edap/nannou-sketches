#[warn(unused_assignments)]
use nannou::color::Rgb;
use nannou::color::rgb;

const COLOR_PER_SCHEME: usize = 5;

pub struct Palette {
    pub colors: [Rgb; 30],
    pub len: usize,
}

impl Palette {
    pub fn new() -> Self {
        let colors_rgb: [Rgb; 30] = [
            // LimePalette
            rgb(0.733, 1.0, 0.31), //lime
            rgb(1.0, 0.353, 0.208), //red
            rgb(0.086, 0.29, 0.8), //blu
            rgb(0.698, 0.188, 0.075), //red hard
            rgb( 0.098, 0.0, 0.749), //blu hard
            // greenAndRosePal
            rgb(1.0, 0.592, 0.706), //rose
            rgb(0.898, 0.341, 0.878), //lilla
            rgb(0.902, 0.494, 0.063), // orange
            rgb(0.0, 0.498, 0.353), //green
            rgb( 0.098, 0.0, 0.749),//blu

            // nes variation
            rgb(0.055, 0.8, 0.812), //azul
            rgb(0.965, 0.914, 0.396), //yellow
            rgb(0.902, 0.659, 0.141), //ocra
            rgb(0.953, 0.447, 0.035), //arancione
            rgb(1.0, 0.267, 0.016), // red
            // primoPal
            rgb(0.055, 0.8, 0.812), // azul
            rgb(0.702, 0.839, 0.38),// verde vomito
            rgb(0.945, 0.757, 0.137), //giallo
            rgb(0.902, 0.659, 0.141), //ocra
            rgb(0.525, 0.094, 0.949), //viola
            // bigBubblePal
            rgb(0.965, 0.914, 0.396), //yellow
            rgb(0.333, 1.0, 0.235), //green
            rgb(0.078, 0.643, 0.8), //azul
            rgb(0.706, 0.0, 0.514), //violetwetr
            rgb(0.0, 0.0, 0.0), //black

            //Primo2
            rgb(0.687, 0.592, 0.231), //oliva
            rgb(0.098, 0.980, 0.898), // azzurro
            rgb(0.980, 0.788, 0.0), //giallo
            rgb(0.96, 0.478, 0.913), //lilla
            rgb(0.678, 0.0, 0.607), // viola
        ];

        let len = colors_rgb.len();
        Palette {
            colors: colors_rgb,
            len,
        }
    }

    pub fn get_scheme(&self, id: usize) -> &[Rgb] {
        let mut index = id;
        if index >= (self.len / COLOR_PER_SCHEME) {
            index = 0;
        }
        index = id * COLOR_PER_SCHEME;
        let to = index + COLOR_PER_SCHEME -1;
        &self.colors[index..=to]
    }
}