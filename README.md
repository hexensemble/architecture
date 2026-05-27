# Architecture

A small Rust demo exploring two patterns I wanted to showcase:

1. **A layered application engine** — a stack-based Layer system (menu / game / pause)
   driven by an Application loop, with input bindings, actions, and settings
   abstracted behind generic traits.
2. **A client/server game session** — the same client binary can run a fully
   integrated local server, or connect to a standalone dedicated server binary,
   selected via `settings.json`.

Built with:

[`raylib`](https://crates.io/crates/raylib) - windowing/rendering.

[`hecs`](https://crates.io/crates/hecs) - ECS.

[`renet`](https://crates.io/crates/renet) - net transport.
