# battle-city
- [x] Design levels (Ldtk software)
- [x] Load levels
- [x] Switch levels
- [x] Random resurrection positions
- [x] Collision detection using physics engine
- [x] Resurrection, shield, explosion and water etc sprite animations
- [x] game ui
- [x] game sounds
- [x] pause game
- [x] enemies ai
- [x] local multiplayer
- [x] WASM support

Play Online: [Click here]](https://nightswatchgames.github.io/games/battle-city/) (Open with PC Chrome/Firefox/Edge)

## Get started
1. Native
```
cargo run
```
2. WASM
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
```
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/battle-city.wasm
```

## Screenshots
![start_menu](screenshots/start_menu.png)
![game_playing](screenshots/game_playing.png)
![game_over](screenshots/game_over.png)

## References
- [bevy-cheatbook](https://github.com/bevy-cheatbook/bevy-cheatbook)（[中文翻译](https://yiviv.com/bevy-cheatbook/)）
- [Unity制作坦克大战](https://www.bilibili.com/video/BV1PW41197Su)
- [Battle City - Wikipedia](https://en.wikipedia.org/wiki/Battle_City)