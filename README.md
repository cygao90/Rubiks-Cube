# Rubik's Cube

Use the left mouse button to rotate the cube.

Hold the right mouse button to adjust the view.

Support scramble and solve.

The solve functionality is provided by [kewb](https://github.com/luckasRanarison/kewb) with some modifications to support wasm.

Play online: [link](https://cygao90.github.io/games/rubiks-cube/)

## Build
- local
```
cargo build
```

- wasm
```
cargo build --profile wasm-release --target wasm32-unknown-unknown

wasm-bindgen --out-name rubiks-cube \
  --out-dir wasm \
  --target web target/wasm32-unknown-unknown/release/Rubiks-Cube.wasm
```
