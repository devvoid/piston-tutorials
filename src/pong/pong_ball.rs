//Takes the pong example and adds a ball to it

extern crate piston_window;

extern crate find_folder;

//Crate that allows the generation of pseudo-random numbers
//Needed for generating the angle a ball travels after colliding with a paddle
extern crate rand;

//Crate that allows sound playing
extern crate ears;

use ears::{Sound, AudioController};
use rand::Rng;
use piston_window::*;

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "Pong",
        [640, 320]
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    let seperator = Texture::from_path(
        &mut window.factory,
        assets.join("pong/seperator.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let paddle = Texture::from_path(
        &mut window.factory,
        assets.join("pong/paddle.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let ball = Texture::from_path(
        &mut window.factory,
        assets.join("pong/ball.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let player_speed: f64 = 150.0;

    let mut left_y: f64 = 120.0;

    let mut left_moving_up: bool = false;
    let mut left_moving_down: bool = false;

    let mut right_y: f64 = 120.0;

    let mut right_moving_up: bool = false;
    let mut right_moving_down: bool = false;

    //Ball start position, at the center of the screen
    let mut ball_pos_x: f64 = 316.0;
    let mut ball_pos_y: f64 = 156.0;

    //Ball speed in pixels per second
    let ball_speed: f64 = 200.0;

    //The angle the ball moves in
    //gen_range generates a number between 0 and 360
    let mut ball_angle: f64 = rand::thread_rng().gen_range(0.0, 360.0);

    //Whether or not the ball has bounced after making contact with a wall/paddle
    //Used to prevent the ball from getting stuck if delta time just so happens to put it inside of a collidable object
    let mut ball_has_bounced: bool = false;

    //Scores of both players
    let mut player_one_score: u32 = 0;
    let mut player_two_score: u32 = 0;

    //Ball sound effect
    let mut snd = Sound::new(assets.join("pong/ball.wav").to_str().unwrap()).unwrap();

    //Load a font, for text rendering.
    let mut font = Glyphs::new(
        assets.join("pong/font.ttf"), //Path to the font. I can't include a font in the tutorial files for copyright reasons, so you'll have to find your own
        window.factory.clone(), //For some reason, this function needs to take ownership of factory to make the font, so here I clone one from the window
        TextureSettings::new() //Just like the sprites from last tutorial, you can use a TextureSettings struct to set more advanced options
    ).unwrap();

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
                if left_y > 0.0 {
                    let new_left_position = left_y - (player_speed * u.dt);

                    if new_left_position <= 0.0 {
                        left_y = 0.0;
                    }
                    else {
                        left_y = new_left_position;
                    }
                }
            }

            if left_moving_down {
                if left_y < 240.0 {
                    let new_left_position = left_y + (player_speed * u.dt);

                    if new_left_position >= 240.0 {
                        left_y = 240.0;
                    }
                    else {
                        left_y = new_left_position;
                    }
                }
            }

            if right_moving_up {
                if right_y > 0.0 {
                    let new_right_position = right_y - (player_speed * u.dt);

                    if new_right_position <= 0.0 {
                        right_y = 0.0;
                    }
                    else {
                        right_y = new_right_position;
                    }
                }
            }

            if right_moving_down {
                if right_y < 240.0 {
                    let new_right_position = right_y + (player_speed * u.dt);

                    if new_right_position >= 240.0 {
                        right_y = 240.0;
                    }
                    else {
                        right_y = new_right_position;
                    }
                }
            }

            //If the ball is off-screen to the right, plus 20 pixels as a buffer to make sure the ball has gone a little ways off-screen first,
            //reset the ball and give player 1 a point
            if ball_pos_x > 660.0 {
                ball_pos_x = 316.0;
                ball_pos_y = 156.0;
                ball_angle = rand::thread_rng().gen_range(0.0, 360.0);

                player_one_score += 1;
            }

            //Same as above, but with the left screen and player 2
            if ball_pos_x < -20.0 {
                ball_pos_x = 316.0;
                ball_pos_y = 156.0;
                ball_angle = rand::thread_rng().gen_range(0.0, 360.0);

                player_two_score += 1;
            }

            //If the ball tries to go off the top of the screen, make it bounce off
            if ball_pos_y <= 0.0 && ball_has_bounced == false {
                //Invert the angle to make the ball bounce the other way
                ball_angle = -ball_angle;

                //Play the ball sound
                snd.play();

                ball_has_bounced = true;
            }

            //Do the same for the bottom of the screen
            if ball_pos_y >= 312.0 && ball_has_bounced == false {
                ball_angle = -ball_angle;

                snd.play();

                ball_has_bounced = true;
            }

            //If the ball isn't touching anything, turn ball_has_bounced back to false
            if ball_pos_y > 0.0 && ball_pos_y < 312.0 {
                ball_has_bounced = false;
            }

            //Handle ball colliding with left paddle
            if left_y < ball_pos_y && (left_y + 80.0) > ball_pos_y && ball_pos_x > 50.0 && ball_pos_x < 60.0 {
                ball_angle = rand::thread_rng().gen_range(110.0, 250.0);

                snd.play();
            }

            //Handle ball colliding with right paddle
            if right_y < ball_pos_y && (right_y + 80.0) > ball_pos_y && ball_pos_x + 8.0 > 590.0 && ball_pos_x + 8.0 < 600.0 {
                ball_angle = -rand::thread_rng().gen_range(110.0, 250.0);

                snd.play();
            } 

            ball_pos_x += ball_angle.cos() * ball_speed * u.dt;
            ball_pos_y += ball_angle.sin() * ball_speed * u.dt;

        }

        if let Some(_r) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                image(&seperator, c.transform.trans(318.0, 0.0), g);

                image(&paddle, c.transform.trans(50.0, left_y), g);

                image(&paddle, c.transform.trans(590.0, right_y), g);

                //Draw the ball
                image(&ball, c.transform.trans(ball_pos_x, ball_pos_y), g);

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