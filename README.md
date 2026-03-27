# Expedition 33 ASR Autosplitter
Credit goes to Nikoheart for the original asl autosplitter, as well as ISO2768-mK & PlaccidPenguin for edits made to it. This is essentially a WASM port of it, along with some new split points added. This was made because running ASL on linux sucks. You can find the ASL [here](https://github.com/Nikoheartttv/Autosplitters/tree/main/Clair%20Obscur%20Expedition%2033).

An auto splitter for Expedition 33 using the ASR runtime used in Livesplit One, making it cross-platform compatible.

## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain stable
```

The auto splitter can now be compiled:
```sh
cargo b --release
```

The auto splitter is then available at:
```
target/wasm32-unknown-unknown/release/coe33_asr_chatgpt.wasm
```

Make sure to look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

## Development

You can use the [debugger](https://github.com/LiveSplit/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory, step through the code and more.

The repository comes with preconfigured Visual Studio Code tasks. During
development it is recommended to use the `Debug Auto Splitter` launch action to
run the `asr-debugger`. You need to install the `CodeLLDB` extension to run it.

You can then use the `Build Auto Splitter (Debug)` task to manually build the
auto splitter. This will automatically hot reload the auto splitter in the
`asr-debugger`.

Alternatively you can install the [`cargo
watch`](https://github.com/watchexec/cargo-watch?tab=readme-ov-file#install)
subcommand and run the `Watch Auto Splitter` task for it to automatically build
when you save your changes.

The debugger is able to step through the code. You can set breakpoints in VSCode
and it should stop there when the breakpoint is hit. Inspecting variables may
not work all the time.

## Feature Checklist
- [x] New Game+ Support
- [x] Reset support
- [x] Properly wait for Build Version, currently soft locks
- [x] Mods folder check
- [ ] Load remove long loads post cutscenes (broken in ASL)

## New Split Points
- [x] Bruleram Battle
- [x] Monolith Train
- [x] Entering Lumiere?
- [ ] Creation Flee (Would need to implement logic to check for flee)

## Game Info
- BattleFlowState
    * 0: Out of battle
    * 1: Battle loading
    * 2: In battle
- BattleEndState
    * 0: In Battle
    * 1: Battle Won
    * 2: Expedition Failed, switches back to 0 for 2nd party
    * 3: Battle Over (Fake Paintress)
- CSCinematicStatus
    * 0: Not in cinematic
    * 1: In cinematic
    * 5: Cinematic paused
## Time Played
- It slows down with slowdown effects, can't be used to calculate battle time accurately.
