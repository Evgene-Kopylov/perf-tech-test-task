#![allow(unused)]

use macroquad::prelude::*;
use std::time::{Instant};


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const BULB_COLOR: Color = DARKGRAY;


const SIDE: f32 = 1.;
const START_POS: Vec2 = Vec2::new(1000., 1000.);

#[derive(Debug, Clone, Copy)]
struct Bulb {
    pos: Vec2,
    new: bool,
}


struct Field {
    front_line: Vec<Bulb>,
    bulbs: Vec<Bulb>,
    total: i32,
}


impl Bulb {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            new: true
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x - START_POS.x + 15.,
            self.pos.y - START_POS.y + 15.,
            SIDE,
            SIDE,
            BULB_COLOR
        );
    }
}


impl Field {
    fn start(start_pos: Vec2) -> Self {
        let mut bulbs = Vec::new();
        bulbs.push(Bulb::new(start_pos));
        Self {
            front_line: bulbs,
            bulbs: Vec::new(),
            total: 1,
        }
    }

    fn sum_of_literal_digits(&self, pos: Vec2) -> f32 {
        let x = (pos.x as i32).to_string();
        let y = (pos.y as i32).to_string();
        let xy = x + &y;
        let mut s = 0.;
        for c in xy.chars() {
            s += c.to_string().parse::<f32>().unwrap() as f32;
        }
        s as f32
    }

    fn move_to(&mut self, pos: Vec2) {
        let free_space = self.front_line.iter().filter(
            |&bulb| bulb.pos == pos
        ).collect::<Vec<&Bulb>>().len() == 0;

        let can_step_by_sum: bool = self.sum_of_literal_digits(pos) < 26.;


        if free_space && can_step_by_sum {
            self.front_line.push(Bulb::new(pos))
        }
    }

    fn expand(&mut self) -> bool {
        let mut have_new = false;
        for i in 0..self.front_line.len() {
            if self.front_line[i].new {
                have_new = true;

                let right_pos = Vec2::new(self.front_line[i].pos.x + 1., self.front_line[i].pos.y);
                self.move_to(right_pos);

                // let left_pos = Vec2::new(self.bulbs[i].pos.x - 1., self.bulbs[i].pos.y);
                // self.move_to(left_pos);

                let up_pos = Vec2::new(self.front_line[i].pos.x, self.front_line[i].pos.y + 1.);
                self.move_to(up_pos);

                // let down_pos = Vec2::new(self.bulbs[i].pos.x, self.bulbs[i].pos.y - 1.);
                // self.move_to(down_pos);

                self.front_line[i].new = false;
                self.bulbs.push(self.front_line[i].clone());
            }


        }

        self.front_line = self.front_line.iter().filter(
            |&bulb| bulb.new
        ).map(|b| *b).collect::<Vec<Bulb>>();


        have_new
    }

    fn draw(&self) {
        for bulb in self.bulbs.iter() {
            bulb.draw();
        }
    }

}



#[macroquad::main("breakout")]
async fn main() {
    let time_start = Instant::now();
    let mut field = Field::start(START_POS);

    loop {
        clear_background(GRAY);


        field.draw();
        let active = field.expand();
        if !active {
            println!("steps: {}", field.bulbs.len());
            println!("time: {}", time_start.elapsed().as_secs_f64());
            break
        }

        // let total = format!("steps: {}", field.bulbs.len());
        // draw_text_ex(&total, 35.0, 35.0, TextParams::default());
        // let time = format!("time: {}", time_start.elapsed().as_secs_f64());
        // draw_text_ex(&time, 35.0, 75.0, TextParams::default());
        next_frame().await
    }
}
