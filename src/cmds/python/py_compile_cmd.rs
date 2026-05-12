//! Runs Python py_compile with compact error output.

use crate::core::runner;
use crate::core::utils::{resolved_command, strip_ansi, tool_exists, truncate};
use anyhow::Result;

pub fn run(args: &[String], verbose: u8) -> Result<i32> {
    let mut cmd = if tool_exists("python") {
        resolved_command("python")
    } else {
        resolved_command("python3")
    };
    cmd.arg("-m").arg("py_compile");

    for arg in args {
        cmd.arg(arg);
    }

    if verbose > 0 {
        eprintln!("Running: python -m py_compile {}", args.join(" "));
    }

    runner::run_filtered(
        cmd,
        "py_compile",
        &args.join(" "),
        |raw| filter_py_compile_output(&strip_ansi(raw)),
        runner::RunOptions::default().no_trailing_newline(),
    )
}

pub fn filter_py_compile_output(output: &str) -> String {
    let trimmed = output.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    truncate(trimmed, 4_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_py_compile_empty_success() {
        assert_eq!(filter_py_compile_output(""), "");
    }

    #[test]
    fn test_filter_py_compile_trims_output() {
        assert_eq!(filter_py_compile_output("\nSyntaxError: invalid syntax\n"), "SyntaxError: invalid syntax");
    }
}
