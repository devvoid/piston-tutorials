//Axis-Aligned Bounding Box collision detection
//Fairly standard implimentation, nothing too special here

extern crate piston_window;

use piston_window::*;

struct Rect {
    x: f64, //X position of the top left corner
    y: f64, //Y position of the top left corner
    w: f64, //Width of the rectangle
    h: f64, //Height of the rectangle
}

impl Rect {
    //Creates and returns a new rectangle
    pub fn new(new_x: f64, new_y: f64, new_w: f64, new_h: f64) -> Rect {
        Rect { x: new_x, y: new_y, w: new_w, h: new_h }
    }

    //Returns true if the rectangles are intersecting, false otherwise.
    pub fn intersects(&self, other: &Rect) -> bool {
        if self.x < other.x + other.w &&    //Top left of this rectangle is farther left than the top right of the other
        self.x + self.w > other.x &&        //Top right of this rectangle is farther right than the top left of the other
        self.y < other.y + other.h &&       //Top left of this rectangle is farther up than the bottom left of the other
        self.h + self.y > other.y {         //Bottom left of this rectangle is farther town than the top left of the other
            return true;
        }
        
        false
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!",
            [320, 320]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //Create two rectangles to try it out.
    let rect_1 = Rect::new(100.0, 100.0, 120.0, 120.0);
    let mut rect_2 = Rect::new(50.0, 50.0, 20.0, 20.0);

    while let Some(e) = window.next() {

        //This is a very basic implimentation of WASD-movement
        //It's very jittery and doesn't work very well, but it's enough for now
        //Check the pong examples for a smoother and more functional example of keyboard input and movement
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::W => { rect_2.y -= 5.0; }
                Key::S => { rect_2.y += 5.0; }
                Key::A => { rect_2.x -= 5.0; }
                Key::D => { rect_2.x += 5.0; }
                _ => {}
            }
        }

        if let Some(_u) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                //rect_1 will be red
                rectangle([1.0, 0.0, 0.0, 1.0],
                      [rect_1.x, rect_1.y, rect_1.w, rect_1.h],
                      c.transform,
                      g);

                //rect_2 will be green if it's colliding with rect_1, and blue otherwise.
                if rect_1.intersects(&rect_2) {
                    rectangle([0.0, 1.0, 0.0, 1.0],
                          [rect_2.x, rect_2.y, rect_2.w, rect_2.h],
                          c.transform,
                          g);
                } else {
                    rectangle([0.0, 0.0, 1.0, 1.0],
                          [rect_2.x, rect_2.y, rect_2.w, rect_2.h],
                          c.transform,
                          g);
                }
        });
        }
    }
}
