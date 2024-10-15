use ndarray::{Array1, Zip};
use raylib::{
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::PhysicsObject;

pub trait Constraint {
    fn constraint(
        &mut self,
        scene_objects: &mut Vec<Box<dyn PhysicsObject>>,
        d: &RaylibDrawHandle,
    ) -> Array1<f32>;
    fn jacobian(
        &mut self,
        scene_objects: &mut Vec<Box<dyn PhysicsObject>>,
        d: &RaylibDrawHandle,
    ) -> Array1<f32>;
}

pub struct ScreenEdge {}
impl ScreenEdge {
    pub fn new() -> Self {
        Self {}
    }
}
impl Constraint for ScreenEdge {
    fn constraint(
        &mut self,
        scene_objects: &mut Vec<Box<dyn PhysicsObject>>,
        d: &RaylibDrawHandle,
    ) -> Array1<f32> {
        let mut output = Array1::<f32>::zeros(scene_objects.len() * 2);
        if !d.is_mouse_button_down(raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT) {
            for i in 0..scene_objects.len() {
                if let Some(obj) = scene_objects.get_mut(i) {
                    let position = obj.get_position() / 64_f32;
                    output[i * 2] = if position.x < 0_f32 {
                        -position.x
                    } else if position.x > 640_f32 {
                        640_f32 - position.x
                    } else {
                        0_f32
                    };
                    output[i * 2 + 1] = if position.y < 0_f32 {
                        -position.y
                    } else if position.y > 480_f32 {
                        480_f32 - position.y
                    } else {
                        0_f32
                    };
                }
            }
        }
        output
    }

    fn jacobian(
        &mut self,
        scene_objects: &mut Vec<Box<dyn PhysicsObject>>,
        d: &RaylibDrawHandle,
    ) -> Array1<f32> {
        let mut output = Array1::<f32>::zeros(scene_objects.len() * 2);
        if !d.is_mouse_button_down(raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT) {
            for i in 0..scene_objects.len() {
                if let Some(obj) = scene_objects.get_mut(i) {
                    let position = obj.get_position() / 64_f32;
                    output[i * 2] = if position.x < 0_f32 {
                        -1_f32
                    } else if position.x > 640_f32 {
                        1_f32
                    } else {
                        0_f32
                    } * 0.5_f32;
                    output[i * 2 + 1] = if position.y < 0_f32 {
                        -1_f32
                    } else if position.y > 480_f32 {
                        1_f32
                    } else {
                        0_f32
                    } * 0.5_f32;
                    // output[i * 2] = 1_f32 / output[i * 2];
                    // output[i * 2 + 1] = 1_f32 / output[i * 2 + 1];
                }
            }
            const SUB_STEPS: u32 = 10;
            let dt = 0.0167f32;
            let sub_dt: f32 = dt / SUB_STEPS as f32;
            let mut pos_error = -self.constraint(scene_objects, d);
            pos_error.map_inplace(|x| {
                *x *= 0.8_f32 / sub_dt;
            });
            // for index in 0..pos_error.len() / 2 {
            //     let normalized_pos_error =
            //         Vector2::new(pos_error[index * 2], pos_error[index * 2 + 1]).normalized();
            //     pos_error[index * 2] = normalized_pos_error.x;
            //     pos_error[index * 2 + 1] = normalized_pos_error.y;
            // }
            /*Zip::from(output.view_mut())
            .and(pos_error.view())
            .for_each(|x, y| {
                *x -= y / 100_f32;
                //x = (1_f32 / sub_dt) / *x;
                if x.is_infinite() || x.is_nan() {
                    *x = 0_f32;
                }
            });*/
        }
        output
    }
}

pub struct MouseFollow {}
impl MouseFollow {
    pub fn new() -> Self {
        Self {}
    }
}
impl Constraint for MouseFollow {
    fn constraint(
        &mut self,
        scene_objects: &mut Vec<Box<dyn PhysicsObject>>,
        d: &RaylibDrawHandle,
    ) -> Array1<f32> {
        let mut output = Array1::<f32>::zeros(scene_objects.len() * 2);
        // !d.is_mouse_button_down(raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT) {
        if true {
            for i in 0..scene_objects.len() {
                if let Some(obj) = scene_objects.get_mut(i) {
                    let position = obj.get_position() / 64_f32;
                    output[i * 2] = if position.x < 0_f32 {
                        -position.x
                    } else if position.x > 640_f32 {
                        640_f32 - position.x
                    } else {
                        0_f32
                    };
                    output[i * 2 + 1] = if position.y < 0_f32 {
                        -position.y
                    } else if position.y > 480_f32 {
                        480_f32 - position.y
                    } else {
                        0_f32
                    };
                }
            }
        }
        output
    }

    fn jacobian(
        &mut self,
        scene_objects: &mut Vec<Box<dyn PhysicsObject>>,
        d: &RaylibDrawHandle,
    ) -> Array1<f32> {
        let mut output = Array1::<f32>::zeros(scene_objects.len() * 2);
        if !d.is_mouse_button_down(raylib::prelude::MouseButton::MOUSE_BUTTON_LEFT) {
            for i in 0..scene_objects.len() {
                if let Some(obj) = scene_objects.get_mut(i) {
                    let position = obj.get_position() / 64_f32;
                    /*output[i * 2] = if position.x < 0_f32 {
                        -1_f32
                    } else if position.x > 640_f32 {
                        1_f32
                    } else {
                        0_f32
                    } * 0.5_f32;*/
                    output[i * 2 + 1] = if position.y < 0_f32 {
                        -1_f32
                    } /*else if position.y > 480_f32 {
                        1_f32
                    } */else {
                        0_f32
                    } * 0.5_f32;
                    // output[i * 2] = 1_f32 / output[i * 2];
                    // output[i * 2 + 1] = 1_f32 / output[i * 2 + 1];
                }
            }
            const SUB_STEPS: u32 = 10;
            let dt = 0.0167f32;
            let sub_dt: f32 = dt / SUB_STEPS as f32;
            let mut pos_error = -self.constraint(scene_objects, d);
            pos_error.map_inplace(|x| {
                *x *= 0.8_f32 / sub_dt;
            });
            // for index in 0..pos_error.len() / 2 {
            //     let normalized_pos_error =
            //         Vector2::new(pos_error[index * 2], pos_error[index * 2 + 1]).normalized();
            //     pos_error[index * 2] = normalized_pos_error.x;
            //     pos_error[index * 2 + 1] = normalized_pos_error.y;
            // }
            /*Zip::from(output.view_mut())
            .and(pos_error.view())
            .for_each(|x, y| {
                *x -= y / 100_f32;
                //x = (1_f32 / sub_dt) / *x;
                if x.is_infinite() || x.is_nan() {
                    *x = 0_f32;
                }
            });*/
        }
        output
    }
}
