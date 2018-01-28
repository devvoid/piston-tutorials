///Feature name: keyboard
/// An example showing how keyboard input can be handled by making the rectangle from the past few examples moveable with WASD.


extern crate piston_window;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!",
            [200, 200]
        )
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rectangle_x = 75.0;
    let mut rectangle_y = 75.0;

    //These will be explained in just a minute.
    let mut moving_up = false;
    let mut moving_down = false;
    let mut moving_left = false;
    let mut moving_right = false;

    //Speed of the rectangle in pixels.
    let mut speed = 30.0;

    while let Some(e) = window.next() {
        //Triggers when a keyboard key is pressed, updating the key variable to the key pressed.
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                //All available keycodes are defined in Piston's Key enum.

                //Note that when you first press a key, there's a sort of "stutter" to it that I have trouble explaining.
                //Hold down one key in any text editor and see how there's a delay from first press to spamming presses.
                //That happens in Piston too; if you directly changed rectangle_x and rectangle_y here, the same effect
                //happen with the rectangle, which isn't what anyone wants.
                //Instead, you set the bools to true when pressed here, to false in the next statement, and actually move them
                //in the update loop.

                //You don't have access to the delta time variable from here anyway, so you'd have to do it in the update loop either way.
                Key::W => {
                    moving_up = true;
                },

                Key::S => {
                    moving_down = true;
                }

                Key::A => {
                    moving_left = true;
                },

                Key::D => {
                    moving_right = true;
                },

                //You can check if either key is held using a bitwise OR operation.
                //So far as I can figure out so far, you can't use a bitwise AND to check if both are being held. I could be wrong though,
                //but in my tests, it never worked.
                //In this case, your speed will be doubled while holding down either Space or R.
                Key::Space | Key::R => {
                    speed = 60.0;
                }

                _ => {}
            }
        }

        //Triggers when a keyboard key is released, updating the key variable to the key released.
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                //Set all booleans back to false and speed back to 30 when their respective keys are pressed.
                Key::W => {
                    moving_up = false;
                },

                Key::S => {
                    moving_down = false;
                }

                Key::A => {
                    moving_left = false;
                },

                Key::D => {
                    moving_right = false;
                },

                Key::Space | Key::R => {
                    speed = 30.0;
                }

                _ => {}
            }
        }

        if let Some(u) = e.update_args() {
            //If the rectangle tries to move both up and down, it will jitter in place.
            //If that happens, this will set them both to false.
            if moving_up && moving_down {
                moving_up = false;
                moving_down = false;
            }

            if moving_left && moving_right {
                moving_left = false;
                moving_right = false;
            }

            //Finally, it's time to actually move the rectangle.
            //Remember that 0,0 is the top left of the window, so to move up, you'd subtract from the Y position.
            if moving_up {
                rectangle_y -= speed * u.dt;
            }

            if moving_down {
                rectangle_y += speed * u.dt;
            }                       

            if moving_left {
                rectangle_x -= speed * u.dt;
            }

            if moving_right {
                rectangle_x += speed * u.dt;
            }
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                rectangle([1.0, 1.0, 0.0, 1.0],
                    [rectangle_x, rectangle_y, 50.0, 50.0],
                    c.transform,
                    g);
            });
        }
    }
}