use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;

//green snake
const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.0];

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    pub fn oppposite(&self) -> Direction{
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Block{
    x:i32,
    y:i32
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}


impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }
}

fn draw(&self, con: &Context, g: &mut G2d){
    for block in &self.body {
        draw_block(SNAKE_COLOR, block.x, block.y, con, g);
    }
}

pub fn head_position(&self) -> (i32, i32){
    let head_block = self.body.front().unwrap();
    (head.block.x, head.block.y);
}
