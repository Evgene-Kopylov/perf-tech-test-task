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

    fn move_to(&mut self, pos: Vec2) {
        if self.bulbs.iter().filter(
            |&bulb| bulb.pos == pos
        ).collect::<Vec<&Bulb>>().len() == 0 {
            self.bulbs.push(Bulb::new(pos))
        }
    }

    fn expand(&mut self) {
        for i in 0..self.bulbs.len() {
            if self.bulbs[i].new {

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
