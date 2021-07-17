use common::{Config, Figure};
use core::f64::consts::PI;

// Rectangle, x * y
fn rectangle_area(length: f64, width: f64) -> f64 {
    if length < 0.0 || width < 0.0 {
        0.0
    } else {
        length * width
    }
}

// Circle, Pi * R^2
fn circle_area(radius: f64) -> f64 {
    if radius < 0_f64 {
        0.0_f64
    } else {
        radius.powf(2_f64) * PI
    }
}

pub fn calculate_area(config: Config) -> f64 {
    let result = match config.figure {
        Figure::Circle => circle_area(config.radius.unwrap()),
        Figure::Rectangle => rectangle_area(config.length.unwrap(), config.width.unwrap()),
        _ => 0.0,
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_to_four_digits(val: f64) -> f64 {
        f64::trunc(val * 10000.00) / 10000.0
    }

    #[test]
    fn rectngle_area_test() {
        assert_eq!(4.0, rectangle_area(2.0, 2.0));
        assert_eq!(10.1, rectangle_area(1.01, 10.0));
        assert_eq!(0.0, rectangle_area(2.0, 0.0));
        assert_eq!(0.0, rectangle_area(0.0, 2.0));
        assert_eq!(0.0, rectangle_area(0.0, 0.0));
        assert_eq!(0.0, rectangle_area(2.0, -1.0));
    }

    #[test]
    fn circle_area_test() {
        assert_eq!(12.5663, round_to_four_digits(circle_area(2.0)));
        assert_eq!(0.0, circle_area(0.0));
        assert_eq!(0.0, circle_area(-1.0));
    }

    #[test]
    fn calculate_area_test() {
        assert_eq!(
            12.5663,
            round_to_four_digits(calculate_area(Config {
                figure: Figure::Circle,
                radius: Some(2.0),
                length: None,
                width: None,
                height: None
            }))
        );

        assert_eq!(
            8.0,
            calculate_area(Config {
                figure: Figure::Rectangle,
                radius: None,
                length: Some(2.0),
                width: Some(4.0),
                height: None
            })
        );
    }
}
