#[derive(Clone, Copy)]
pub enum PlateColor{
    Red,
    Blue,
    Yellow,
    Green,
    White
}

impl PlateColor{
    pub fn to_ansi_code(&self) -> &str{
        match self{
            PlateColor::Red=>return "1",
            PlateColor::Blue=>return "4",
            PlateColor::Yellow=>return "3",
            PlateColor::Green=>return "2",
            PlateColor::White=>return "7"
        }
    }
}

#[derive(Clone, Copy)]
pub struct Plate{
    weight: f32,
    color: PlateColor,
    pub quantity: i32,
    ascii_art: AsciiPlate

}

impl Plate{
    pub fn new(weight: f32, color: PlateColor, quantity: i32, ascii_art: AsciiPlate) -> Self{
        Self{
            weight,
            color,
            quantity,
            ascii_art
        }
    }

    pub fn get_ascii_art(&self) -> AsciiPlate{
        return self.ascii_art
    }
/*
    pub fn to_string(&self) -> String{
        return format!("\x1b[10{}m\x1b[30m{} {}kg\t{}\x1b[0m", self.color.to_ansi_code(), self.quantity, self.weight, self.color.to_string());
    }
*/
    pub fn get_weight(&self) -> f32{
        self.weight
    }

    pub fn get_color(&self) -> PlateColor {
        self.color
    }
}

#[derive(Clone, Copy)]
pub enum AsciiPlate{
    Plate1,
    Plate2,
    Plate3,
    Plate4,
    Plate5,
    Plate6
}

impl AsciiPlate{
    pub fn to_ascii_arr(&self) -> &[&str; 7] {
        match self{
            AsciiPlate::Plate1 => return ASCII_PLATE1,
            AsciiPlate::Plate2 => return ASCII_PLATE2,
            AsciiPlate::Plate3 => return ASCII_PLATE3,
            AsciiPlate::Plate4 => return ASCII_PLATE4,
            AsciiPlate::Plate5 => return ASCII_PLATE5,
            AsciiPlate::Plate6 => return ASCII_PLATE6,
        }
    }
}
// Used for 25kg red plate.
const ASCII_PLATE1: &[&str; 7] = &[
    "|¦|",
    "|¦|",
    "|¦|",
    "|¦|",
    "|¦|",
    "|¦|",
    "|¦|",
];

//Used for 20 kg blue plate.
const ASCII_PLATE2: &[&str; 7] = &[
    "||",
    "||",
    "||",
    "||",
    "||",
    "||",
    "||",
];

//Used for 15 and 10kg yellow and green plates.
const ASCII_PLATE3: &[&str; 7] = &[
    "|",
    "|",
    "|",
    "|",
    "|",
    "|",
    "|",
];

//Used for 5 and 2.5kg white and red plates.
const ASCII_PLATE4: &[&str; 7] = &[
    " ",
    "|",
    "|",
    "|",
    "|",
    "|",
    " ",
];

//Used for 2 and 1.5kg blue and yellow plates.
const ASCII_PLATE5: &[&str; 7] = &[
    " ",
    " ",
    "|",
    "|",
    "|",
    " ",
    " ",
];

//Used for 1 and 0.5kg green and white plates.
const ASCII_PLATE6: &[&str; 7] = &[
    " ",
    " ",
    " ",
    "|",
    " ",
    " ",
    " ",
];
