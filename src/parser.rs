pub(crate) struct Redirect {
    pub path: String,
    pub append: bool,
}

pub(crate) struct ParsedCommand {
    pub command: String,
    pub args: Vec<String>,
    pub argument: String,
    pub stdout: Option<Redirect>,
    pub stderr: Option<Redirect>,
}

/// Parse the input string into a command and its arguments
/// # Arguments:
/// * `input` - A string slice containing the input command
/// # Returns:
/// * A tuple containing the command as a String and a vector of arguments as Vec<String>
pub(crate) fn parse_input(input: &str) -> ParsedCommand {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut backslash_happened = false;

    let input = input.replace("1>", ">");
    let mut chars = input.chars().peekable();

    let special_chars = ['\\', '$', '"', '"'];

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double_quote & !backslash_happened => {
                in_single_quote = !in_single_quote;
            }
            '"' if !backslash_happened & !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ' ' if !in_single_quote & !in_double_quote & !backslash_happened => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            _ if backslash_happened & !in_double_quote => {
                current.push(c);
                backslash_happened = false;
            }
            '\\' if !in_single_quote & !backslash_happened => {
                backslash_happened = true;
            }
            _ if backslash_happened & in_double_quote => {
                if special_chars.contains(&c) {
                    current.push(c);
                } else {
                    current.push('\\');
                    current.push(c);
                }
                backslash_happened = false;
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    // trim all arguments
    args.iter_mut()
        .for_each(|arg| *arg = arg.trim().to_string());

    let command = args.get(0).cloned().unwrap_or_default();
    let mut remaining_args = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        vec![]
    };
    let mut argument = remaining_args.join(" ").trim().to_string(); // used for echo/type/cd etc.
                                                                    // check if redirect is present
    let mut redirect_stdout: Option<String> = None;
    let mut redirect_stderr: Option<String> = None;
    let mut redirect_stdout_mode: bool = false;
    let mut redirect_stderr_mode: bool = false;

    if argument.contains(">") {
        let argument_clone = argument.clone();

        let split_pattern = if argument.contains(" > ") | argument.contains(" >> ") {
            if argument.contains(" >> ") {
                redirect_stdout_mode = true;
                " >> "
            } else {
                " > "
            }
        } else {
            if argument.contains(" 2>> ") {
                redirect_stderr_mode = true;
                " 2>> "
            } else {
                " 2> "
            }
        };

        let argument_split = argument_clone.split(split_pattern).collect::<Vec<&str>>();
        argument = argument_split
            .get(0)
            .cloned()
            .unwrap_or(&"")
            .trim()
            .to_string();
        // remove the redirect part from remaining_args
        if let Some(index) = remaining_args
            .iter()
            .position(|s| (s == ">") | (s == "2>") | (s == ">>") | (s == "2>>"))
        {
            // Truncate the vector, keeping elements *before* the index
            remaining_args.truncate(index);
            // println!("argument_split: {:?}", argument_split);
            if (split_pattern == " > ") | (split_pattern == " >> ") {
                redirect_stdout = Some(argument_split.get(1).unwrap_or(&"").to_string());
            } else {
                redirect_stderr = Some(argument_split.get(1).unwrap_or(&"").to_string());
            }
        }
    };

    ParsedCommand {
        command,
        args: remaining_args,
        argument,
        stdout: redirect_stdout.map(|path| Redirect {
            path,
            append: redirect_stdout_mode,
        }),
        stderr: redirect_stderr.map(|path| Redirect {
            path,
            append: redirect_stderr_mode,
        }),
    }
}
