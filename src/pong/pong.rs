//Load two paddles and a seperator from textures
//Also introduces keyboard input, and a more advanced game loop
//The ball will be introduced in the pong_ball example, since it's more complicated than what you see here and I don't want to introduce too many new concepts too fast

extern crate piston_window;

//Allows you to search for folders instead of having to specify an exact path
//Very helpful while testing because searching for folders is relative to the terminal the program is being run from
//so this makes sure you don't have to copy any assets to the build folder while still testing
extern crate find_folder;

use piston_window::*;

fn main() {
    //Search for the assets folder and save its path
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    
    //Create window, exactly like in hello_window
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "Pong",
        [640, 320]
    ).opengl(opengl).exit_on_esc(true).build().unwrap();

    //Load the texture for the seperator in the middle of the field
    let seperator = Texture::from_path(
        &mut window.factory, //OpenGL context to load the texture in
        assets.join("pong/seperator.png"), //Path to the texture
        Flip::None, //Flipping mode for the texture
        &TextureSettings::new() //Custom settings for the texture can be passed in using a TextureSettings struct, but that's not needed here, so pass in a blank one
    ).unwrap();

    //Load the texture used for the paddles
    let paddle = Texture::from_path(
        &mut window.factory,
        assets.join("pong/paddle.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    //Speed in pixels that the paddles move per second
    let player_speed: f64 = 150.0;

    //Y position for the left paddle
    //Starts at 120, around the center of the screen
    let mut left_y: f64 = 120.0;

    //Movement variables for the left paddle
    let mut left_moving_up: bool = false;
    let mut left_moving_down: bool = false;

    //Y position for the right paddle
    let mut right_y: f64 = 120.0;

    //Movement variables for the right paddle
    let mut right_moving_up: bool = false;
    let mut right_moving_down: bool = false;

    //This loop is different from the one in hello_world - it properly responds to different kinds of events, instead of just always drawing regardless of event
    while let Some(e) = window.next() {
        
        //e.press_args is used for handling keyboard presses
        //Sets bools to true. Actual movement is handled in the update event
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => {left_moving_up = true;}
                Key::S => {left_moving_down = true;}
                Key::I => {right_moving_up = true;}
                Key::K => {right_moving_down = true;}
                _ => {}
            }
        }

        //e.release_args is used for handling releasing keyboard keys
        //Sets bools to false
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::W => {left_moving_up = false;}
                Key::S => {left_moving_down = false;}
                Key::I => {right_moving_up = false;}
                Key::K => {right_moving_down = false;}
                _ => {}
            }
        }
        
        //e.update_args is a generic update
        if let Some(u) = e.update_args() {
            //If the paddle tries to move up and down at the same time, it'll jitter in place
            //This just sets them both to false if it tries to do that
            if left_moving_up && left_moving_down {
                left_moving_up = false;
                left_moving_down = false;
            }

            if right_moving_up && right_moving_down {
                left_moving_up = false;
                left_moving_down = false;
            }

            if left_moving_up {
                //Y is 0 at the top of the screen and decreases as it goes further down the screen
                //This checks if the paddle is beneath the top of the screen,
                //so that the paddle can't be moved above the screen
                if left_y > 0.0 {
                    //u.dt is delta time - the amount of time that has passed since the last frame
                    //This makes the movement variables resolution-independant. The game will run at the same speed if it's running at 30 or 60 fps
                    let new_left_position = left_y - (player_speed * u.dt);

                    //If the new position would be above the screen, put the paddle right at the top instead
                    if new_left_position <= 0.0 {
                        left_y = 0.0;
                    }

                    //If it wouldn't be above the screen, move it to the proper space
                    else {
                        left_y = new_left_position;
                    }
                }
            }

            if left_moving_down {
                //Same as above, but with the bottom of the screen instead of the top
                //Coordinates for the sprite start at the top-left, so 240 is the height of the window minus the height of the sprite,
                //which is when the sprite would visually look like it's touching the bottom.
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

            //This is exactly the same as the version handling the left.
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
        }

        if let Some(_r) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                //Draws the image
                //c.transform is the default place to draw a texture. By default, it points to (0, 0)
                //Passing in c.transform by itself will put the texture's top-left corner at the top-left of the window
                //the trans function will translate it by the coordinates given
                //In this case, the image is translated by half the window width, minus half the texture width
                //This puts the image in the center of the screen
                image(&seperator, c.transform.trans(318.0, 0.0), g);

                //Draw the left paddle
                image(&paddle, c.transform.trans(50.0, left_y), g);

                //Draw the right paddle
                image(&paddle, c.transform.trans(590.0, right_y), g);
            });
        }
    }
}