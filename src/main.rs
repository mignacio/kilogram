use std::env;

mod plate;

use crate::plate::*;

fn main() {
    let plate_stack: [Plate; 10] = [
        Plate::new(25.0, PlateColor::Red,    0, AsciiPlate::Plate1),
        Plate::new(20.0, PlateColor::Blue,   0, AsciiPlate::Plate2),
        Plate::new(15.0, PlateColor::Yellow, 0, AsciiPlate::Plate3),
        Plate::new(10.0, PlateColor::Green,  0, AsciiPlate::Plate3),
        Plate::new(5.0, PlateColor::White,   0, AsciiPlate::Plate4),
        Plate::new(2.5, PlateColor::Red,     0, AsciiPlate::Plate4),
        Plate::new(2.0, PlateColor::Blue,    0, AsciiPlate::Plate5),
        Plate::new(1.5, PlateColor::Yellow,  0, AsciiPlate::Plate5),
        Plate::new(1.0, PlateColor::Green,   0, AsciiPlate::Plate6),
        Plate::new(0.5, PlateColor::White,   0, AsciiPlate::Plate6)
    ];

    let args: Vec<String> = env::args().collect();
    let mut total_weight: f32;
    let bar_weight: f32;

    match args.len() {
        2 => {
            match args[1].parse::<f32>(){
                Ok(n) => {
                    total_weight = n;
                    bar_weight = 20.0; //assume bar is 20kg
                },
                Err(_e) =>{
                    println!("Weight must be a valid number \n\te:({}).", _e);
                    return
                },
            }
        },
        3 => {
            match args[1].parse::<f32>(){
                Ok(n) => total_weight = n,
                Err(_e) =>{
                    println!("Weight must be a valid number \n\te:({}).", _e);
                    return
                },
            }
            match args[2].parse::<char>(){
                Ok('m') => bar_weight = 20.0,
                Ok('w') => bar_weight = 15.0,
                _ => {
                    println!("Bar must be a 'm' or 'w' character.");
                    return
                },
            }
        }
        _ => {
            println!("Usage: pass a weight as argument to know which plates to load on a weighlifting bar.");
            println!("- Example 1: weightlifting-calculator 145 w (shows how to load a women's 15kg bar with a total of 145kg).");
            println!("- Example 2: weightlifting-calculator 132 m (shows how to load a men's 20kg bar with a total of 132kg)");
            return
        },
    };

    if !(total_weight >= (bar_weight + 5.0)) {
        println!("Weight must be greater or equal than the weight of the bar with collars ({})kg).", bar_weight + 5.0);
        return
    }

    total_weight = (total_weight - bar_weight) / 2.0;

    let has_collar: bool = true;
    if has_collar {
        total_weight -= 2.5;
    }

    //Vector that represents how the bar is loaded with plates.
    //The first element is the plate closest to the bar.
    let mut ascii_plates: Vec<Plate> = Vec::new();

    for mut plate in plate_stack {
        plate.quantity = (total_weight / plate.get_weight()).floor() as i32;
        total_weight -= plate.quantity as f32 * plate.get_weight();
        for _n in 0..plate.quantity{
            ascii_plates.push(plate);
        }
    }

    const BAR: &[&str; 7] = &[
        "                  ",
        "                  ",
        "                  ",
        "=————————————————=",
        "                  ",
        "                  ",
        "                  ",
    ];

    const BAR_END: &[&str; 7] = &[
        " ",
        " ",
        " ",
        "=",
        " ",
        " ",
        " ",
    ];

    const COLLAR: &[&str; 7] = &[
        " ",
        " ",
        " ",
        "Ò",
        " ",
        " ",
        " ",
    ];



    for n in 0..7{
        print!("{}", BAR_END[n]);
        print!("{}", BAR_END[n]);
        print!("{}", COLLAR[n]);
        for ascii_plate in ascii_plates.iter().rev(){
            print!("\x1b[9{}m{}\x1b[0m", ascii_plate.get_color().to_ansi_code(), ascii_plate.get_ascii_art().to_ascii_arr()[n]);
        }

        print!("{}", BAR[n]);

        for ascii_plate in ascii_plates.iter(){
            print!("\x1b[9{}m{}\x1b[0m", ascii_plate.get_color().to_ansi_code(), ascii_plate.get_ascii_art().to_ascii_arr()[n]);
        }

        print!("{}", COLLAR[n]);
        print!("{}", BAR_END[n]);
        print!("{}", BAR_END[n]);
        println!();
    }
} 
