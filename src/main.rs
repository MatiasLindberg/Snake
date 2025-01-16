use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const SIZE: usize = 32;
const CELL: f32 = 20.0;
const OFFSET_Y: f32 = 60.0;
const OFFSET_X: f32 = 40.0;
type Point = (usize, usize);
type DrawPoint = (f32, f32);

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(Clone, Serialize, Deserialize)]
struct Score {
    name: String,
    points: usize,
    moves: Vec<Point>,
}
#[derive(Serialize, Deserialize)]
struct ListOfScores {
    scores_list: Vec<Score>,
}

impl ListOfScores {
    fn sort_scores(&mut self) {
        self.scores_list.sort_by(|a, b| b.points.cmp(&a.points));
    }

    fn new_score(&mut self, s: &Snake) {
        if s.pts >= self.scores_list.last().unwrap().points {
            self.scores_list.pop();
            let n = s.record();
            self.scores_list.push(n);
        }
        self.sort_scores();
    }
    fn reset_scores_list(&mut self) {
        let tom: Score = Score {
            name: "Tom".to_string(),
            points: 250,
            moves: Vec::new(),
        };
        let tim: Score = Score {
            name: "Tim".to_string(),
            points: 200,
            moves: Vec::new(),
        };
        let jim: Score = Score {
            name: "Jim".to_string(),
            points: 150,
            moves: Vec::new(),
        };
        let kim: Score = Score {
            name: "Kim".to_string(),
            points: 50,
            moves: Vec::new(),
        };
        let dim: Score = Score {
            name: "Dim".to_string(),
            points: 10,
            moves: Vec::new(),
        };
        self.scores_list = vec![tom, tim, jim, kim, dim];
    }

    fn draw_highscore(&self) {
        draw_rectangle(
            OFFSET_X + 50.0,
            OFFSET_Y + 50.0,
            720.0 - 2.0 * OFFSET_X - 100.0,
            720.0 - 2.0 * OFFSET_X - 100.0,
            DARKGRAY,
        );

        let text_start = OFFSET_X + 120.0;
        let mut text_line = OFFSET_Y + 120.0;
        for score in &self.scores_list {
            let txt = format!("{}    {}", score.name, score.points);
            draw_text(&txt, text_start, text_line, 80.0, RED);
            text_line += 100.0;
        }
    }
    fn save_scores(&self) -> std::io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("highscores.json")?;

        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
    fn load_scores(&self) {}
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
    name: String,
    snake: VecDeque<Point>,
    head: Point,
    dir: Direction,
    pts: usize,
    moves: Vec<Point>,
}

impl Snake {
    fn record(&self) -> Score {
        Score {
            name: self.name.clone(),
            points: self.pts.clone(),
            moves: self.moves.clone(),
        }
    }
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
    start: DrawPoint,
    end: DrawPoint,
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

fn eat_fruit(snake: &mut Snake, fruit: &mut Fruit) {
    snake.add_points(&fruit.pts);

    let range: Vec<_> = (0..SIZE).collect();
    'nxt_fruit: loop {
        let next_y = range.choose();
        let next_x = range.choose();
        match (next_y, next_x) {
            (Some(&y), Some(&x)) => {
                if !snake.snake.contains(&(y, x)) {
                    fruit.change_position((y, x));
                    break 'nxt_fruit;
                }
            }
            _ => println!("ERROR in num gen!"),
        }
    }
    if snake.pts >= 20 * fruit.pts {
        fruit.change_points(fruit.pts + 5);
    }
}

fn new_game() -> (Snake, Fruit) {
    let fruit: Fruit = Fruit {
        pos: (10, 16),
        draw_pos: (OFFSET_X + CELL * 16.0, OFFSET_Y + CELL * 10.0),
        pts: 5,
    };

    let snake: Snake = Snake {
        name: "Matias".to_string(),
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
    let mut highscore: bool = false;

    let mut scores: ListOfScores = ListOfScores {
        scores_list: Vec::new(),
    };
    scores.reset_scores_list();
    scores.save_scores();
    let mut fruit: Fruit;
    let mut snake: Snake;
    (snake, fruit) = new_game();

    let grid: Vec<Line> = build_grid();

    loop {
        clear_background(BLACK);
        draw_grid(&grid);
        let txt = format!("Points: {}", snake.pts);
        draw_text(&txt, 270.0, 40.0, 50.0, BLUE);
        snake.draw_snake();
        fruit.draw_fruit();

        if start {
            if is_key_pressed(KeyCode::Enter) {
                start = false;
                (snake, fruit) = new_game();
            }
        } else if is_key_down(KeyCode::Tab) {
            highscore = true;
        } else if is_key_released(KeyCode::Tab) {
            highscore = false;
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
                        scores.new_score(&snake);
                        start = true;
                    }
                    Some(_x) => {
                        if snake.head == fruit.pos {
                            eat_fruit(&mut snake, &mut fruit);
                        } else {
                            snake.snake.pop_back();
                        }
                    }
                }
            }
        }

        if start {
            draw_start();
        } else if pause {
            draw_pause();
        } else if highscore {
            scores.draw_highscore();
        }

        next_frame().await
    }
}
