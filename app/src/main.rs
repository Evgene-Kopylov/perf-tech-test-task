#![allow(unused)]

use macroquad::prelude::*;



const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const BULB_COLOR: Color = DARKGRAY;


const SIDE: f32 = 2.;
const START_POS: Vec2 = Vec2::new(10., 10.);

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
            self.pos.x,
            self.pos.y,
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

    fn expand(&mut self) {
        for bulb in self.bulbs.clone() {
            let left_pos = Vec2::new(bulb.pos.x + 1., bulb.pos.y);
            let bulbs = &self.bulbs;

            if bulbs.iter().filter(|&bulb| bulb.pos == left_pos).collect::<Vec<&Bulb>>().len() == 0 {
                self.bulbs.push(Bulb::new(left_pos))
            }

            // TODO
        }
    }

    fn draw(&self) {
        for bulb in self.bulbs.iter() {
            bulb.draw();
        }
    }

}



#[macroquad::main("breakout")]
async fn main() {
    let mut field = Field::start();

    loop {
        field.expand();

        clear_background(GRAY);

        field.draw();

        next_frame().await
    }
}
