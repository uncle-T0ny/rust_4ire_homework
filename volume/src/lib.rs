use common::{Config, Figure};
use core::f64::consts::PI;

// Sphere 4/3 * Pi * R^3
fn sphere_volume(radius: f64) -> f64 {
    if radius < 0.0 {
        0.0
    } else {
        4.0 / 3.0 * PI * radius.powi(3)
    }
}

// Parallelepiped length * width * height
fn parallelepiped_volume(length: f64, width: f64, height: f64) -> f64 {
    if length < 0.0 || width < 0.0 || height < 0.0 {
        0.0
    } else {
        length * width * height
    }
}

pub fn calculate_volume(config: Config) -> f64 {
    let result = match config.figure {
        Figure::Sphere => sphere_volume(config.radius.unwrap()),
        Figure::Parallelepiped => parallelepiped_volume(
            config.length.unwrap(),
            config.width.unwrap(),
            config.height.unwrap(),
        ),
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

    fn assert_eq(val1: f64, val2: f64) {
        assert_eq!(round_to_four_digits(val1), round_to_four_digits(val2))
    }

    #[test]
    fn parallelepiped_volume_test() {
        assert_eq(16.0, parallelepiped_volume(2.0, 4.0, 2.0));
        assert_eq(32.0, parallelepiped_volume(2.0, 4.0, 4.0));
        assert_eq(0.0, parallelepiped_volume(0.0, 0.0, 0.0));
        assert_eq(0.0, parallelepiped_volume(-1.0, 0.0, 0.0));
    }

    #[test]
    fn sphere_volume_test() {
        assert_eq(113.0973, sphere_volume(3.0));
        assert_eq(129.6813, sphere_volume(3.14));
        assert_eq(0.0, sphere_volume(0.0));
        assert_eq(0.0, sphere_volume(-1.0));
    }

    #[test]
    fn calculate_volume_test() {
        // test Sphere
        assert_eq(113.0973, calculate_volume(Config {
            figure: Figure::Sphere,
            radius: Some(3.0),
            length: None,
            width: None,
            height: None,
        }));

        // test Parallelepiped
        assert_eq(16.0, calculate_volume(Config {
            figure: Figure::Parallelepiped,
            radius: None,
            length: Some(2.0),
            width: Some(4.0),
            height: Some(2.0),
        }));
    }
}
