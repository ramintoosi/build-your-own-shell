[![progress-banner](https://backend.codecrafters.io/progress/shell/d4b673f4-c43e-4811-9e13-6b433aa0ea5b)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your `shell` implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git commit -am "pass 1st stage" # any msg
git push origin master
```

Time to move on to the next stage!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.87)` installed locally
1. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Running tests locally

Use `./run_tester.sh` to run the official
[shell-tester](https://github.com/codecrafters-io/shell-tester) against this
repo without submitting. On first run it downloads the tester binary to
`/tmp/shell-tester`.

```sh
./run_tester.sh                 # default: redirections
./run_tester.sh base            # early stages (init through run a program)
./run_tester.sh redirections    # stdout/stderr redirect + append
./run_tester.sh completions     # command completion (tab)
./run_tester.sh all             # base + navigation + quoting + redirections
./run_tester.sh el9             # a single stage by slug
```

Notes:

- Stage `ei0` (PWD) is skipped in `all` because it uses `sudo` to rename
  `/usr/bin/pwd`. Run that stage via `codecrafters submit` instead.
- A temporary `.tester-run/` copy of the repo is created during runs (gitignored).
