# rtask — CLI/TUI Todo List Manager

> A simple and minimalistic task manager for your terminal. Built with love and Rust.

Current Version: `0.2.0`
License: *GPL (planned)*

---

## Features

* ✅ Add and remove tasks
* ✅ Mark tasks as done or not done
* 🔁 Auto-remove tasks after marking them as done (with `-r` flag)
* 📋 List all tasks
* 🖥️ Interactive TUI mode

---

## Installation

```sh
cargo install --path .
```

---

## CLI Usage

```sh
rtask [COMMAND]
```

### Commands

* `add [TITLE]...` — Add a task with the specified title
* `remove [TITLE]...` — Remove a task with the given title
* `mark [OPTIONS] [TITLE]...` — Mark a task as done/undone

  * `-r`, `--remove` — Automatically remove task after marking it as done
* `list` — List all tasks with their statuses
* `tui` — Launch interactive terminal user interface
* `help` — Show help message

### Global Options

* `-h`, `--help` — Show help message
* `-V`, `--version` — Show version info

---

## TUI Mode

Run:

```sh
rtask tui
```

In TUI mode you can do all stuff what you can in cli mode



## TODO

*

---

## Notes

I'm still learning Rust and growing as a developer — please excuse any bugs, weird behavior, or silly mistakes in the code. If you notice anything broken, feel free to open an issue or PR!

Also, forgive my poor English if something sounds odd :)

Thanks for trying `rtask`!

---

*"One task at a time ✨"*
