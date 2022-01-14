use std::collections::LinkedList;
use macroquad::prelude::*;

type Point = (i32, i32);

const WIDTH: i32 = 640;
const HEIGHT: i32 = 640;
const CELL_SIDE: f32 = 16.0;
const SQUARE_COUNT_SIDE: i32 = WIDTH/(CELL_SIDE as i32);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    direction: Point
}

#[macroquad::main(window_configuration)]
async fn main() {
    let mut is_game_over = true;
    let mut last_update = get_time();
    let refresh_rate = 0.1;

    let mut apple: Point = get_apple();
    let mut snake = Snake {
        head: (2, 0),
        direction: (1, 0),
        body: LinkedList::from([(1,0)]),
    };
    let mut score = 0;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    loop {
        if is_game_over {
            draw_lost_screen(score);

            if is_key_down(KeyCode::Enter) {
                score = 0;
                is_game_over = false;
            }
        }
        else {
            if is_key_down(KeyCode::Right) && snake.direction != left {
                snake.direction = right;
            }
            else if is_key_down(KeyCode::Left) && snake.direction != right {
                snake.direction = left;
            }
            else if is_key_down(KeyCode::Up) && snake.direction != down {
                snake.direction = up;
            }
            else if is_key_down(KeyCode::Down) && snake.direction != up {
                snake.direction = down;
            }

            if get_time() - last_update > refresh_rate {
                last_update = get_time();

                snake.body.push_front(snake.head);
                snake.head = (snake.head.0 + snake.direction.0, snake.head.1 + snake.direction.1);

                if snake.head == apple{
                    apple = get_apple();
                    score += 1;
                }
                else { snake.body.pop_back(); }

                //Collision
                for (x, y) in &snake.body {
                    if *x == snake.head.0 && *y == snake.head.1 {
                        is_game_over = true;
                    }
                }
            }

            draw_game_screen(apple, &snake);
        }

        next_frame().await
    }
}

fn draw_game_screen(apple: Point, snake: &Snake) -> () {
    clear_background(WHITE);

    draw_rectangle(apple.0 as f32 * CELL_SIDE, apple.1 as f32 * CELL_SIDE, CELL_SIDE, CELL_SIDE, RED);
    draw_rectangle(snake.head.0 as f32 * CELL_SIDE, snake.head.1 as f32 * CELL_SIDE, CELL_SIDE, CELL_SIDE, DARKGREEN);

    for (x, y) in &snake.body {
        draw_rectangle(*x as f32 * CELL_SIDE, *y as f32 * CELL_SIDE, CELL_SIDE, CELL_SIDE, LIME);
    }
}

fn draw_lost_screen(score: i32) -> () {
    clear_background(WHITE);
    
    draw_centered(format!("Game over. Your score was {}", score).as_str(), 30.0, 0.0);
    draw_centered("Press enter to restart", 30.0, 30.0);
}

fn get_apple() -> Point {
    return (rand::gen_range(0, SQUARE_COUNT_SIDE), rand::gen_range(0, SQUARE_COUNT_SIDE));
}

fn draw_centered(text: &str, font_size: f32, offset_y: f32) -> () {
    let text_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(text, screen_width() / 2. - text_size.width / 2., screen_height() / 2. - text_size.height / 2. + offset_y, font_size, BLACK);
}

fn window_configuration() -> Conf {
    Conf {
        window_title: "SimpleSnake".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}