extern crate webots_sys;

use webots_sys::*;

macro_rules! c_str {
    ($s:expr) => { {
        concat!($s, "\0").as_ptr() as *const i8
    } }
}

const MAX_SPEED: f64 = 6.28;

fn main() {
    unsafe {
        wb_robot_init();

        let left_motor = wb_robot_get_device(c_str!("left wheel motor"));
        let right_motor = wb_robot_get_device(c_str!("right wheel motor"));
        wb_motor_set_position(left_motor, INFINITY);
        wb_motor_set_position(right_motor, INFINITY);

        wb_motor_set_velocity(left_motor, 0.5 * MAX_SPEED);
        wb_motor_set_velocity(right_motor, 0.5 * MAX_SPEED);
        
        while wb_robot_step(64) != -1 {}
        wb_robot_cleanup();
    }
}