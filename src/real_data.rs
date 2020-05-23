use crate::planet::*;
use crate::vec2::*;

pub static NASA_MASS_FACTOR: f64 = 10e24; // kg
pub static NASA_RADIUS_FACTOR: f64 = 10e9; // m
pub static NASA_SPEED_FACTOR: f64 = 1000.0; // m/s
pub static MASS_OF_SUN: f64 = 1.989 * 10e30; // kg

fn real_values_to_planet(mass: f64, radius: f64, speed: f64, sun: &Planet) -> Planet {
    let loc = Vec2::new(radius, 0.0) + sun.loc();
    let vel = Vec2::new(0.0, speed);
    Planet::new(mass, loc, vel)
}

pub fn real_planets(sun: Planet) -> Vec<Planet> {
    let mercury: Planet = real_values_to_planet(
        0.33 * NASA_MASS_FACTOR,
        57.9 * NASA_RADIUS_FACTOR,
        47.4 * NASA_SPEED_FACTOR,
        &sun,
    );
    let venus: Planet = real_values_to_planet(
        4.87 * NASA_MASS_FACTOR,
        108.2 * NASA_RADIUS_FACTOR,
        35.0 * NASA_SPEED_FACTOR,
        &sun,
    );
    let earth: Planet = real_values_to_planet(
        5.97 * NASA_MASS_FACTOR,
        149.6 * NASA_RADIUS_FACTOR,
        29.8 * NASA_SPEED_FACTOR,
        &sun,
    );
    let moon: Planet = real_values_to_planet(
        0.073 * NASA_MASS_FACTOR,
        (149.6 + 0.384) * NASA_RADIUS_FACTOR,
        (29.8 + 1.0) * NASA_SPEED_FACTOR,
        &sun,
    );
    let mars: Planet = real_values_to_planet(
        0.642 * NASA_MASS_FACTOR,
        227.9 * NASA_RADIUS_FACTOR,
        24.1 * NASA_SPEED_FACTOR,
        &sun,
    );

    vec![mercury, venus, earth, moon, mars]
}
