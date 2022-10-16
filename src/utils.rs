use cgmath::{Vector2};
use crossterm::{
    cursor::{DisableBlinking,},
    execute, queue,
    style::{
        self,  Color,  ResetColor,  SetForegroundColor, 
    },
};
use std::fmt::{Debug, Display, Formatter, self};
use std::io::{self, stdout, Write};

#[derive(Clone,Debug, Copy)]
pub enum Pixel {
    Empty,
    Full
}
pub type V2ff = Vector2<f32>;

#[derive(Clone, Copy)]
pub struct Stars{
    pub depth: usize,
    pub velocity: V2ff,
    pub sprite: char,
}

impl Default for Stars {
    fn default() -> Self {
        Self {
            depth: usize::default(),
            velocity: V2ff::new(0., 0.),
            sprite: '*',
        }
    }
}
impl Display for Stars {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sprite) 
    }
}
impl Debug for Stars {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Star: \n\
        \tdepth:  {}\n\
        \tvelocity:  ({}, {})\n\
        \tsprite: {}",self.depth, self.velocity.x, self.velocity.y, self.sprite) 
    }
}

impl Stars {
    pub fn new(depth: usize, velocity: V2ff, sprite: char) -> Self {
        Self {
            depth: depth,
            velocity: velocity,
            sprite: sprite,
        }
    }
}

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 40;
pub const FPS: u8 = 30;
pub const LEN_A: usize = WIDTH * HEIGHT;
pub const LEN_B: usize = LEN_A /2;
// const LEN_B: usize = LEN_A; 
#[derive(Debug)]
pub struct ScreenBuffer{
    pub a: [Pixel; LEN_A],
    pub b: [char; LEN_B]
}

impl ScreenBuffer {
    pub fn init() -> Self{
        let a = [Pixel::Empty; LEN_A];
        let b = [' '; LEN_B];
        ScreenBuffer {
            a,
            b
        }
    }
    pub fn refresh(&mut self){
        self.a = [Pixel::Empty; LEN_A];
        self.b = [' '; LEN_B];
    }


    pub fn show(&self){
        let up = format!("\u{001b}[{}A", (HEIGHT/2) + 1);
        let left = format!("\u{001b}[{}D", WIDTH + 2);
        let mut stdout = stdout();
        // let (cols, rows) = size().unwrap();
        execute!(stdout, 
            // SetSize(WIDTH as u16, (HEIGHT/2) as u16),
            // terminal::Clear(terminal::ClearType::All),
            // SavePosition,
            SetForegroundColor(Color::Blue),
            // Red background
            // SetBackgroundColor(Color::Red),
            // Print text
            // Print("Blue text on Red.".to_string()),
            // Reset to default colors
            DisableBlinking,
            style::Print("+--------------------------------------------------------------------------------+\n".to_string())
        ).unwrap();


        let mut i = 0;
        for _ in 0..(HEIGHT/2) {
            // let s = String::from_iter(&self.b[i..i+WIDTH]);
            let s = format!("|{}|\n", String::from_iter(&self.b[i..i+WIDTH]));
            queue!(stdout, 
                // style::Print("|".to_string()),
                style::Print(s),
                // style::Print("|\n".to_string()),
            ).unwrap();
            i += WIDTH
        }
        execute!(stdout, 
            style::Print("+--------------------------------------------------------------------------------+".to_string()),
            style::Print(&left),
            style::Print(&up),
            ResetColor,
            // MoveTo(0, 0),
        ).unwrap();
        // stdout.execute(RestorePosition).unwrap();
        stdout.flush().unwrap();
    }
    pub fn circle(&mut self, pos: V2ff, r: f32){
        let rr = V2ff::new(r, r);
        let bl = pos - rr;
        let tr = pos + rr;
        let bx = f32::floor(bl.x) as i32;
        let by = f32::floor(bl.y) as i32;
        let tx = f32::ceil(tr.x) as i32;
        let ty = f32::ceil(tr.y) as i32;
        
        for y in by..=ty {
            for x in bx..=tx {
                if (0 <= x) && (x < WIDTH as i32) && (0 <= y) && (y < HEIGHT as i32) {
                    let dx = pos.x - (x as f32 + 0.5) as f32; 
                    let dy = pos.y - (y as f32 + 0.5) as f32;
                    if ((dx)*(dx) + (dy)*(dy)) <= r*r {
                        self.a[y as usize * WIDTH + x as usize] = Pixel::Full;
                    }
                }
            }
        }
        for y in 0..(HEIGHT/2) as i32 {
            for x in 0..WIDTH as i32 {
                self.b[y as usize * WIDTH + x as usize] = match (self.a[(2 * y) as usize * WIDTH + x as usize], self.a[(2 * y + 1) as usize * WIDTH + x as usize]){
                        (Pixel::Empty, Pixel::Empty) => ' ',
                        (Pixel::Full, Pixel::Empty) => '^',
                        (Pixel::Empty, Pixel::Full) => '_',
                        (Pixel::Full, Pixel::Full) => '0',
                    };
            }
        }
    }
}
