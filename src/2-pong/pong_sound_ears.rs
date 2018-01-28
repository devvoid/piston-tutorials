///Feature name: pong_sound_ears
/// The pong example, with added sound using Ears. piston_music-based version of this tutorial coming soon.

extern crate piston_window;

extern crate find_folder;

//Crate that allows the generation of pseudo-random numbers
//Needed for generating the angle a ball travels after colliding with a paddle
extern crate rand;

extern crate ears;

use rand::Rng;
use piston_window::*;
use ears::{Sound, Music, AudioController};

struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    pub fn new(new_x: f64, new_y: f64, new_w: f64, new_h: f64) -> Rect {
        Rect { x: new_x, y: new_y, w: new_w, h: new_h }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        if self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.h + self.y > other.y {
            return true;
        }
        
        false
    }
}

//Speeds for the players and balls, in pixels per second.
const PLAYER_SPEED: f64 = 150.0;
const BALL_SPEED: f64 = 200.0;

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Pong", [640, 320])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let seperator = Texture::from_path(
        &mut window.factory,
        assets.join("pong/seperator.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("Cannot find pong/seperator.png in the assets folder");

    //Left paddle
    let mut left_paddle = Rect::new(50.0, 120.0, 10.0, 80.0);
    let mut left_moving_up: bool = false;
    let mut left_moving_down: bool = false;

    //Right paddle
    let mut right_paddle = Rect::new(590.0, 120.0, 10.0, 80.0);
    let mut right_moving_up: bool = false;
    let mut right_moving_down: bool = false;

    //The ball
    let mut ball = Rect::new(316.0, 156.0, 8.0, 8.0);

    //The angle the ball moves in
    //gen_range generates a number between 0 and 360
    let mut ball_angle: f64 = rand::thread_rng().gen_range(0.0, 360.0);

    //Scores of both players
    let mut player_one_score: u32 = 0;
    let mut player_two_score: u32 = 0;

    //Load a font, for text rendering.
    let mut font = Glyphs::new(
        assets.join("coders_crux.ttf"),
        window.factory.clone(), //For some reason, this function needs to take ownership of factory to make the font, so here I clone one from the window
        TextureSettings::new() //Just like the sprites from last tutorial, you can use a TextureSettings struct to set more advanced options
    ).unwrap();

    //Load the sound.
    let mut snd = Sound::new( &assets.join("pong/beep.ogg").to_str().unwrap() ).expect("Unable to find pong/beep.ogg in the assets folder");

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => {left_moving_up = true;}
                Key::S => {left_moving_down = true;}
                Key::I => {right_moving_up = true;}
                Key::K => {right_moving_down = true;}
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::W => {left_moving_up = false;}
                Key::S => {left_moving_down = false;}
                Key::I => {right_moving_up = false;}
                Key::K => {right_moving_down = false;}
                _ => {}
            }
        }
        
        if let Some(u) = e.update_args() {
            if left_moving_up && left_moving_down {
                left_moving_up = false;
                left_moving_down = false;
            }

            if right_moving_up && right_moving_down {
                left_moving_up = false;
                left_moving_down = false;
            }

            if left_moving_up {
                if left_paddle.y > 0.0 {
                    let new_left_position = left_paddle.y - (PLAYER_SPEED * u.dt);

                    if new_left_position <= 0.0 {
                        left_paddle.y = 0.0;
                    }
                    else {
                        left_paddle.y = new_left_position;
                    }
                }
            }

            if left_moving_down {
                if left_paddle.y < 240.0 {
                    let new_left_position = left_paddle.y + (PLAYER_SPEED * u.dt);

                    if new_left_position >= 240.0 {
                        left_paddle.y = 240.0;
                    }
                    else {
                        left_paddle.y = new_left_position;
                    }
                }
            }

            if right_moving_up {
                if right_paddle.y > 0.0 {
                    let new_right_position = right_paddle.y - (PLAYER_SPEED * u.dt);

                    if new_right_position <= 0.0 {
                        right_paddle.y = 0.0;
                    }
                    else {
                        right_paddle.y = new_right_position;
                    }
                }
            }

            if right_moving_down {
                if right_paddle.y < 240.0 {
                    let new_right_position = right_paddle.y + (PLAYER_SPEED * u.dt);

                    if new_right_position >= 240.0 {
                        right_paddle.y = 240.0;
                    }
                    else {
                        right_paddle.y = new_right_position;
                    }
                }
            }

            //If the ball is off-screen to the right, plus 20 pixels as a buffer to make sure the ball has gone a little ways off-screen first,
            //reset the ball and give player 1 a point
            if ball.x > 660.0 {
                ball.x = 316.0;
                ball.y = 156.0;
                ball_angle = rand::thread_rng().gen_range(0.0, 360.0);

                player_one_score += 1;
            }

            //Same as above, but with the left side of the screen and player 2
            if ball.x < -20.0 {
                ball.x = 316.0;
                ball.y = 156.0;
                ball_angle = rand::thread_rng().gen_range(0.0, 360.0);

                player_two_score += 1;
            }

            //If the ball tries to go off the top of the screen, make it bounce off
            if ball.y <= 0.0 {
                //Invert the angle to make the ball bounce the other way
                ball_angle = -ball_angle;

                //Set the ball to just barely under the screen, so it won't jitter there if delta time puts it a decent ways past the wall
                ball.y = 0.01;

                snd.play();
            }

            //Do the same for the bottom of the screen
            if ball.y >= 312.0 {
                ball_angle = -ball_angle;

                ball.y = 311.99;

                snd.play();
            }

            //Handle ball colliding with left paddle
            if ball.intersects(&left_paddle) {
                //Generate an angle that'll send it towards the right, plus a small buffer so it won't go vertically
                ball_angle = rand::thread_rng().gen_range(110.0, 250.0);

                //Set the ball to just right of the paddle so it won't immediately collide again.
                ball.x = left_paddle.x + left_paddle.w + 0.01;

                snd.play();
            }

            //Handle ball colliding with right paddle
            if ball.intersects(&right_paddle) {
                //Same as the left, but negative so it boinces left instead of right
                ball_angle = 360.0 - rand::thread_rng().gen_range(110.0, 250.0);

                ball.x = right_paddle.x - ball.w - 0.01;

                snd.play();
            } 

            //Use cos and sin to convert a 360-degree angle to X and Y coordinates.
            ball.x += ball_angle.cos() * BALL_SPEED * u.dt;
            ball.y += ball_angle.sin() * BALL_SPEED * u.dt;

        }

        if let Some(_r) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                image(&seperator, c.transform.trans(318.0, 0.0), g);

                rectangle([1.0; 4],
                      [left_paddle.x, left_paddle.y, left_paddle.w, left_paddle.h],
                      c.transform,
                      g);

                rectangle([1.0; 4],
                      [right_paddle.x, right_paddle.y, right_paddle.w, right_paddle.h],
                      c.transform,
                      g);

                //Draw the ball
                rectangle([1.0; 4],
                    [ball.x, ball.y, ball.w, ball.h],
                    c.transform,
                    g);

                text::Text::new_color([1.0; 4], 32).draw(
                    &player_one_score.to_string(),
                    &mut font,
                    &c.draw_state,
                    c.transform.trans(150.0, 50.0),
                    g
                ).unwrap();

                text::Text::new_color([1.0; 4], 32).draw(
                    &player_two_score.to_string(),
                    &mut font,
                    &c.draw_state,
                    c.transform.trans(490.0, 50.0),
                    g
                ).unwrap();
            });
        }
    }
}