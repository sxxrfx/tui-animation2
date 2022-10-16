use core::time;

use cgmath::{ElementWise, Vector2};

use crate::utils::{ScreenBuffer, V2ff, FPS, HEIGHT, WIDTH};

pub type V2uu = Vector2<u32>;
pub struct Game {
    scrbuf: ScreenBuffer,
    win_size: V2uu,
    fps: u8,
}

impl Game {
    pub fn new(scrbuf: ScreenBuffer, win_size: V2uu, fps: u8) -> Self {
        Self {
            scrbuf: scrbuf,
            win_size: win_size,
            fps: fps,
        }
    }

    pub fn init() -> Self {
        Self {
            scrbuf: ScreenBuffer::init(),
            win_size: V2uu::new(WIDTH as u32, HEIGHT as u32),
            fps: FPS,
        }
    }

    pub fn space_animation(&mut self) {
        let one_micro = time::Duration::from_micros(1);
        let sleep_time = one_micro * 1000 * 1000 / self.fps as u32;
        let r: f32 = 8.0;
        let mut pos = V2ff::new(0.0, 30.0);
        let acc = V2ff::new(0.0, -9.8) / self.fps as f32;

        let loss = 0.05_f32;
        let mut velocity = V2ff::new(35.0, -1.0) / self.fps as f32;

        const C: Vector2<f32> = V2ff::new(0.0, HEIGHT as f32 - 1.0);
        const D: Vector2<f32> = V2ff::new(-1.0, 1.0);
        let mut i = 0;
        let turns = 1000;
        loop {
            if i > turns {
                break;
            }
            if pos.x > WIDTH as f32 {
                pos = V2ff::new(0.0, 30.0);
                velocity = V2ff::new(35.0, -1.0) / self.fps as f32;
            }
            if velocity.y < 0.0 && pos.y <= r {
                // if pos.y <= 0.0 {
                //     pos.y = -pos.y;
                // }
                // pos.y = 1.4 * r - pos.y;
                pos.y = r;
                velocity.y *= -1.0 * (1.0 - loss);
                // velocity.y *= -1.0;
            }
            self.scrbuf.circle((C - pos).mul_element_wise(D), r);
            self.scrbuf.show();
            self.scrbuf.refresh();

            std::thread::sleep(sleep_time);
            i += 1;
            velocity += acc;
            pos += velocity;
        }
    }

    pub fn ball_animation(&mut self) {
        let one_micro = time::Duration::from_micros(1);
        let sleep_time = one_micro * 1000 * 1000 / self.fps as u32;
        let r: f32 = 8.0;
        let mut pos = V2ff::new(0.0, 30.0);
        let acc = V2ff::new(0.0, -9.8) / self.fps as f32;

        let loss = 0.05_f32;
        let mut velocity = V2ff::new(35.0, -1.0) / self.fps as f32;

        const C: Vector2<f32> = V2ff::new(0.0, HEIGHT as f32 - 1.0);
        const D: Vector2<f32> = V2ff::new(-1.0, 1.0);
        let mut i = 0;
        let turns = 1000;
        loop {
            if i > turns {
                break;
            }
            if pos.x > WIDTH as f32 {
                pos = V2ff::new(0.0, 30.0);
                velocity = V2ff::new(35.0, -1.0) / self.fps as f32;
            }
            if velocity.y < 0.0 && pos.y <= r {
                // if pos.y <= 0.0 {
                //     pos.y = -pos.y;
                // }
                // pos.y = 1.4 * r - pos.y;
                pos.y = r;
                velocity.y *= -1.0 * (1.0 - loss);
                // velocity.y *= -1.0;
            }
            self.scrbuf.circle((C - pos).mul_element_wise(D), r);
            self.scrbuf.show();
            self.scrbuf.refresh();

            std::thread::sleep(sleep_time);
            i += 1;
            velocity += acc;
            pos += velocity;
        }
    }
}
