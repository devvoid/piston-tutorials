///Feature name: hello_window
/// A simple example, showing how to open a window, using the default PistonWindow backend.


//piston_window contains pretty much all Piston functions needed for simple window creation, drawing, everything like that.
extern crate piston_window;
use piston_window::*;

fn main() {
    //Create a PistonWindow, which is used for drawing.
    //Piston has various other backends you could use for window creation instead, such as GLFW or SDL2, but I'll go with the default for simplicity's sake.
    //You create a window by first making a WindowSettings struct, setting all the fields the way you want them, then calling .build() on it.
    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!", //The name shown above the window.
            [200, 200] //Default window size, in pixels.
        )
        .opengl(OpenGL::V3_2) //OpenGL version to use. You can change this to OpenGL::V2_1 if your graphics card does not support OpenGL 3.2.
        .exit_on_esc(true) //If true, the window will close if the escape key is pressed.
        .build() //Use these settings to build the window.
        .unwrap();

    //This is the window's event loop.
    //This loop will run for as long as the window is open, updating variable "e" to represent the event currently being handled.
    while let Some(e) = window.next() {
        //If the window tries to draw something, run this code.
        if let Some(_r) = e.render_args() {
            //draw_2d takes a reference to the event, and a closure used as the drawing code.
            //The closure captures two variables: c, which is used for transformations,
            //and g, which is the current graphics backend.
            window.draw_2d(&e, |c, g| {
                //Clear the window to black. Takes an array of four floats to form an RGBA color, and g to know which graphics backend to clear.
                //[0.0; 4] is just a shorthand for [0.0, 0.0, 0.0, 0.0].
                clear([0.0; 4], g);

                //Draw a yellow rectangle in the middle of the screen.
                rectangle([1.0, 1.0, 0.0, 1.0], //Color, in RGBA.
                    [75.0, 75.0, 50.0, 50.0], //X position of the top left, Y position of the top left, width, and height of the rectangle, in that order.
                    c.transform, //Transformation of the window. This is the top-left of the screen unless you call functions on it.
                    g); //Graphics backend to use.
            });
        }
    }
}