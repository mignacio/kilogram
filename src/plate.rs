#[derive(Clone, Copy)]
pub enum PlateColor{
    Red,
    Blue,
    Yellow,
    Green,
    White
}

impl PlateColor{
    pub fn to_string(&self) -> String{
        match self{
            PlateColor::Red=>return "Red".to_string(),
            PlateColor::Blue=>return "Blue".to_string(),
            PlateColor::Yellow=>return "Yellow".to_string(),
            PlateColor::Green=>return "Green".to_string(),
            PlateColor::White=>return "White".to_string()
        }
    }

    pub fn to_ansi_code(&self) -> String{
        match self{
            PlateColor::Red=>return "1".to_string(),
            PlateColor::Blue=>return "4".to_string(),
            PlateColor::Yellow=>return "3".to_string(),
            PlateColor::Green=>return "2".to_string(),
            PlateColor::White=>return "7".to_string()
        }
    }
}

pub struct Plate{
    weight: f32,
    color: PlateColor,
    pub quantity: i32

}

impl Plate{
    pub fn new(weight: f32, color: PlateColor, quantity: i32) -> Self{
        Self{
            weight,
            color,
            quantity
        }
    }

    pub fn to_string(&self) -> String{
        return format!("\x1b[4{}m\x1b[30m{} {}kg\t{}\x1b[0m", self.color.to_ansi_code(), self.quantity, self.weight, self.color.to_string());
    }

    pub fn get_weight(&self) -> f32{
        self.weight
    }

    pub fn get_color(&self) -> PlateColor {
        self.color
    }
}