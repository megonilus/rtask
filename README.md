# rtask — CLI/TUI Todo List Manager

> is a project that has taught me a lot about Rust!

Current Version: `0.3.0`
License: _GPL (planned)_

---

## Requirements

- Rust (latest stable) — [Install Rust via rustup](https://rustup.rs/)
- Cargo (comes bundled with Rust)

Make sure you have Rust and Cargo installed before proceeding.

## Features

- Add and remove tasks
- Mark tasks as done or not done
- Change priority of tasks
- List all tasks
- Interactive TUI mode

---

## Installation

```sh
cd <path_to_source> && make install
```

---

## TUI Mode

Run:

```sh
rtask tui
```

In TUI mode you can do all stuff what you can in cli mode

## TODO

- [ ] Add more features and bring deeper meaning to the project
- [ ] Use Nerd Font icons for better visuals
- [ ] Create `utils.rs` for helper functions
- [ ] Let user choose database file name (both in CLI and TUI)
- [ ] Move output logic to separate `handler.rs`
- [ ] Improve terminal UI: layout, colors, structure
- [ ] Allow custom colors via config
- [ ] Add config file for persistent preferences
- [ ] Support `list --done` to filter completed tasks
- [ ] Support `list --search <query>` to search tasks
- [ ] Allow optional deadlines on task creation
- [ ] Add sorting by priority

## Notes

I'm still learning Rust and programming itself — please excuse any bugs, weird behavior, or silly mistakes in the code.

Also, forgive my poor English if something sounds odd :)

Thanks for trying `rtask`!

---
