use crate::objects::*;
use raylib::prelude::*;

pub mod objects;

struct Solver {
    scene_objects: Vec<Box<dyn PhysicsObject>>,
}

impl Solver {
    fn new() -> Self {
        Self {
            scene_objects: Vec::with_capacity(100),
        }
    }

    fn apply_gravity(&mut self) {
        for ele in self.scene_objects.iter_mut() {
            ele.accelerate(Vector2::new(0_f32, 981_f32) * 64_f32);
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
                    let n: Vector2 = to_obj / dist;
                    obj.set_position(centre_position + n * centre_radius);
                }
            }
        } else {
            for obj in self.scene_objects.iter_mut() {
                let velocity = obj.get_velocity();

                if obj.get_position().x < 0_f32 {
                    obj.get_position_mut().x = 0_f32;
                    obj.get_old_position_mut().x = obj.get_position().x + velocity.x
                } else if obj.get_position().x > 640_f32 * 64_f32 {
                    obj.get_position_mut().x = 640_f32 * 64_f32;
                    obj.get_old_position_mut().x = obj.get_position().x + velocity.x;
                } else if obj.get_position().y < 0_f32 {
                    obj.get_position_mut().y = 0_f32;
                    obj.get_old_position_mut().y = obj.get_position().y + velocity.y;
                } else if obj.get_position().y > 480_f32 * 64_f32 {
                    obj.get_position_mut().y = 480_f32 * 64_f32;
                    obj.get_old_position_mut().y = obj.get_position().y + velocity.y
                }
                // obj.set_position(
                //     obj.get_position()
                //         + Vector2::new(
                //             if obj.get_position().x < 5_f32 * 64_f32 {
                //                 (obj.get_position().x - 5_f32 * 64_f32) * -2_f32
                //             } else if obj.get_position().x
                //                 > d.get_screen_width() as f32 * 64_f32 - 5_f32 * 64_f32
                //             {
                //                 (obj.get_position().x - d.get_screen_width() as f32 * 64_f32
                //                     + 5_f32 * 64_f32)
                //                     * -2_f32
                //             } else {
                //                 0_f32
                //             },
                //             if obj.get_position().y < 5_f32 * 64_f32 {
                //                 (obj.get_position().y - 5_f32 * 64_f32) * -2_f32
                //             } else if obj.get_position().y
                //                 > d.get_screen_height() as f32 * 64_f32 - 5_f32 * 64_f32
                //             {
                //                 (obj.get_position().y - d.get_screen_height() as f32 * 64_f32
                //                     + 5_f32 * 64_f32)
                //                     * -2_f32
                //             } else {
                //                 0_f32
                //             },
                //         ),
                // );
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

    for i in 0..40 {
        let element = Box::from(Circle::new());
        solver.scene_objects.push(element);
        let element = &mut solver.scene_objects[i];
        (*element).set_position(Vector2::new(
            (*element).get_position().x + ((rand::random::<f32>() * 300_f32) - 150_f32) * 64_f32,
            (*element).get_position().y + ((rand::random::<f32>() * 80_f32) - 40_f32) * 64_f32,
        ));
        (*element).set_old_position((*element).get_position());
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        //ele.update((1_f32 / 144_f32) / 1.7_f32);
        const SUB_STEPS: u32 = 20;
        let dt = 0.0167f32;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _i in 0..SUB_STEPS {
            solver.apply_gravity();
            solver.apply_constraints(&d);
            solver.update_positions(sub_dt);
        }

        d.clear_background(Color::WHITE);

        for ele in solver.scene_objects.iter_mut() {
            ele.draw(&mut d);
        }

        {
            let mut total_kinetic_energy = 0_f32;
            for ele in solver.scene_objects.iter_mut() {
                total_kinetic_energy += (ele.get_position() - ele.get_old_position()).length();
            }
            d.draw_text(
                &format!("Total kinetic energy: {total_kinetic_energy}"),
                0,
                10,
                20,
                Color::BLACK,
            );
        }
    }
}
