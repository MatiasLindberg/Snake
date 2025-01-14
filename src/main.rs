use macroquad::prelude::*;
use std::collections::LinkedList;

const SIZE: usize = 32;
const CELL: f32 = 50.0;
type Point = (usize, usize);

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct ScoreSnake {
    points: usize,
    moves: LinkedList<Point>,
}

struct Fruit {
    position: (usize, usize),
    points: usize,
}

impl Fruit {
    fn change_points(&mut self, points: usize) {
        self.points = points;
    }
    fn change_position(&mut self, pos: (usize, usize)) {
        self.position = pos;
    }
    fn draw_fruit() {}
}

struct Snake {
    snake: LinkedList<Point>,
    direction: Direction,
    points: usize,
    moves: LinkedList<Point>,
}

impl Snake {
    fn turn_snake(&mut self, new_dir: Direction) {
        if (self.direction == Direction::LEFT && new_dir == Direction::RIGHT)
            || (self.direction == Direction::UP && new_dir == Direction::DOWN)
            || (self.direction == Direction::DOWN && new_dir == Direction::UP)
            || (self.direction == Direction::RIGHT && new_dir == Direction::LEFT)
        {
            return;
        }
        self.direction = new_dir;
    }
    fn move_snake() {}

    fn add_points(&mut self, points: usize) {
        self.points += points;
    }

    fn draw_snake() {}
}

struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    fn new_fruit_position(&self) -> (usize, usize) {
        let mut possible: Vec<(usize, usize)> = Vec::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.map[y][x] {
                    possible.push((y, x));
                }
            }
        }
        // Get random possible
        (0, 0)
    }
    fn draw_map() {}
}

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
