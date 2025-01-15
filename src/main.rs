use macroquad::prelude::*;
use std::collections::VecDeque;

const SIZE: usize = 32;
const CELL: f32 = 20.0;
const OFFSET_Y: f32 = 60.0;
const OFFSET_X: f32 = 40.0;
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
    pos: Point,
    draw_pos: (f32, f32),
    pts: usize,
}

impl Fruit {
    fn change_points(&mut self, points: usize) {
        self.pts = points;
    }
    fn change_position(&mut self, pos: Point) {
        self.pos = pos;
        self.draw_pos = (
            OFFSET_X + CELL * self.pos.1 as f32,
            OFFSET_Y + CELL * self.pos.0 as f32,
        );
    }
    fn draw_fruit(&self) {
        draw_rectangle(self.draw_pos.0, self.draw_pos.1, CELL, CELL, RED);
    }
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
    }
    fn move_snake(&mut self) -> Option<()> {
        match &self.dir {
            Direction::UP => self.head.0 = (self.head.0 + SIZE - 1) % SIZE,
            Direction::DOWN => self.head.0 = (self.head.0 + SIZE + 1) % SIZE,
            Direction::LEFT => self.head.1 = (self.head.1 + SIZE - 1) % SIZE,
            Direction::RIGHT => self.head.1 = (self.head.1 + SIZE + 1) % SIZE,
        }
        if self.snake.contains(&self.head) {
            self.moves.push(self.head.clone());
            self.snake.push_front(self.head.clone());
            None
        } else {
            self.moves.push(self.head.clone());
            self.snake.push_front(self.head.clone());
            Some(())
        }
    }

    fn add_points(&mut self, points: &usize) {
        self.pts += points;
    }

    fn draw_snake(&self) {
        for p in self.snake.iter().skip(1) {
            draw_rectangle(
                OFFSET_X + CELL * p.1 as f32,
                OFFSET_Y + CELL * p.0 as f32,
                CELL,
                CELL,
                DARKGREEN,
            );
        }
        draw_rectangle(
            OFFSET_X + CELL * self.head.1 as f32,
            OFFSET_Y + CELL * self.head.0 as f32,
            CELL,
            CELL,
            GREEN,
        );
    }
}

struct Line {
    start: (f32, f32),
    end: (f32, f32),
}

fn build_grid() -> Vec<Line> {
    let mut grid: Vec<Line> = Vec::new();
    for i in 0..SIZE + 1 {
        grid.push(Line {
            start: (OFFSET_X + CELL * i as f32, OFFSET_Y),
            end: (OFFSET_X + CELL * i as f32, OFFSET_Y + CELL * SIZE as f32),
        });
        grid.push(Line {
            start: (OFFSET_X, OFFSET_Y + CELL * i as f32),
            end: (OFFSET_X + CELL * SIZE as f32, OFFSET_Y + CELL * i as f32),
        });
    }
    grid
}

fn draw_grid(grid: &Vec<Line>) {
    for l in grid {
        draw_line(l.start.0, l.start.1, l.end.0, l.end.1, 1.0, DARKGRAY)
    }
}

fn draw_start() {
    draw_text("Press enter to start the game!", 35.0, 300.0, 50.0, WHITE);
}
fn draw_pause() {
    draw_text("Press enter to continue!", 100.0, 300.0, 50.0, WHITE);
}

fn new_game() -> (Snake, Fruit) {
    let fruit: Fruit = Fruit {
        pos: (10, 16),
        draw_pos: (OFFSET_X + CELL * 16.0, OFFSET_Y + CELL * 10.0),
        pts: 5,
    };

    let snake: Snake = Snake {
        snake: VecDeque::from(vec![(28, 16), (29, 16), (30, 16)]),
        head: (28, 16),
        dir: Direction::UP,
        pts: 0,
        moves: Vec::new(),
    };
    (snake, fruit)
}

#[macroquad::main("Snake")]
async fn main() {
    request_new_screen_size(720.0, 720.0);
    let time = 0.1;
    let mut last = get_time();

    let mut start: bool = true;
    let mut pause: bool = false;

    let mut fruit: Fruit;
    let mut snake: Snake;
    (snake, fruit) = new_game();

    let grid: Vec<Line> = build_grid();

    loop {
        if start {
            if is_key_pressed(KeyCode::Enter) {
                start = false;
                (snake, fruit) = new_game();
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

            if get_time() - last > time {
                last = get_time();
                match snake.move_snake() {
                    None => {
                        println!("Game ended. Your points were {}.", snake.pts);
                        start = true;
                    }
                    Some(_x) => {
                        if snake.head == fruit.pos {
                            snake.add_points(&fruit.pts);
                            fruit.change_position((16, 16));
                        } else {
                            snake.snake.pop_back();
                        }
                    }
                }
            }
        }

        clear_background(BLACK);
        draw_grid(&grid);
        let txt = format!("Points: {}", snake.pts);
        draw_text(&txt, 270.0, 40.0, 50.0, BLUE);
        snake.draw_snake();
        fruit.draw_fruit();

        if start {
            draw_start();
        } else if pause {
            draw_pause();
        }
        next_frame().await
    }
}
