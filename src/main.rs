use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

const SIZE: usize = 32;
const CELL: f32 = 20.0;
const OFFSET_Y: f32 = 60.0;
const OFFSET_X: f32 = 40.0;
const TIME: f64 = 0.15;
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
    fruits: Vec<Point>,
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
        self.save_scores().unwrap();
    }

    fn draw_highscore(&self) {
        draw_rectangle(
            OFFSET_X + 50.0,
            OFFSET_Y + 50.0,
            720.0 - 2.0 * OFFSET_X - 100.0,
            720.0 - 2.0 * OFFSET_X - 100.0,
            DARKGRAY,
        );

        let text_start = OFFSET_X + 110.0;
        let mut text_line = OFFSET_Y + 120.0;
        for score in &self.scores_list {
            let mut txt: String = score.name.to_string();
            txt.push_str(
                &(std::iter::repeat(" ")
                    .take(12 - score.name.len())
                    .collect::<String>()),
            );
            txt.push_str(&score.points.to_string());
            draw_text(&txt, text_start, text_line, 60.0, RED);
            text_line += 90.0;
        }
    }
    fn save_scores(&self) -> std::io::Result<()> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("Highscores.json")?;

        serde_json::to_writer_pretty(file, &self.scores_list)?;
        Ok(())
    }

    fn load_scores(&mut self) -> std::io::Result<()> {
        let file = std::fs::File::open("Highscores.json")?;
        let sco: Vec<Score> = serde_json::from_reader(&file)?;
        self.scores_list = sco;
        Ok(())
    }
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
    fruits: Vec<Point>,
}

impl Snake {
    fn record(&self) -> Score {
        Score {
            name: self.name.clone(),
            points: self.pts.clone(),
            moves: self.moves.clone(),
            fruits: self.fruits.clone(),
        }
    }
    fn turn_snake(&mut self, new_dir: Direction) {
        match new_dir {
            Direction::UP => {
                if *self.snake.front().unwrap() == ((self.head.0 + SIZE - 1) % SIZE, self.head.1) {
                    return;
                }
            }
            Direction::DOWN => {
                if *self.snake.front().unwrap() == ((self.head.0 + SIZE + 1) % SIZE, self.head.1) {
                    return;
                }
            }
            Direction::LEFT => {
                if *self.snake.front().unwrap() == (self.head.0, (self.head.1 + SIZE - 1) % SIZE) {
                    return;
                }
            }
            Direction::RIGHT => {
                if *self.snake.front().unwrap() == (self.head.0, (self.head.1 + SIZE + 1) % SIZE) {
                    return;
                }
            }
        }
        self.dir = new_dir;
    }
    fn move_snake(&mut self) -> Option<()> {
        self.moves.push(self.head.clone());
        self.snake.push_front(self.head.clone());
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
            Some(())
        }
    }

    fn add_points(&mut self, points: &usize) {
        self.pts += points;
    }

    fn draw_snake(&self) {
        for p in self.snake.iter() {
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
    snake.fruits.push(fruit.pos.clone());

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

async fn replay(sco: &Score) {
    let mut last = get_time();
    let grid: Vec<Line> = build_grid();

    let mut fru: std::iter::Peekable<std::slice::Iter<'_, Point>> = sco.fruits.iter().peekable();
    let mut mov: std::iter::Peekable<std::slice::Iter<'_, Point>> = sco.moves.iter().peekable();
    let mut sna: VecDeque<Point> = VecDeque::from(vec![(28, 16), (29, 16), (30, 16)]);

    let mut pause: bool = true;
    let mut ended: bool = false;

    let txt_name = sco.name.to_string();
    let txt_points = sco.points.to_string();
    loop {
        clear_background(BLACK);
        draw_grid(&grid);

        if is_key_pressed(KeyCode::Escape) {
            return;
        } else if is_key_pressed(KeyCode::Enter) {
            pause = !pause;
        }

        draw_text(&txt_name, 180.0, 40.0, 50.0, BLUE);
        draw_text(&txt_points, 450.0, 40.0, 50.0, BLUE);

        draw_rectangle(
            OFFSET_X + CELL * sna[0].1 as f32,
            OFFSET_Y + CELL * sna[0].0 as f32,
            CELL,
            CELL,
            GREEN,
        );
        for p in 1..sna.len() {
            draw_rectangle(
                OFFSET_X + CELL * sna[p].1 as f32,
                OFFSET_Y + CELL * sna[p].0 as f32,
                CELL,
                CELL,
                DARKGREEN,
            );
        }
        draw_rectangle(
            OFFSET_X + fru.peek().unwrap().1 as f32 * CELL,
            OFFSET_Y + fru.peek().unwrap().0 as f32 * CELL,
            CELL,
            CELL,
            RED,
        );

        if mov.peek().is_none() {
            ended = true;
        }

        if ended {
            draw_text("Replay ended!", 220.0, 300.0, 50.0, WHITE);
            draw_text("Press Esc to return.", 150.0, 360.0, 50.0, WHITE);
        } else if pause {
            draw_pause();
        } else if get_time() - last > TIME {
            last = get_time();

            if mov.peek() == fru.peek() {
                fru.next();
            } else {
                sna.pop_back();
            }
            sna.push_front(*mov.next().unwrap());
        }
        next_frame().await
    }
}

async fn highscore_menu() {
    let mut scores: ListOfScores = ListOfScores {
        scores_list: Vec::new(),
    };
    match scores.load_scores() {
        Ok(()) => {}
        Err(e) => println!("Error loading {}", e),
    }

    let grid: Vec<Line> = build_grid();
    let mut pos: f32 = 0.0;

    let mut e_pressed: bool;
    if is_key_down(KeyCode::Enter) {
        e_pressed = true;
    } else {
        e_pressed = false;
    }
    clear_input_queue();
    loop {
        clear_background(BLACK);
        draw_grid(&grid);
        scores.draw_highscore();
        draw_text("Return", OFFSET_X + 110.0, OFFSET_Y + 570.0, 60.0, RED);
        draw_circle(OFFSET_X + 80.0, OFFSET_Y + 100.0 + pos, 12.0, RED);

        if is_key_released(KeyCode::Enter) {
            e_pressed = false;
        }
        if is_key_pressed(KeyCode::Down) {
            pos += 90.0;
            if pos > 450.0 {
                pos = 0.0;
            }
        } else if is_key_pressed(KeyCode::Up) {
            pos -= 90.0;
            if pos < 0.0 {
                pos = 450.0;
            }
        } else if is_key_pressed(KeyCode::Escape) {
            return;
        } else if is_key_pressed(KeyCode::Enter) && !e_pressed {
            let n = (pos / 90.0) as usize;
            if n == 5 {
                return;
            } else {
                replay(&scores.scores_list[n]).await;
            }
        }
        next_frame().await
    }
}

async fn main_menu() -> Option<String> {
    let mut new_name = String::new();
    let mut active: u8 = 0;

    let mut e_pressed: bool;
    if is_key_down(KeyCode::Enter) {
        e_pressed = true;
    } else {
        e_pressed = false;
    }

    let grid: Vec<Line> = build_grid();
    let box_width = 720.0 - 2.0 * OFFSET_X - 200.0;
    let box_hight = 720.0 - 2.0 * OFFSET_X - 520.0;
    let box_x = OFFSET_X + 100.0;
    let box_y = OFFSET_Y + 100.0;
    loop {
        clear_background(BLACK);
        draw_grid(&grid);

        draw_rectangle(
            OFFSET_X + 50.0,
            OFFSET_Y + 50.0,
            720.0 - 2.0 * OFFSET_X - 100.0,
            720.0 - 2.0 * OFFSET_X - 100.0,
            DARKGRAY,
        );

        match active {
            0 => {
                draw_rectangle(box_x, box_y, box_width, box_hight, DARKGREEN);
                draw_rectangle(box_x, box_y + 150.0, box_width, box_hight, DARKBLUE);
                draw_rectangle(box_x, box_y + 300.0, box_width, box_hight, DARKBLUE);
            }
            1 => {
                draw_rectangle(box_x, box_y, box_width, box_hight, DARKBLUE);
                draw_rectangle(box_x, box_y + 150.0, box_width, box_hight, DARKGREEN);
                draw_rectangle(box_x, box_y + 300.0, box_width, box_hight, DARKBLUE);
            }
            _ => {
                draw_rectangle(box_x, box_y, box_width, box_hight, DARKBLUE);
                draw_rectangle(box_x, box_y + 150.0, box_width, box_hight, DARKBLUE);
                draw_rectangle(box_x, box_y + 300.0, box_width, box_hight, DARKGREEN);
            }
        }

        if active == 0 {
            if let Some(key) = get_char_pressed() {
                if key.is_alphabetic() && new_name.len() < 11 {
                    new_name.push(key);
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                new_name.pop();
            }
        }

        if new_name.is_empty() {
            draw_text(
                "Name",
                OFFSET_X + 300.0 - 4.0 * 15.0,
                OFFSET_Y + 180.0,
                80.0,
                RED,
            );
        } else {
            draw_text(
                &new_name,
                OFFSET_X + 300.0 - new_name.len() as f32 * 15.0,
                OFFSET_Y + 180.0,
                80.0,
                RED,
            );
        }

        draw_text("Highscores", OFFSET_X + 150.0, OFFSET_Y + 330.0, 80.0, RED);
        draw_text(
            "Quit",
            OFFSET_X + 300.0 - 4.0 * 15.0,
            OFFSET_Y + 480.0,
            80.0,
            RED,
        );

        if is_key_released(KeyCode::Enter) {
            e_pressed = false;
        }

        if is_key_pressed(KeyCode::Enter) && !e_pressed {
            match active {
                0 => return Some(new_name),
                1 => highscore_menu().await,
                _ => return None,
            }
        } else if is_key_pressed(KeyCode::Escape) {
            break;
        } else if is_key_pressed(KeyCode::Up) {
            active = (active + 3 - 1) % 3;
            clear_input_queue();
        } else if is_key_pressed(KeyCode::Down) {
            active = (active + 1 + 3) % 3;
            clear_input_queue();
        }
        next_frame().await
    }
    None
}

async fn new_game() -> Option<(Snake, Fruit)> {
    let new_name: String;

    match main_menu().await {
        Some(x) => new_name = x,
        None => return None,
    }

    let fruit: Fruit = Fruit {
        pos: (10, 16),
        draw_pos: (OFFSET_X + CELL * 16.0, OFFSET_Y + CELL * 10.0),
        pts: 5,
    };

    let snake: Snake = Snake {
        name: new_name,
        snake: VecDeque::from(vec![(29, 16), (30, 16)]),
        head: (28, 16),
        dir: Direction::UP,
        pts: 0,
        moves: Vec::new(),
        fruits: Vec::new(),
    };
    Some((snake, fruit))
}

#[macroquad::main("Snake")]
async fn main() {
    request_new_screen_size(720.0, 720.0);
    let mut last = get_time();

    let mut start: bool = false;
    let mut pause: bool = true;
    let mut highscore: bool = false;

    let mut scores: ListOfScores = ListOfScores {
        scores_list: Vec::new(),
    };
    match scores.load_scores() {
        Ok(()) => println!("Loaded scores!"),
        Err(e) => println!("Error loading {}", e),
    }
    let mut fruit: Fruit;
    let mut snake: Snake;
    match new_game().await {
        Some(x) => (snake, fruit) = x,
        None => return,
    }

    let grid: Vec<Line> = build_grid();

    loop {
        clear_background(BLACK);
        clear_input_queue();
        draw_grid(&grid);
        let txt = format!("Points: {}", snake.pts);
        draw_text(&txt, 270.0, 40.0, 50.0, BLUE);
        snake.draw_snake();
        fruit.draw_fruit();

        if start {
            if is_key_pressed(KeyCode::Enter) {
                start = false;
                match new_game().await {
                    Some(x) => (snake, fruit) = x,
                    None => return,
                }
            }
        } else if is_key_down(KeyCode::Tab) {
            highscore = true;
        } else if is_key_released(KeyCode::Tab) {
            highscore = false;
        } else if is_key_pressed(KeyCode::Enter) {
            pause = !pause;
        } else if is_key_pressed(KeyCode::Escape) {
            start = true;
        } else if !pause && !start {
            if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                snake.turn_snake(Direction::UP);
            } else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                snake.turn_snake(Direction::LEFT);
            } else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                snake.turn_snake(Direction::RIGHT);
            } else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                snake.turn_snake(Direction::DOWN);
            }

            if get_time() - last > TIME {
                last = get_time();
                match snake.move_snake() {
                    None => {
                        println!("Game ended. Your points were {}.", snake.pts);
                        snake.fruits.push(fruit.pos);
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
        } else if highscore {
            scores.draw_highscore();
        } else if pause {
            draw_pause();
        }
        next_frame().await
    }
}
