///Feature name: sprite
/// An example showing how sprite loading/rendering works, as well as demonstrating find_folder.

//Allows you to search for folders instead of having to specify an exact path
//Very helpful while testing because searching for folders is relative to the terminal the program is being run from
//so this makes sure you don't have to copy any assets to the build folder while still testing
extern crate find_folder;
extern crate piston_window;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!",
            [400, 400]
        )
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut sprite_x = 0.0;
    let mut sprite_y = 0.0;

    let mut moving_up = false;
    let mut moving_down = false;
    let mut moving_left = false;
    let mut moving_right = false;

    let mut speed = 30.0;

    //Search for the assets folder and save its path
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    //Load the sprite.
    let sprite = Texture::from_path(
        &mut window.factory,
        assets.join("rust.png"), //assets.join() will combine the found folder with the argument, and return a combined PathBuf string.
        Flip::None, //Whether or not the texture needs to be flipped.
        &TextureSettings::new() //More advanced settings can be specified by creating a TextureSettings struct, and passing it in here. This one's empty because it's not needed.
    ).expect("Cannot find rust.png in the assets folder");

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
                sprite_y -= speed * u.dt;
            }

            if moving_down {
                sprite_y += speed * u.dt;
            }                       

            if moving_left {
                sprite_x -= speed * u.dt;
            }

            if moving_right {
                sprite_x += speed * u.dt;
            }
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);

                //Draws the image
                //c.transform is the default place to draw a texture. By default, it points to (0, 0)
                //Passing in c.transform by itself will put the texture's top-left corner at the top-left of the window
                //the trans function will translate it by the coordinates given
                image(&sprite, c.transform.trans(sprite_x, sprite_y), g);
            });
        }
    }
}