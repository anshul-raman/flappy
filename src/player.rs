use crate::constans::*;
use bracket_lib::prelude::*;

#[derive(Debug)]
enum WingState {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Player {
    velocity_y: f32,
    x: f32,
    y: f32,
    acc_x: f32,
    acc_y: f32,
    velocity_x: f32,
    gravity: f32,
    wing_state: WingState,
    clock_sec: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            velocity_x: 1.0,
            velocity_y: 0.0,
            gravity: 2.0,
            acc_x: 0.00001,
            acc_y: 0.0,
            wing_state: WingState::Up,
            clock_sec: 0.0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y as i32, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn gravity_and_move(&mut self, time_sec: f32) {
        // Update velocity
        let u_x = self.velocity_x;
        let u_y = self.velocity_y;
        self.velocity_y = u_y + (self.gravity + self.acc_y) * time_sec;
        self.velocity_y = self.velocity_y.max(VELOCITY_Y_MIN);
        self.velocity_x = u_x + self.acc_x * time_sec;
        self.velocity_x = self.velocity_x.min(VELOCITY_X_MAX);

        // x direction
        let s_x = u_x + self.acc_x * time_sec;
        self.x += s_x;

        // y direction
        let s_y = self.velocity_y * time_sec;
        self.y += s_y;

        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity_y = 0.0;
        }

        // Update clock
        self.clock_sec += time_sec;
        if self.clock_sec >= 0.25 {
            self.clock_sec = 0.0;
            self.wing_state = WingState::Up;
            self.acc_y = 0.0;
        }
    }

    pub fn flap(&mut self) {
        match &self.wing_state {
            WingState::Up => {
                self.wing_state = WingState::Down;
                self.acc_y -= 50.0;
            }
            _ => {}
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x as i32
    }

    pub fn get_y(&self) -> i32 {
        self.y as i32
    }
}
