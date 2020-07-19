use nannou::color::rgba;
#[warn(unused_assignments)]
use nannou::color::Rgba;

const COLOR_PER_SCHEME: usize = 5;

pub struct Palette {
    pub colors: [Rgba; 30],
    pub len: usize,
}

impl Palette {
    pub fn new() -> Self {
        let colors: [Rgba; 30] = [
            // LimePalette
            rgba(0.733, 1.0, 0.31, 1.0),    //lime
            rgba(1.0, 0.353, 0.208, 1.0),   //red
            rgba(0.086, 0.29, 0.8, 1.0),    //blu
            rgba(0.698, 0.188, 0.075, 1.0), //red hard
            rgba(0.098, 0.0, 0.749, 1.0),   //blu hard
            // greenAndRosePal
            rgba(1.0, 0.592, 0.706, 1.0),   //rose
            rgba(0.898, 0.341, 0.878, 1.0), //lilla
            rgba(0.902, 0.494, 0.063, 1.0), // orange
            rgba(0.0, 0.498, 0.353, 1.0),   //green
            rgba(0.098, 0.0, 0.749, 1.0),   //blu
            // nes variation
            // rgba(0.055, 0.8, 0.812, 1.0),   //azul
            // rgba(0.965, 0.914, 0.396, 1.0), //yellow
            // rgba(0.902, 0.659, 0.141, 1.0), //ocra
            // rgba(0.953, 0.447, 0.035, 1.0), //arancione
            // rgba(1.0, 0.267, 0.016, 1.0),   // red

            // primoPal
            rgba(0.055, 0.8, 0.812, 1.0),   // azul
            rgba(0.702, 0.839, 0.38, 1.0),  // verde vomito
            rgba(0.945, 0.757, 0.137, 1.0), //giallo
            rgba(0.902, 0.259, 0.141, 1.0), //ocra CHANGE
            rgba(0.525, 0.094, 0.949, 1.0), //viola
            // bigBubblePal
            rgba(0.965, 0.914, 0.396, 1.0), //yellow
            rgba(0.333, 1.0, 0.235, 1.0),   //green
            rgba(0.078, 0.643, 0.8, 1.0),   //azul
            rgba(0.706, 0.0, 0.514, 1.0),   //violetwetr
            rgba(0.0, 0.0, 0.0, 1.0),       //black
            //Primo2
            rgba(0.687, 0.592, 0.231, 1.0), //oliva
            rgba(0.098, 0.980, 0.898, 1.0), // azzurro
            rgba(0.980, 0.788, 0.0, 1.0),   //giallo
            rgba(0.96, 0.478, 0.913, 1.0),  //lilla
            rgba(0.678, 0.0, 0.607, 1.0),   // viola
            //Nes Variation 2
            rgba(0.921568627, 0.960784314, 0.337254902, 1.0), // giallo limone
            rgba(0.121568627, 0.760784314, 0.42745098, 1.0),  // verde chiaro
            rgba(1.0, 0.054901961, 0.152941176, 1.0),         // rosso carminio
            rgba(0.745098039, 0.8, 0.0, 1.0),                 // verde oliva chiaro
            rgba(0.0, 0.521568627, 0.980392157, 1.0),         // azzurro
        ];

        let len = colors.len();
        Palette {
            colors: colors,
            len,
        }
    }

    pub fn get_scheme(&self, id: usize) -> &[Rgba] {
        let mut index = id;
        if index >= (self.len / COLOR_PER_SCHEME) {
            index = 0;
        }
        index = id * COLOR_PER_SCHEME;
        let to = index + COLOR_PER_SCHEME - 1;
        &self.colors[index..=to]
    }

    pub fn get_first(&self, scheme_id: usize, offset: usize) -> Rgba {
        return self.get_scheme(scheme_id)[(0 + offset) % COLOR_PER_SCHEME];
    }
    pub fn get_second(&self, scheme_id: usize, offset: usize) -> Rgba {
        return self.get_scheme(scheme_id)[(1 + offset) % COLOR_PER_SCHEME];
    }
    pub fn get_third(&self, scheme_id: usize, offset: usize) -> Rgba {
        return self.get_scheme(scheme_id)[(2 + offset) % COLOR_PER_SCHEME];
    }
    pub fn get_fourth(&self, scheme_id: usize, offset: usize) -> Rgba {
        return self.get_scheme(scheme_id)[(3 + offset) % COLOR_PER_SCHEME];
    }
    pub fn get_fifth(&self, scheme_id: usize, offset: usize) -> Rgba {
        return self.get_scheme(scheme_id)[(4 + offset) % COLOR_PER_SCHEME];
    }
}
