# piston-tutorials
Series of tutorials on how to use the Piston game engine.

## Requirements
Must have a basic understanding of Rust, including concepts like functions, variables, references, and dependencies. I'll try to explain as much as I can, though.

Graphics card must support OpenGL 2.1 or higher to run Piston.

Two different sound libraries are included and used in the examples in the sound folder: Ears, and piston\_music. Ears requires OpenAL and libsnd, while piston\_music requires SDL2 and SDL2\_mixer. If you don't have these libraries installed and don't plan on using the sound examples, you can just remove them from Cargo.toml's dependencies sections.

## Intended order
1. hello_window (base window creation, basic explanation of event loop)
2. aabb (AABB-based collision detection)
3. pong (find_folder, more detailed event loop, keyboard input, sprite loading)
4. pong_ball (random number generation, text rendering)
5. sound-ears and sound-piston (sound loading and playing)
    * Order doesn't matter; I was initially unaware of piston\_music's existence, but since Ears is much simpler to add (as well as having different library requirements), I decided to just make tutorials for both.

## Credits
The Coder's Crux font used in the pong_ball example was created by Andrew McClusky. You can find its download page [here](https://www.dafont.com/coders-crux.font).

The song used in the sound examples is The Entertainer by Scott Joplin, and is licensed under Creative Commons 0. You can find its download page on the Free Music Archive [here](http://freemusicarchive.org/music/Scott_Joplin/Frog_Legs_Ragtime_Era_Favorites/04_-_scott_joplin_-_the_entertainer)

Speaker icons used in the sound examples are taken from Wikipedia, and are also licensed under Creative Commons 0.

The beeping sound used in the sound examples are from use code149 on Freesound, and can be downloaded from [here](https://freesound.org/people/code419/sounds/402853/). No edits or alterations were made to the original track.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.