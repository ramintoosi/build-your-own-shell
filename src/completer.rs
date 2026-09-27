use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Helper;
use rustyline::Result;

use crate::builtins::BUILTINS;
use crate::path::get_path_executables;

/// autocomplete the command
pub(crate) struct ShellCompleter;

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let line_to_cursor = &line[..pos];

        let start = line_to_cursor.rfind(' ').map_or(0, |i| i + 1);
        let word = &line_to_cursor[start..];

        let executables = get_path_executables();
        let all_commands = [
            BUILTINS.to_vec(),
            executables
                .keys()
                .map(|k| k.as_str())
                .collect::<Vec<&str>>(),
        ]
        .concat();
        let mut matches = Vec::new();

        for &cmd in &all_commands {
            if cmd.starts_with(word) {
                matches.push(Pair {
                    display: cmd.to_string(),
                    replacement: format!("{} ", cmd),
                })
            }
        }
        // sort matches by display
        matches.sort_by(|a, b| a.display.cmp(&b.display));
        Ok((start, matches))
    }
}

// Empty trait implementations to satisfy the Helper requirements
impl Highlighter for ShellCompleter {}
impl Hinter for ShellCompleter {
    type Hint = String;

    fn hint(&self, _: &str, _: usize, _: &Context<'_>) -> Option<String> {
        None
    }
}
impl Validator for ShellCompleter {}
impl Helper for ShellCompleter {}
