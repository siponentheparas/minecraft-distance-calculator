use std::{io};
use clearscreen::clear;

fn main() {
    clear().expect("error");
    'start: loop {
        println!("Remember, Y is optional");
        println!("Coordinates (X Y Z): ");
        let mut coordinates = String::new();
        io::stdin().read_line(&mut coordinates).expect("error");
        let coord_vec_string: Vec<&str> = coordinates.split(" ").collect();
        let mut coord_vec: Vec<f32> = Vec::new();

        if coordinates.contains("exit") {
            clear().expect("error");
            std::process::exit(0);
        }

        for coord in coord_vec_string {
            let coord: f32 = match coord.trim().parse() {
                Ok(number) => number,
                Err(_error) => {
                    clear().expect("error");
                    println!("Please input only the coordinates seperated with spaces!");
                    continue 'start;
                } 
            };

            coord_vec.push(coord);
        }

        if coord_vec.len() < 2 {
            println!("Please input at least X and Z coordinates!");
        }

        println!("Coordinates 2 (X Y Z): ");
        let mut coordinates_2 = String::new();
        io::stdin().read_line(&mut coordinates_2).expect("error");
        let coord_vec_string_2: Vec<&str> = coordinates_2.split(" ").collect();
        let mut coord_vec_2: Vec<f32> = Vec::new();

        if coordinates_2.contains("exit") {
            clear().expect("error");
            std::process::exit(0);
        }

        for coord_2 in coord_vec_string_2 {
            let coord_2: f32 = match coord_2.trim().parse() {
                Ok(number) => number,
                Err(_error) => {
                    clear().expect("error");
                    println!("Please input only the coordinates seperated with spaces!");
                    continue 'start;
                } 
            };

            coord_vec_2.push(coord_2);
        }

        if coord_vec_2.len() < 2 {
            println!("Please input at least X and Z coordinates!");
        }

        let mut distance: f32 = 0.0;

        if coord_vec.len() == 2 {
            if coord_vec_2.len() == 2 {
                if coord_vec[0] > coord_vec_2[0] {
                    distance = distance + (coord_vec[0] - coord_vec_2[0]);
                } else {
                    distance = distance + (coord_vec_2[0] - coord_vec[0]);
                }
    
                if coord_vec[1] > coord_vec_2[1] {
                    distance = distance + (coord_vec[1] - coord_vec_2[1]);
                } else {
                    distance = distance + (coord_vec_2[1] - coord_vec[1]);
                }
            } else {
                if coord_vec[0] > coord_vec_2[0] {
                    distance = distance + (coord_vec[0] - coord_vec_2[0]);
                } else {
                    distance = distance + (coord_vec_2[0] - coord_vec[0]);
                }
    
                if coord_vec[1] > coord_vec_2[2] {
                    distance = distance + (coord_vec[1] - coord_vec_2[2]);
                } else {
                    distance = distance + (coord_vec_2[2] - coord_vec[1]);
                }
            }
        } else {
            if coord_vec_2.len() == 2 {
                if coord_vec[0] > coord_vec_2[0] {
                    distance = distance + (coord_vec[0] - coord_vec_2[0]);
                } else {
                    distance = distance + (coord_vec_2[0] - coord_vec[0]);
                }
    
                if coord_vec[2] > coord_vec_2[1] {
                    distance = distance + (coord_vec[2] - coord_vec_2[1]);
                } else {
                    distance = distance + (coord_vec_2[1] - coord_vec[2]);
                }
            } else {
                if coord_vec[0] > coord_vec_2[0] {
                    distance = distance + (coord_vec[0] - coord_vec_2[0]);
                } else {
                    distance = distance + (coord_vec_2[0] - coord_vec[0]);
                }
    
                if coord_vec[2] > coord_vec_2[2] {
                    distance = distance + (coord_vec[2] - coord_vec_2[2]);
                } else {
                    distance = distance + (coord_vec_2[2] - coord_vec[2]);
                }
            }
        }

        clear().expect("error");

        println!("Distance between coords 1 and coords 2 is: {distance}");
    }
}
