///Feature name: base
/// Just hello_world, but with all comments stripped. Basically just here to serve as a base so I don't have to re-type the basics every time.

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
    
    while let Some(e) = window.next() {
        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.0; 4], g);

                rectangle([1.0, 1.0, 0.0, 1.0],
                    [75.0, 75.0, 50.0, 50.0],
                    c.transform,
                    g);
            });
        }
    }
}