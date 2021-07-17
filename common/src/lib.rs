use std::{env, process, str::FromStr};
use strum_macros::EnumString;

#[derive(EnumString, PartialEq, Debug)]
pub enum Figure {
    #[strum(ascii_case_insensitive)]
    Circle,
    #[strum(ascii_case_insensitive)]
    Rectangle,
    #[strum(ascii_case_insensitive)]
    Sphere,
    #[strum(ascii_case_insensitive)]
    Parallelepiped,
}

pub struct Config {
    pub figure: Figure,
    pub radius: Option<f64>,
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let figure = match args.next() {
            Some(arg) => Figure::from_str(&arg).unwrap_or_else(|_| {
                eprintln!("Can't recognize figure type");
                process::exit(1);
            }),
            None => return Err("First argument not provided, expected figure type"),
        };

        match figure {
            Figure::Circle | Figure::Sphere => {
                let radius: f64 = match args.next() {
                    Some(arg) => f64::from_str(&arg).unwrap_or_else(|_| {
                        eprintln!("Can't parse radius");
                        process::exit(1);
                    }),
                    None => return Err("Please provide circle radius as an argument"),
                };
                return Ok(Config {
                    figure,
                    radius: Some(radius),
                    length: None,
                    width: None,
                    height: None,
                });
            }
            Figure::Parallelepiped => {
                let length: f64 = match args.next() {
                    Some(arg) => f64::from_str(&arg).unwrap_or_else(|_| {
                        eprintln!("Can't parse length");
                        process::exit(1);
                    }),
                    None => return Err("Please provide parallelepiped length as first"),
                };
                let width: f64 = match args.next() {
                    Some(arg) => f64::from_str(&arg).unwrap_or_else(|_| {
                        eprintln!("Can't parse width");
                        process::exit(1);
                    }),
                    None => return Err("Please provide parallelepiped width as second argument"),
                };
                let height: f64 = match args.next() {
                    Some(arg) => f64::from_str(&arg).unwrap_or_else(|_| {
                        eprintln!("Can't parse height");
                        process::exit(1);
                    }),
                    None => return Err("Please provide parallelepiped height as third argument"),
                };
                return Ok(Config {
                    figure,
                    radius: None,
                    length: Some(length),
                    width: Some(width),
                    height: Some(height),
                });
            }
            Figure::Rectangle => {
                let length: f64 = match args.next() {
                    Some(arg) => f64::from_str(&arg).unwrap_or_else(|_| {
                        eprintln!("Can't parse length");
                        process::exit(1);
                    }),
                    None => return Err("First param length not provided"),
                };
                let width: f64 = match args.next() {
                    Some(arg) => f64::from_str(&arg).unwrap_or_else(|_| {
                        eprintln!("Can't parse width");
                        process::exit(1);
                    }),
                    None => return Err("Second param width not provided"),
                };
                return Ok(Config {
                    figure,
                    radius: None,
                    length: Some(length),
                    width: Some(width),
                    height: None,
                });
            }
        }
    }
}
