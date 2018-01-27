//piston_music is more complicated than Ears, at least in my opinion, but it's still a pretty good crate.
//Note that at the time of writing, there isn't any documentation for piston_music on docs.piston.rs, and
//it is listed as a place the Piston team is researching on the main repository.
//This implies, at least to me, that it's still a work in progress, and it could change dramatically as time goes on.
//So, these examples could break at any time, and I have to take even more guesses than usual to find out what various things mean.

extern crate music;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;

//Possible states for the background music.
enum SoundState {
    Full,
    Half,
    Mute
}

//piston_music works by binding sound files to enum values.
//According to the piston_music source code, enums used like this need to impliment
//Hash and Eq, and have a static lifetime.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Piano,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Sound {
    Beep,
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

    //Current sound state. Can be changed in the event loop.
    let mut current_state = SoundState::Full;

    //Create music context and begin running code.

    //Music and Sound are the enums bound for music and sound respectively.
    //The underscore can optionally be replaced with an FnOnce instead, and the function inside will be called as soon as initialization is complete
    //For simplicity's take, I'll just use a closure in the function arguments instead, but later on I'll make another example showing how that works

    //The 16 is the number of sound channels to allocate - every sound being played takes up one channel,
    //so this setup means that 16 sounds can be playing at once.

    //Note that the examples in the piston_music repository just have all the window-updating code inside of the closure.
    //I have absolutely no idea if this is best practice, or if using the FnOnce would be better. For simplicity, I'll just use the closure instead.
    music::start::<Music, Sound, _>(16, || {
        //Bind music and sound files to the enums defined earlier.
        music::bind_music_file(Music::Piano, assets.join("sound/the_entertainer.wav"));
        music::bind_sound_file(Sound::Beep, assets.join("sound/beep.wav"));

        //Initialize the volume to maximum by default.
        //set_volume takes an f64 between 1.0 and 0.0. piston_music MAX_VOLUME as 1.0 and MIN_VOLUME as 0.0 for convinience
        music::set_volume(music::MAX_VOLUME);

        //Start playing the music bound to Music::Piano, and set it to repeat forever.

        music::play_music(&Music::Piano, music::Repeat::Forever);

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

                    //Play the beeping sound.
                    Key::Space => {
                        //Play the sound bound to Sound::Beep, it will repeat once like this.
                        //Times can be given any u16 to define how many times it will repeat before stopping.
                        //Music uses the global volume state, but sound doesn't; it takes the volume as a parameter.
                        music::play_sound(&Sound::Beep, music::Repeat::Times(0), music::MAX_VOLUME);
                    }

                    _ => {}
                }
            }

            //Use current_state to set music volume.
            if let Some(_u) = e.update_args() {
                //Set volume.
                match current_state {
                    SoundState::Full => { music::set_volume(1.0); },
                    SoundState::Half => { music::set_volume(0.5); },
                    SoundState::Mute => { music::set_volume(0.0); }
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
    });
}