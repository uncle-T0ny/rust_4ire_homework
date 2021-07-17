use area::calculate_area;
use common::Config;
use common::Figure;
use std::env;
use std::process;
use volume::calculate_volume;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match config.figure {
        Figure::Circle => println!(
            "{:?} area = {}",
            Figure::Circle,
            calculate_area(config)
        ),
        Figure::Rectangle => println!(
            "{:?} area = {}",
            Figure::Rectangle,
            calculate_area(config)
        ),
        Figure::Parallelepiped => println!(
            "{:?} volume = {}",
            Figure::Parallelepiped,
            calculate_volume(config)
        ),
        Figure::Sphere => println!(
            "{:?} volume = {}",
            Figure::Sphere,
            calculate_volume(config)
        ),
    }
}
