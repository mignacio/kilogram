use std::env;


enum PlateColor{
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
}

struct Plate{
    pub weight: f32,
    pub color: PlateColor,
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
        return format!("{} {}kg\t{}", self.quantity, self.weight, self.color.to_string());
    }
}


fn main() {
    let plate_stack: [Plate; 10] = [
        Plate::new(25.0, PlateColor::Red,    0),
        Plate::new(20.0, PlateColor::Blue,   0),
        Plate::new(15.0, PlateColor::Yellow, 0),
        Plate::new(10.0, PlateColor::Green,  0),
        Plate::new(5.0, PlateColor::White,   0),
        Plate::new(2.5, PlateColor::Red,     0),
        Plate::new(2.0, PlateColor::Blue,    0),
        Plate::new(1.5, PlateColor::Yellow,  0),
        Plate::new(1.0, PlateColor::Green,   0),
        Plate::new(0.5, PlateColor::White,   0)
    ];

    let bar_weight: f32 = 20.0;
    let mut total_weight: f32 = 148.0;
    
    total_weight = (total_weight - bar_weight) / 2.0;

    let has_collar: bool = true;
    if has_collar {
        total_weight -= 2.5;
    }

    for mut plate in plate_stack {
        plate.quantity = (total_weight / plate.weight).floor() as i32;
        total_weight -= plate.quantity as f32 * plate.weight; 
        println!("{}", plate.to_string());
    }
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
}
