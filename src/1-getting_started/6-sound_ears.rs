///Feature: sound_ears
/// Example demonstrating Ears, a simple Rust sound library

extern crate ears;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use ears::{Sound, Music, AudioController};

//Possible states for the background music.
enum SoundState {
    Full,
    Half,
    Mute
}

fn main() {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    
    let mut window: PistonWindow = WindowSettings::new(
            "Hello, window!",
            [200, 200]
        )
        .opengl(OpenGL::V3_2)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //Texture for when the sound is at maximum volume
    let player_full = Texture::from_path(
        &mut window.factory,
        assets.join("sound/player_full.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("Unable to find sound/player_full.png");

    //Texture for when the sound is at half volume
    let player_half = Texture::from_path(
        &mut window.factory,
        assets.join("sound/player_half.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("Unable to find sound/player_half.png");

    //Texture for when the sound is muted
    let player_mute = Texture::from_path(
        &mut window.factory,
        assets.join("sound/player_mute.png"),
        Flip::None,
        &TextureSettings::new()
    ).expect("Unable to find sound/player_mute.png");

    //Load music file. Paths have to be a &str, but find_folder uses PathBufs, so it has to be converted.
    //Music streams sound off the disk, which is slower than loading the file into memory, but less memory-intensive.
    let mut mus = Music::new( &assets.join("sound/the_entertainer.wav").to_str().unwrap() ).expect("Unable to find sound/the_entertainer.wav");

    //Load sound file
    //Sound loads the entire file into memory, which is faster than streaming it off disk, but more memory-intensive.
    let mut snd = Sound::new( &assets.join("sound/beep.wav").to_str().unwrap() ).expect("Unable to find sound/bleep.wav");

    //Current sound state. Can be changed in the event loop.
    let mut current_state = SoundState::Full;

    //Start playing the music.
    mus.play();

    while let Some(e) = window.next() {

        //Update current_state based on keyboard input.
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                //Make music volume louder.
                Key::W => {
                    match current_state {
                        SoundState::Full => {},
                        SoundState::Half => { current_state = SoundState::Full; },
                        SoundState::Mute => { current_state = SoundState::Half; }
                    }
                },

                //Make music volume quieter.
                Key::S => {
                    match current_state {
                        SoundState::Full => { current_state = SoundState::Half; },
                        SoundState::Half => { current_state = SoundState::Mute; },
                        SoundState::Mute => {}
                    }
                },

                //If the sound isn't already playing, play a beeping sound.
                Key::Space => {
                    snd.play();
                }

                _ => {}
            }
        }

        //Use current_state to set music volume.
        if let Some(_u) = e.update_args() {
            //Set volume.
            //1.0 is the default, and the normal volume of the sound.

            match current_state {
                SoundState::Full => { mus.set_volume(1.0); },
                SoundState::Half => { mus.set_volume(0.5); },
                SoundState::Mute => { mus.set_volume(0.0); }
            }

            //Loop the sound if it stops playing.
            if !mus.is_playing() {
                mus.play();
            }
        }

        if let Some(_r) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);

                match current_state {
                    SoundState::Full => { image(&player_full, c.transform, g); },
                    SoundState::Half => { image(&player_half, c.transform, g); },
                    SoundState::Mute => { image(&player_mute, c.transform, g); }
                }
            });
        }
    }
}