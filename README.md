# Build Your Own Shell

[![progress-banner](https://backend.codecrafters.io/progress/shell/d4b673f4-c43e-4811-9e13-6b433aa0ea5b)](https://app.codecrafters.io/users/ramintoosi)

My solution to the [CodeCrafters "Build Your Own Shell"](https://app.codecrafters.io/courses/shell/overview) challenge, written in Rust for practice. 

Profile: [ramintoosi](https://app.codecrafters.io/users/ramintoosi).

Implements a basic POSIX-style shell covering the course stages:

## Features

- **REPL** with readline-style editing via [rustyline](https://github.com/kkawakami/rustyline)
- **Builtins:** `echo`, `cd`, `pwd`, `type`, `exit`, `history`, `jobs`, `complete`, `declare`
- **External programs** resolved from `PATH`
- **Quoting** (single and double quotes)
- **I/O redirection** (`>`, `>>`, `2>`, `2>>`)
- **Pipelines** (`|`)
- **Background jobs** (`&`) with `jobs` listing and completion notifications
- **Tab completion** for builtins, executables, files/directories, and programmable completers (`complete -C`)
- **History** with persistence (`history -r` / `-w` / `-a`, `HISTFILE` load on startup / save on exit)
- **Parameter expansion** (`$VAR`, `${VAR}`) and `declare`

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.80+
- Cargo

## Build & run

```sh
cargo run --release
```

Or use the helper script (builds if needed, then runs):

```sh
./your_program.sh
```

You'll get a `$ ` prompt. Examples:

```sh
$ echo hello
hello
$ pwd
/home/you
$ ls | wc -l
42
$ sleep 5 &
[1] 12345
$ jobs
[1]+  Running                 sleep 5 &
$ history
$ exit
```

## Project layout

```
src/
  main.rs               # entry point
  lib.rs                # REPL loop, pipelines, variable expansion
  builtins.rs           # shell builtins
  executor.rs           # command execution
  parser.rs             # argument / quote parsing
  redirect.rs           # stdout / stderr redirection
  path.rs               # PATH lookup
  history.rs            # in-memory + file history
  background.rs         # background job tracking
  completer.rs          # tab completion
  external_completers.rs
```

## Testing

Against the official [shell-tester](https://github.com/codecrafters-io/shell-tester):

```sh
./run_tester.sh                 # all sections
./run_tester.sh quoting         # one section
./run_tester.sh background-jobs
./run_tester.sh history-persistence
./run_tester.sh filename-completion
./run_tester.sh programmable-completion
./run_tester.sh parameter-expansion
```

Available sections: `basics`, `navigation`, `quoting`, `redirection`, `completion`, `filename-completion`, `programmable-completion`, `pipelines`, `background-jobs`, `history`, `history-persistence`, `parameter-expansion`.

