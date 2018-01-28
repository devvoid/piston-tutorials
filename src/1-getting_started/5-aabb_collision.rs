///Feature name: aabb_collision
/// An example showing Axis-Aligned Bounding Box (AABB) collision.

extern crate piston_window;
use piston_window::*;

//Here's a struct to represent the AABB rectangle.
struct Rect {
    x: f64, //X position of the top left corner.
    y: f64, //Y position of the top left corner.
    w: f64, //Width of the rectangle.
    h: f64, //Height of the rectangle.
}

impl Rect {
    //Creates and returns a new rectangle.
    pub fn new(new_x: f64, new_y: f64, new_w: f64, new_h: f64) -> Rect {
        Rect { x: new_x, y: new_y, w: new_w, h: new_h }
    }

    //Returns true if the rectangles are intersecting, false otherwise.
    pub fn intersects(&self, other: &Rect) -> bool {
        if self.x < other.x + other.w &&    //Top left of this rectangle is farther left than the top right of the other.
        self.x + self.w > other.x &&        //Top right of this rectangle is farther right than the top left of the other.
        self.y < other.y + other.h &&       //Top left of this rectangle is farther up than the bottom left of the other.
        self.h + self.y > other.y {         //Bottom left of this rectangle is farther town than the top left of the other.
            return true;
        }
        
        false
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!",
            [320, 320] //Make the window larger to give a bit more space to test it out.
        )
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut rectangle_1 = Rect::new(75.0, 75.0, 50.0, 50.0);
    let rectangle_2 = Rect::new(75.0, 75.0, 150.0, 150.0);

    let mut moving_up = false;
    let mut moving_down = false;
    let mut moving_left = false;
    let mut moving_right = false;

    let mut speed = 30.0;

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
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

                Key::Space | Key::R => {
                    speed = 60.0;
                }

                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
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
            if moving_up && moving_down {
                moving_up = false;
                moving_down = false;
            }

            if moving_left && moving_right {
                moving_left = false;
                moving_right = false;
            }

            if moving_up {
                //Note that this has been updated.
                rectangle_1.y -= speed * u.dt;
            }

            if moving_down {
                rectangle_1.y += speed * u.dt;
            }                       

            if moving_left {
                rectangle_1.x -= speed * u.dt;
            }

            if moving_right {
                rectangle_1.x += speed * u.dt;
            }
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                //Draw rectangle_2 as dark red.
                rectangle([0.5, 0.0, 0.0, 1.0],
                      [rectangle_2.x, rectangle_2.y, rectangle_2.w, rectangle_2.h],
                      c.transform,
                      g);

                //Rect_1 will be blue if not touching rectangle_2, and yellow otherwise.
                if rectangle_1.intersects(&rectangle_2) {
                    rectangle([0.0, 0.0, 1.0, 1.0],
                        [rectangle_1.x, rectangle_1.y, rectangle_1.w, rectangle_1.h],
                        c.transform,
                        g);
                } else {
                    rectangle([1.0, 1.0, 0.0, 1.0],
                        [rectangle_1.x, rectangle_1.y, rectangle_1.w, rectangle_1.h],
                        c.transform,
                        g);
                }
            });
        }
    }
}