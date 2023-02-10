#![allow(unused)]

use macroquad::prelude::*;



const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const BULB_COLOR: Color = DARKGRAY;


const SIDE: f32 = 1.;
const START_POS: Vec2 = Vec2::new(1000., 1000.);

#[derive(Debug, Clone)]
struct Bulb {
    pos: Vec2,
    new: bool,
}


struct Field {
    bulbs: Vec<Bulb>,
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
    fn start() -> Self {
        let mut bulbs = Vec::new();
        bulbs.push(Bulb::new(START_POS));
        Self {
            bulbs
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
        let free_space = self.bulbs.iter().filter(
            |&bulb| bulb.pos == pos
        ).collect::<Vec<&Bulb>>().len() == 0;

        let can_step_by_sum: bool = self.sum_of_literal_digits(pos) < 26.;

        if free_space && can_step_by_sum {
            self.bulbs.push(Bulb::new(pos))
        }
    }

    fn expand(&mut self) -> bool {
        let mut have_new = false;
        for i in 0..self.bulbs.len() {
            if self.bulbs[i].new {
                have_new = true;

                let right_pos = Vec2::new(self.bulbs[i].pos.x + 1., self.bulbs[i].pos.y);
                self.move_to(right_pos);

                let left_pos = Vec2::new(self.bulbs[i].pos.x - 1., self.bulbs[i].pos.y);
                self.move_to(left_pos);

                let up_pos = Vec2::new(self.bulbs[i].pos.x, self.bulbs[i].pos.y + 1.);
                self.move_to(up_pos);

                let down_pos = Vec2::new(self.bulbs[i].pos.x, self.bulbs[i].pos.y - 1.);
                self.move_to(down_pos);
            }

            self.bulbs[i].new = false;

        }
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
    let mut field = Field::start(START_POS);

    loop {
        clear_background(GRAY);


        field.draw();
        let active = field.expand();

        let total = format!("{}", field.bulbs.len());
        draw_text_ex(&total, 35.0, 35.0, TextParams::default());

        next_frame().await
    }
}
