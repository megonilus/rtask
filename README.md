# rtask — CLI/TUI Todo List Manager

> is a project that has taught me a lot about Rust!

Current Version: `0.2.0`
License: *GPL (planned)*

---

## Features

* ✅ Add and remove tasks
* ✅ Mark tasks as done or not done
* 📋 List all tasks
* 🖥️ Interactive TUI mode

---

## Installation
Download the latest release or alternatively compile yourself:
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

* More features
* Safer code
* Better error handling
* Easier installation
* 
  

---

## Notes

I'm still learning Rust and programming itself — please excuse any bugs, weird behavior, or silly mistakes in the code.
I'm not very active on Github so I can go a long time without responding to PRs and Issues

Also, forgive my poor English if something sounds odd :)



Thanks for trying `rtask`!

---
