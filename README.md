# piston-tutorials
Series of tutorials on how to use the Piston game engine.

Top line of every .rs file says the name of the feature. Compile examples using `cargo run --release --bin [FEATURE NAME]`.

## Requirements
Must have a basic understanding of Rust, including concepts like functions, variables, references, and dependencies. I'll try to explain as much as I can, though.

By default, examples need OpenGL 3.2 to run, but you can run them on 2.1 by changing one line of code in the window initialization.

Two different sound libraries are included and demonstrated in the getting_started folder: Ears, and piston\_music. Ears requires OpenAL and libsnd, while piston\_music requires SDL2 and SDL2\_mixer. If you don't have these libraries installed and don't plan on using the sound examples, you can just remove them from Cargo.toml's dependencies section.

## Notes
Two sound examples are included, both numbered in getting\_started as example 6. When the tutorials were first written, I had no idea that piston\_music even existed, and so I wrote an example based on the Ears library instead. When I found out about piston\_music, I wrote an example about it too, but I kept the Ears example because it's much simpler to add and setup than piston\_music, and it has different library requirements. Future examples involving sound will likely use piston\_music exclusively, as this is a tutorial series about Piston.

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