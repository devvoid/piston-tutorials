# piston-tutorials
Series of tutorials on how to use the Piston game engine.

## Requirements
Must have a basic understanding of Rust, including concepts like functions, variables, references, and dependencies. I'll try to explain as much as I can, though.

Graphics card must support OpenGL 2.1 or higher to run Piston.

Piston doesn't seem to have a native audio API from what I can tell, so the Ears library is being used instead. Ears requires OpenAL and libsnd be installed on the target system. Refer to the [Ears documentation](https://github.com/jhasse/ears) for information on installing them.

## Intended order
1. hello_window (base window creation, basic explanation of event loop)
2. pong (find_folder, more detailed event loop, keyboard input, sprite loading)
3. aabb (AABB-based collision detection)
4. pong_ball (random number generation, sound playing, text rendering)