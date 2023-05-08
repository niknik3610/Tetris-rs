# Tetris-rs
Tetris, written in Rust using OpenGL. \
(Note: This is very much work in progress. Nothing works at the moment)

## Building
Use Rust's cargo compiler to build the program. Further information can be found here: https://doc.rust-lang.org/cargo/getting-started/installation.html \
Please view the dependencies, as the program will not build without them!

Then run the command: 
```
cargo build --release
```

## Dependencies
You will need to have the SDL2 dev-library installed. For more information please view: \
https://wiki.libsdl.org/SDL2/Installation (Good luck if you are on windows). \
Additionally you might need some OpenGL dependencies, the Rust Compiler (Cargo) should guide you on what to install.

## References 
For more information on OpenGL and SDL2 please view these websites, as they are what much of my implementation are based on: \
https://learnopengl.com \
http://nercury.github.io/rust/opengl/tutorial/2018/02/11/opengl-in-rust-from-scratch-05-triangle-colors.html 
