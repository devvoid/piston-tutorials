///Feature name: event_loop
/// Example of an event loop. This code makes the yellow rectangle from the previous example move to the right, and then wrap arround after going offscreen.


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

    //The rectangle's X position.
    let mut rectangle_x = 75.0;

    while let Some(e) = window.next() {
        //update_args() refers to a generic update, not responding to keyboard input or anything of the like.
        //This block will run once a frame.
        if let Some(u) = e.update_args() {
            //u.dt is an f64. It's short for "delta time"
            //Delta time is just a measurement of the number of seconds between this frame and the one that came before it
            //Since the update code runs once a frame, if you setup the code to just add 30, and the game ran at 30FPS,
            //the rectangle_x would move 900 pixels per second. But, if the framerate was then increased to 60, it would then move 1800 pixels per second.
            //Multiplying the number by delta time means that the rectangle will now move 30 pixels per second, regardless of the framerate.
            //Pretty much any time you do something that increments as time goes on, you'll want to multiply by u.dt to make sure you don't get timing issues.
            rectangle_x += u.dt * 30.0;

            //0, 0 is the top-left of the screen, so this will check to see if the rectangle went off-screen to the left.
            if rectangle_x > 200.0 {
                //If that happens, reset the rectangle_x to be to the left of the screen by 50 pixels.
                //Since the rectangle is 50 pixels wide, this will give it the illusion of smoothly wrapping around.
                rectangle_x = -50.0;
            }
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                rectangle([1.0, 1.0, 0.0, 1.0],
                    [rectangle_x, 75.0, 50.0, 50.0], //Replace the first 75.0 with the rectangle_x variable.
                    c.transform,
                    g);
            });
        }
    }
}