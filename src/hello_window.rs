//Covers base window creation
//If you can compile and run this example, you should be good to go

extern crate piston_window;

use piston_window::*;

fn main() {
    //OpenGL version to use. You can change this to OpenGL::V2_1 if your graphics card doesn't support OpenGL 3.2
    let opengl = OpenGL::V3_2;

    //Create a PistonWindow, which is used for drawing.
    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!", //The name shown above the window.
            [200, 200] //Default window size, in pixels.
        )
        .opengl(opengl) //OpenGL version to use.
        .exit_on_esc(true) //If true, the window will close if the escape key is pressed.
        .build() //Use these settings to build the window.
        .unwrap();

    //This is the window's event loop.
    //This loop will run for as long as the window is open, updating variable "e" to represent the event currently being handled.
    while let Some(e) = window.next() {
        //Since there's nothing going on here, just draw regardless of event type.
        //draw_2d takes a reference to the event, and a closure used as the drawing code.
        //THe closure captures two variables: _c, which is used for transformations (not needed for this example, so marked with an underscore to show it's unused), and g, which is the current graphics backend.
        window.draw_2d(&e, |_c, g| {
            //Clear the window to black. Takes an array of four floats to form an RGBA color, and g to know which graphics backend to clear.
            //[0.0; 4] is just a shorthand for [0.0, 0.0, 0.0, 0.0].
            clear([0.0; 4], g);
        });
    }
}