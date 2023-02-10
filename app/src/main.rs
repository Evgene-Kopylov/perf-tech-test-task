// #![allow(unused)]

use macroquad::prelude::*;
use std::time::{Instant};


const STEP_COLOR: Color = DARKGRAY;
const SIDE: f32 = 1.;
const START_POS: Vec2 = Vec2::new(1000., 1000.);

#[derive(Debug, Clone, Copy)]
struct Step {
    pos: Vec2,
    new: bool,
}


struct Field {
    front_line: Vec<Step>,
    steps: Vec<Step>,
}


impl Step {
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
            STEP_COLOR
        );
    }
}


impl Field {
    fn start(start_pos: Vec2) -> Self {
        let mut steps = Vec::new();
        steps.push(Step::new(start_pos));
        Self {
            front_line: steps,
            steps: Vec::new(),
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
            |&step| step.pos == pos
        ).collect::<Vec<&Step>>().len() == 0;

        let can_step_by_sum: bool = self.sum_of_literal_digits(pos) < 26.;


        if free_space && can_step_by_sum {
            self.front_line.push(Step::new(pos))
        }
    }

    fn expand(&mut self) -> bool {
        let mut have_new = false;
        for i in 0..self.front_line.len() {
            if self.front_line[i].new {
                have_new = true;

                let right_pos = Vec2::new(self.front_line[i].pos.x + 1., self.front_line[i].pos.y);
                self.move_to(right_pos);

                // let left_pos = Vec2::new(self.steps[i].pos.x - 1., self.steps[i].pos.y);
                // self.move_to(left_pos);

                let up_pos = Vec2::new(self.front_line[i].pos.x, self.front_line[i].pos.y + 1.);
                self.move_to(up_pos);

                // let down_pos = Vec2::new(self.steps[i].pos.x, self.steps[i].pos.y - 1.);
                // self.move_to(down_pos);

                self.front_line[i].new = false;
                self.steps.push(self.front_line[i].clone());
            }


        }

        self.front_line = self.front_line.iter().filter(
            |&step| step.new
        ).map(|b| *b).collect::<Vec<Step>>();


        have_new
    }

    fn draw(&self) {
        for step in self.steps.iter() {
            step.draw();
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
        let steps = format!("steps: {}", field.steps.len());
        let time = format!("time: {:.1}", time_start.elapsed().as_secs_f64());
        if !active {
            println!("{}", steps);
            println!("{}", time);
            break
        }
        draw_text_ex(&steps, 35.0, 35.0, TextParams::default());
        draw_text_ex(&time, 35.0, 75.0, TextParams::default());
        next_frame().await
    }
}
