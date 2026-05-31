# Architecture

<br>

Layered application architecture with local/remote client-server networking, built with Rust. 🧩 

<br>

A small Rust demo exploring two patterns I wanted to implement and showcase:

1. **Layered application engine** - Stack-based Layer system (menu / game / pause)
   driven by an Application loop, with input bindings, actions, and settings
   abstracted behind generic traits.
2. **Client/server game session** - The same client binary can run a fully
   integrated local server, or connect to a standalone dedicated server binary,
   selected via `settings.json`.
