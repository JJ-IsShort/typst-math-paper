use std::f32;

use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub trait PhysicsObject {
    fn get_mass(&self) -> f32 {
        1_f32
    }
    fn get_velocity(&self) -> Vector2;
    fn get_acceleration(&self) -> Vector2;
    fn get_position(&self) -> Vector2;
    fn get_old_position(&self) -> Vector2;

    fn get_acceleration_mut(&mut self) -> &mut Vector2;
    fn get_position_mut(&mut self) -> &mut Vector2;
    fn get_old_position_mut(&mut self) -> &mut Vector2;

    fn set_mass(&mut self, mass: f32);
    fn set_acceleration(&mut self, acceleration: Vector2);
    fn set_position(&mut self, position: Vector2);
    fn set_old_position(&mut self, old_position: Vector2);

    fn draw(&self, d: &mut RaylibDrawHandle);

    fn update(&mut self, dt: f32);
    fn accelerate(&mut self, acc: Vector2);
}

#[derive(Debug)]
pub struct Circle {
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
    pub position: Vector2,
    pub old_position: Vector2,
    pub acceleration: Vector2,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            radius: 5_f32 * 64_f32,
            mass: 1_f32,
            color: Color::RED,
            position: Vector2::new(640_f32 / 2_f32, 480_f32 / 2_f32) * 64_f32,
            old_position: Vector2::new(640_f32 / 2_f32, 480_f32 / 2_f32) * 64_f32,
            acceleration: Vector2::zero(),
        }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsObject for Circle {
    fn get_mass(&self) -> f32 {
        self.mass
    }
    fn get_velocity(&self) -> Vector2 {
        self.position - self.old_position
    }
    fn get_acceleration(&self) -> Vector2 {
        self.acceleration
    }
    fn get_position(&self) -> Vector2 {
        self.position
    }
    fn get_old_position(&self) -> Vector2 {
        self.old_position
    }

    fn get_acceleration_mut(&mut self) -> &mut Vector2 {
        &mut self.acceleration
    }
    fn get_position_mut(&mut self) -> &mut Vector2 {
        &mut self.position
    }
    fn get_old_position_mut(&mut self) -> &mut Vector2 {
        &mut self.old_position
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle_v(
            self.get_position() / 64_f32,
            self.radius / 64_f32,
            self.color,
        );
    }

    fn update(&mut self, dt: f32) {
        let velocity: Vector2 = self.get_position() * 100_f32 - self.get_old_position() * 100_f32;

        self.old_position = self.position;

        self.position = self.get_position() + (velocity / 100_f32) + self.acceleration * dt * dt;

        self.acceleration = Vector2::zero();
    }

    fn accelerate(&mut self, acc: Vector2) {
        self.acceleration += acc;
    }

    fn set_mass(&mut self, mass: f32) {
        self.mass = mass;
    }

    fn set_acceleration(&mut self, acceleration: Vector2) {
        self.acceleration = acceleration;
    }

    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn set_old_position(&mut self, old_position: Vector2) {
        self.old_position = old_position;
    }
}
