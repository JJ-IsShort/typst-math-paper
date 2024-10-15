use crate::constraints::*;

use crate::objects::*;
use ffi::Rectangle;
use ndarray::Array1;
use ndarray::Array2;

use ndarray::ArrayView1;
use ndarray::Axis;
use ndarray::Ix1;
use ndarray::Ix2;
use ndarray_linalg::solve::Inverse;
use raylib::prelude::*;

pub mod constraints;
pub mod objects;

struct Solver {
    scene_objects: Vec<Box<dyn PhysicsObject>>,
    constraints: Vec<Box<dyn Constraint>>,

    previous_force: Array1<f32>,
}

impl Solver {
    fn new() -> Self {
        Self {
            scene_objects: Vec::with_capacity(100),
            constraints: Vec::with_capacity(100),
            previous_force: Array1::<f32>::zeros(Ix1(0)),
        }
    }

    fn apply_gravity(&mut self) {
        for ele in self.scene_objects.iter_mut() {
            ele.accelerate(Vector2::new(0_f32, 981_f32) * 64_f32);
        }
    }

    fn apply_air_resistance(&mut self) {
        for ele in self.scene_objects.iter_mut() {
            let mut velocity = ele.get_velocity();
            velocity *= 0.001_f32;
            ele.get_old_position_mut().x += velocity.x;
            ele.get_old_position_mut().y += velocity.y;
        }
    }

    fn solve_constraints(&mut self, d: &RaylibDrawHandle) {
        let mut J = Array2::<f32>::zeros(Ix2(self.constraints.len(), self.scene_objects.len() * 2));
        for constraint_index in 0..self.constraints.len() {
            let mut constraint = self.constraints.get_mut(constraint_index).unwrap();
            let constraint_jacobian = constraint.jacobian(&mut self.scene_objects, d);
            // let mut J_slice = J.slice_axis_mut(
            //     Axis(1),
            //     ndarray::Slice::new(
            //         constraint_index.try_into().unwrap(),
            //         Some(constraint_index.try_into().unwrap()),
            //         1,
            //     ),
            // );
            for J_index in 0..(self.scene_objects.len() * 2) {
                J[(constraint_index, J_index)] = constraint_jacobian[J_index];
            }
        }
        let J_clone = J.clone();
        let mut JT = J_clone.t();

        let mut M = Array2::<f32>::zeros(Ix2(
            2 * self.scene_objects.len(),
            2 * self.scene_objects.len(),
        ) as ndarray::Dim<[usize; 2]>);
        for scene_index in 0..self.scene_objects.len() {
            M[(scene_index * 2, scene_index * 2)] = self.scene_objects[scene_index].get_mass();
            M[(scene_index * 2 + 1, scene_index * 2 + 1)] =
                self.scene_objects[scene_index].get_mass();
        }
        let mut W = Inverse::inv(&M).expect("Invalid mass matrix. Couldn't be invverted.");

        let mut Q = Array1::<f32>::from_shape_fn((self.scene_objects.len() * 2), |i| {
            let axis_selector = i % 2;
            match axis_selector {
                0_usize => {
                    self.scene_objects[i / 2].get_acceleration().x
                        * self.scene_objects[i / 2].get_mass()
                }
                1_usize => {
                    self.scene_objects[i / 2].get_acceleration().y
                        * self.scene_objects[i / 2].get_mass()
                }
                _ => 0_f32,
            }
        });

        if self.previous_force.len() != self.scene_objects.len() * 2 {
            self.previous_force.append(
                Axis(0),
                Array1::<f32>::zeros(Ix1(self.scene_objects.len() * 2)).view(),
            );
        }

        let mut right: ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>> =
            J.clone() * W.clone();
        right = right * Q;
        right = right * -1_f32;

        let mut left: ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>> =
            J * W * JT;

        let norm = |vector: &Array1<f32>| {
            let mut sum: f32 = 0_f32;
            for ele in vector.iter() {
                sum += ele.powi(2);
            }
            sum.sqrt()
        };

        let conjugate_gradient =
            |left: &Array2<f32>, right: ArrayView1<f32>, previous_force: &Array1<f32>| {
                // Reimplementation of the Wikipedia conugate gradient code from https://github.com/ange-yaghi/simple-2d-constraint-solver/blob/master/src/conjugate_gradient_sle_solver.cpp
                // Initialize necessary values at k = 0
                let mut residual: Array1<f32> = &right - left.dot(previous_force);
                let mut search_direction = residual.clone();
                let mut old_resid_norm = norm(&residual);
                let mut x = previous_force.clone();
                let mut iteration_count = 0;

                while iteration_count < 1000 && old_resid_norm > f32::EPSILON {
                    let left_search_direction = left.dot(&search_direction);
                    let step_size: f32 =
                        old_resid_norm.powi(2) / (search_direction.dot(&left_search_direction));
                    x = x.clone() + step_size * search_direction.clone();
                    // println!("test: {}", step_size * left_search_direction);
                    // break;
                    residual = residual - step_size * left_search_direction;
                    let new_resid_norm = norm(&residual);
                    search_direction = residual.clone()
                        + (new_resid_norm / old_resid_norm).powi(2) * search_direction.clone();
                    old_resid_norm = new_resid_norm;
                    iteration_count += 1;
                }
                x
            };

        let mut constraint_forces: &mut Array2<f32> = &mut Default::default();
        right.clone_into(constraint_forces);
        constraint_forces
            .columns_mut()
            .into_iter()
            .for_each(|mut constraint_right| {
                let constraint_force =
                    conjugate_gradient(&left, constraint_right.view(), &self.previous_force);
                for index in 0..constraint_force.len() {
                    constraint_right[index] = constraint_force[index];
                }
            });

        let constraint_forces = constraint_forces.sum_axis(Axis(0));

        // Apply calculated forces
        for index in 0..constraint_forces.len() / 2 {
            let mut prev_acceleration = self.scene_objects[index].get_acceleration();
            self.scene_objects[index].accelerate(Vector2 {
                x: constraint_forces[index * 2],
                y: constraint_forces[index * 2 + 1],
            });
            let mut velocity_coefficient =
                self.scene_objects[index].get_acceleration() / prev_acceleration;

            if prev_acceleration.x == 0_f32 {
                velocity_coefficient.x = if self.scene_objects[index].get_acceleration().x == 0_f32
                {
                    1_f32
                } else {
                    0_f32
                }
            }
            if prev_acceleration.y == 0_f32 {
                velocity_coefficient.y = if self.scene_objects[index].get_acceleration().y == 0_f32
                {
                    1_f32
                } else {
                    0_f32
                }
            }

            let mut object_velocity = self.scene_objects[index].get_velocity();
            object_velocity *= velocity_coefficient;
            *self.scene_objects[index].get_old_position_mut() =
                self.scene_objects[index].get_position() - object_velocity;
        }
    }

    fn apply_constraints(&mut self, d: &RaylibDrawHandle) {
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let centre_position = d.get_mouse_position() * 64_f32; //Vector2::new(640_f32 / 2_f32, 480_f32 / 2_f32) * 64_f32;
            let centre_radius = 100_f32 * 64_f32;

            for obj in self.scene_objects.iter_mut() {
                let to_obj: Vector2 = obj.get_position() - centre_position;
                let dist: f32 = (to_obj.x.powf(2f32) + to_obj.y.powf(2f32)).sqrt();

                if dist > centre_radius {
                    let original_position = obj.get_position();
                    let n: Vector2 = to_obj / dist;
                    obj.set_position(centre_position + n * centre_radius);
                    let position_offset = obj.get_position() - original_position;
                    obj.set_old_position(obj.get_old_position() + position_offset / 2_f32);
                }
            }
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for ele in self.scene_objects.iter_mut() {
            ele.update(dt);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    rl.set_target_fps(60);

    let mut solver: Solver = Solver::new();

    for i in 0..10 {
        let element = Box::from(Circle::new());
        solver.scene_objects.push(element);
        let element = &mut solver.scene_objects[i];
        (*element).set_position(Vector2::new(
            (*element).get_position().x + ((rand::random::<f32>() * 300_f32) - 150_f32) * 64_f32,
            (*element).get_position().y + ((rand::random::<f32>() * 80_f32) - 40_f32) * 64_f32,
        ));
        (*element).set_old_position((*element).get_position());
    }

    solver.constraints.push(Box::new(ScreenEdge::new()));
    //solver.constraints.push(Box::new(MouseFollow::new()));

    let mut air_resistance: bool = true;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        //ele.update((1_f32 / 144_f32) / 1.7_f32);
        d.gui_toggle(
            Rectangle {
                x: 10_f32,
                y: 10_f32,
                width: 200_f32,
                height: 24_f32,
            },
            Some(rstr!("Press W for Air Resistance")),
            &mut air_resistance,
        );
        {
            if d.is_key_pressed(KeyboardKey::KEY_W) {
                air_resistance = !air_resistance;
            }
        }

        const SUB_STEPS: u32 = 10;
        let dt = 0.0167f32;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _i in 0..SUB_STEPS {
            solver.apply_gravity();

            solver.apply_constraints(&d);
            solver.solve_constraints(&d);

            if air_resistance {
                solver.apply_air_resistance();
            }

            solver.update_positions(sub_dt);
        }

        d.clear_background(Color::WHITE);

        for ele in solver.scene_objects.iter_mut() {
            ele.draw(&mut d);
        }
    }
}
