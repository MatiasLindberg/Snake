use macroquad::prelude::*;
use std::collections::VecDeque;

const SIZE: usize = 32;
const CELL: f32 = 20.0;
const OFFSET: f32 = 40.0;
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
    moves: Vec<Point>,
}

struct Fruit {
    position: (usize, usize),
    points: usize,
}

impl Fruit {
    fn change_points(&mut self, points: usize) {
        self.points = points;
    }
    fn change_position(&mut self, pos: Point) {
        self.position = pos;
    }
    fn draw_fruit() {}
}

struct Snake {
    snake: VecDeque<Point>,
    head: Point,
    dir: Direction,
    pts: usize,
    moves: Vec<Point>,
}

impl Snake {
    fn turn_snake(&mut self, new_dir: Direction) {
        if (self.dir == new_dir || self.dir == Direction::LEFT && new_dir == Direction::RIGHT)
            || (self.dir == Direction::UP && new_dir == Direction::DOWN)
            || (self.dir == Direction::DOWN && new_dir == Direction::UP)
            || (self.dir == Direction::RIGHT && new_dir == Direction::LEFT)
        {
            return;
        }
        self.dir = new_dir;
        println!("Changed direction");
    }
    fn move_snake(&mut self) -> Option<()> {
        match &self.dir {
            Direction::UP => self.head.0 = (self.head.0 + SIZE - 1) % SIZE,
            Direction::DOWN => self.head.0 = (self.head.0 + SIZE + 1) % SIZE,
            Direction::LEFT => self.head.1 = (self.head.1 + SIZE - 1) % SIZE,
            Direction::RIGHT => self.head.1 = (self.head.1 + SIZE + 1) % SIZE,
        }
        if self.snake.contains(&self.head) {
            None
        } else {
            self.moves.push(self.head.clone());
            self.snake.push_front(self.head.clone());
            self.snake.pop_back();
            Some(())
        }
    }

    fn add_points(&mut self, points: usize) {
        self.pts += points;
    }

    fn draw_snake(&self) {
        draw_rectangle(
            OFFSET + CELL * self.head.1 as f32,
            OFFSET + CELL * self.head.0 as f32,
            CELL,
            CELL,
            GREEN,
        );
        for p in self.snake.iter().skip(1) {
            draw_rectangle(
                OFFSET + CELL * p.1 as f32,
                OFFSET + CELL * p.0 as f32,
                CELL,
                CELL,
                DARKGREEN,
            );
        }
    }
}

fn draw_map() {
    for i in 0..SIZE + 1 {
        draw_line(
            OFFSET + CELL * i as f32,
            OFFSET,
            OFFSET + CELL * i as f32,
            OFFSET + CELL * SIZE as f32,
            1.0,
            DARKGRAY,
        );
        draw_line(
            OFFSET,
            OFFSET + CELL * i as f32,
            OFFSET + CELL * SIZE as f32,
            OFFSET + CELL * i as f32,
            1.0,
            DARKGRAY,
        );
    }
}

fn draw_start() {
    draw_text("Press enter to start the game!", 35.0, 300.0, 50.0, WHITE);
}
fn draw_pause() {
    draw_text("Press enter to continue!", 100.0, 300.0, 50.0, WHITE);
}

#[macroquad::main("MyGame")]
async fn main() {
    request_new_screen_size(720.0, 720.0);
    let mut start: bool = true;
    let mut pause: bool = false;

    let mut snake: Snake = Snake {
        snake: VecDeque::from(vec![(28, 16), (29, 16), (30, 16)]),
        head: (28, 16),
        dir: Direction::UP,
        pts: 0,
        moves: Vec::new(),
    };

    loop {
        clear_background(BLACK);
        draw_map();
        snake.draw_snake();
        if start {
            draw_start();
        } else if pause {
            draw_pause();
        } else {
            snake.move_snake();
        }

        if start {
            if is_key_pressed(KeyCode::Enter) {
                start = false;
            }
        } else if pause {
            if is_key_pressed(KeyCode::Enter) {
                pause = false;
            }
        } else {
            if is_key_pressed(KeyCode::Escape) {
                pause = true;
            }
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                snake.turn_snake(Direction::UP);
            } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                snake.turn_snake(Direction::LEFT);
            } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                snake.turn_snake(Direction::RIGHT);
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                snake.turn_snake(Direction::DOWN);
            }
        }
        next_frame().await
    }
}
