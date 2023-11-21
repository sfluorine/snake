use raylib::ffi::IsKeyPressed;
use raylib::prelude::*;
use std::ops::Add;

const BAR_SIZE: i32 = 35;
const WINDOW_WIDTH: i32 = BAR_SIZE * 18;
const WINDOW_HEIGHT: i32 = BAR_SIZE * 14;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_horizontal(direction: &Direction) -> bool {
    match direction {
        Direction::Left | Direction::Right => true,
        _ => false,
    }
}

fn is_vertical(direction: &Direction) -> bool {
    match direction {
        Direction::Up | Direction::Down => true,
        _ => false,
    }
}

fn draw_grids(dh: &mut RaylibDrawHandle) {
    for y in 1..=14 {
        dh.draw_line(0, BAR_SIZE * y, WINDOW_WIDTH, BAR_SIZE * y, Color::RAYWHITE);

        for x in 1..=18 {
            dh.draw_line(BAR_SIZE * x, 0, BAR_SIZE * x, WINDOW_WIDTH, Color::RAYWHITE);
        }
    }
}

fn draw_snakes(dh: &mut RaylibDrawHandle, snakes: &Vec<Vector2>) {
    for snake in snakes {
        dh.draw_rectangle_v(
            snake,
            Vector2::new(BAR_SIZE as f32, BAR_SIZE as f32),
            Color::GREEN,
        );
    }
}

// TODO: check if head collide with body.
fn update_snakes(
    snakes: &mut Vec<Vector2>,
    snake_vel: &Vector2,
    timer: &mut i32,
    game_over: &mut bool,
) {
    if *timer == 0 {
        *timer = 42;

        for n in (0..snakes.len()).rev() {
            if n != 0 {
                snakes[n] = snakes[n - 1];
            }
        }

        snakes[0] = snakes[0].add(*snake_vel);

        if snakes[0].x as i32 >= WINDOW_WIDTH || snakes[0].x < 0.0 {
            *game_over = true;
        }

        if snakes[0].y as i32 >= WINDOW_HEIGHT || snakes[0].y < 0.0 {
            *game_over = true;
        }
    }
}

fn draw_food(dh: &mut RaylibDrawHandle, food_pos: &Vector2) {
    dh.draw_circle_v(
        Vector2::new(
            food_pos.x + BAR_SIZE as f32 / 2.0,
            food_pos.y + BAR_SIZE as f32 / 2.0,
        ),
        BAR_SIZE as f32 / 2.0,
        Color::RED,
    );
}

fn update_food(food_pos: &mut Vector2, snakes: &mut Vec<Vector2>) {
    if check_collision_point_circle(snakes[0], *food_pos, BAR_SIZE as f32 / 2.0) {
        let mut new_x: i32 = get_random_value::<i32>(0, 17) * BAR_SIZE;
        let mut new_y: i32 = get_random_value::<i32>(0, 13) * BAR_SIZE;

        for snake in &mut *snakes {
            while check_collision_point_circle(
                *snake,
                Vector2::new(new_x as f32, new_y as f32),
                BAR_SIZE as f32 / 2.0,
            ) {
                new_x = get_random_value::<i32>(0, 17) * BAR_SIZE;
                new_y = get_random_value::<i32>(0, 13) * BAR_SIZE;
            }
        }

        food_pos.x = new_x as f32;
        food_pos.y = new_y as f32;

        snakes.push(snakes[snakes.len() - 1]);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("snake")
        .vsync()
        .build();

    let mut snakes = vec![Vector2::new(0.0, 0.0)];

    let mut snake_vel = Vector2::new(0.0, BAR_SIZE as f32);
    let mut snake_dir = Direction::Down;
    let mut food_pos = Vector2::new(0.0, 0.0);
    let mut timer: i32 = 0;
    let mut game_over: bool = false;

    while !rl.window_should_close() {
        let mut dh = rl.begin_drawing(&thread);
        dh.clear_background(Color::BLACK);

        if !game_over {
            update_food(&mut food_pos, &mut snakes);
            draw_snakes(&mut dh, &snakes);
            draw_grids(&mut dh);
            draw_food(&mut dh, &food_pos);
            update_snakes(&mut snakes, &snake_vel, &mut timer, &mut game_over);

            unsafe {
                if !is_horizontal(&snake_dir) && IsKeyPressed(KeyboardKey::KEY_LEFT as i32) {
                    snake_dir = Direction::Left;
                    snake_vel.x = -BAR_SIZE as f32;
                    snake_vel.y = 0.0;
                } else if !is_horizontal(&snake_dir) && IsKeyPressed(KeyboardKey::KEY_RIGHT as i32)
                {
                    snake_dir = Direction::Right;
                    snake_vel.x = BAR_SIZE as f32;
                    snake_vel.y = 0.0;
                } else if !is_vertical(&snake_dir) && IsKeyPressed(KeyboardKey::KEY_UP as i32) {
                    snake_dir = Direction::Up;
                    snake_vel.x = 0.0;
                    snake_vel.y = -BAR_SIZE as f32;
                } else if !is_vertical(&snake_dir) && IsKeyPressed(KeyboardKey::KEY_DOWN as i32) {
                    snake_dir = Direction::Down;
                    snake_vel.x = 0.0;
                    snake_vel.y = BAR_SIZE as f32;
                }
            }
        } else {
            dh.draw_text(
                "GAME OVER!",
                WINDOW_WIDTH / 2 - 25 * 3,
                WINDOW_HEIGHT / 2 - 25,
                25,
                Color::WHITE,
            );
        }

        timer -= 1;
    }
}
