![Sierpiński Triangle](/demo.gif)

# Rust chip8
This is my take on chip8 interpretator written in rust using low level create such as winit, cpal, pixels. This is for educational purpose only. There are many parts that could have been done better, so read/use this repo at your on risk 🙃.

## Usage
Run the following command:
```sh
cargo run /path/file.ch8
```
where `/path/file.ch8` is the path to chip8 rom. If no rom is specified, it will used the default rom that come with this repo (IBM Logo.ch8).

## Resources
* [High-level guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
* [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
* [/r/Emudev](https://www.reddit.com/r/EmuDev/) and Emudev discord server