use cgmath::{ElementWise, Vector2};
use std::time;
mod utils;
use crate::utils::{ScreenBuffer, V2ff, HEIGHT, WIDTH};

fn main() {
    let mut scrbuf = ScreenBuffer::init();
    let one_micro = time::Duration::from_micros(1);
    let fps = 30;
    let sleep_time = one_micro * 1000 * 1000 / fps;
    let r: f32 = 8.0;
    let mut pos = V2ff::new(0.0, 30.0);
    let acc =V2ff::new(0.0, -9.8) / fps as f32;

    let loss = 0.05_f32;
    let mut velocity = V2ff::new(35.0, -1.0) / fps as f32;

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
            velocity = V2ff::new(35.0, -1.0) / fps as f32;

        }
        if velocity.y < 0.0 && pos.y <= r  {
            // if pos.y <= 0.0 {
            //     pos.y = -pos.y;
            // }
            // pos.y = 1.4 * r - pos.y;
            pos.y = r; 
            velocity.y *= -1.0 * (1.0-loss);
            // velocity.y *= -1.0;
        }
        scrbuf.circle((C - pos).mul_element_wise(D) , r);
        scrbuf.show();
        scrbuf.refresh();
        
        std::thread::sleep(sleep_time);
        i+= 1;
        velocity += acc;
        pos += velocity;
    }

}
