# Untitled-Engine

The goal of this project will be to provide a block-based frontend to one or more existing graphical frontends, to help new game devs learn, and to help seasoned game devs explore.
This will probably take a very long time to get started.

## Exporters
Exporters convert the block-based game you have designed in `untitled-engine` to code that can either be compiled, or to script that can be run, depending on the target. **Please note that there WILL be differences between the finished games, and that you should not expect crossplay/multiplayer-between-languages to work well or at all. It is *highly* recommended that you pick one exporter for your game and stick with it.**

### untitled-js
The `untitled-js` exporter will generate a playable *Phaser* game that can be run directly in web browsers.

### untitled-rs
The `untitled-rs` exporter will generate source code for a game that can be built in Rust. I will probably use `macroquad`. I have not yet decided whether I want to make my own library that the game will use to reduce repetition of code. Eventually, I'd like to be able to compile code from the `untitled-rs` exporter automatically.