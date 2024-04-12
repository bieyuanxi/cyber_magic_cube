# Cyber Magic Cube
A demo made by bevy 0.13.

# Development
## Installing Rust

## Installing OS Dependencies
https://bevyengine.org/learn/quick-start/getting-started/setup/

## Run the demo
```
cargo run
```

# Features
1. undo/redo cmd
2. string cmd based operations
3. cubes in N-dimensions(by changing const DIMENSION)
4. cube counts friendly by removing unnecessary cubes in space

# How to interact
Type one of the following cmds and press Enter.
```
1.[x|y|z][row][']
i.e
x1' == rotate column 1 around x axis in anti-clockwise direction
z5  == rotate column 5 around z axis in clockwise direction

2. undo

3. redo
```
