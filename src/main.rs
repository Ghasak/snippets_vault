use chrono::Local;
use clap::{Arg, Command};
use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;

/// Directory where snippets are stored. Customize this as needed.
const SNIPPET_DIR: &str = "Documents/myObsidianDoc/mysnippetsCollection";

/// Entry point of the application.
/// Defines available subcommands and routes the input to appropriate handlers.
fn main() {
    let matches = Command::new("SnippetsVault")
        .version("0.1.0")
        .author("Ghasak Ibrahim")
        .about("A secure and organized vault for managing your code snippets")
        .subcommand(
            Command::new("--create_snippet")
                .about("Create a new snippet")
                .arg(Arg::new("language").required(true))
                .arg(Arg::new("tags").num_args(1..)),
        )
        .subcommand(Command::new("--list_snippets").about("List all snippets"))
        .subcommand(Command::new("--edit_snippet").about("Edit a snippet using fuzzy finder"))
        .subcommand(Command::new("--version").about("Show version information"))
        .subcommand(Command::new("--languages").about("Show supported languages"))
        .after_help(
            r#"
NOTES:
------
	Old Usage: nvimTime [OPTION] [FILE] [ARGUMENT] ...
 ****************************
      Search syntax:
 ****************************
 Besides the already discussed fuzzy search, fzf supports special tokens that change the way search terms are processed:
 \wild    : Exact match, return items that include wild.
 ^music   : Prefix-exact-match, return items that start with music.
 .mp3\    : Suffix-exact-match, return items that end with .mp3.
 \!fire   : Inverse-exact-match, return items that do not include fire.
 \!^music : Inverse-prefix-exact-match, return items that do not start with music.
 \!.mp3\  : Inverse-suffix-exact-match, return items that do not end with .mp3.
 ****************************
            Note
 ****************************
 that SPACE acts as an AND operator and | as an OR. For example, a
 query that matches entries that start with music and end with either
 mp3, wav, or flac would look like this:
 ^music mp3 | wav | flac
 LINK: https://betterprogramming.pub/boost-your-command-line-productivity-with-fuzzy-finder-985aa162ba5d#c4fb
"#
        )
        .get_matches();

    // Global timestamp for snippet naming
    let timestamp = Local::now().format("%Y-%m-%d-%H%M%S").to_string();

    match matches.subcommand() {
        Some(("--create_snippet", sub_matches)) => {
            let language = sub_matches.get_one::<String>("language").unwrap();
            let tags: Vec<&str> = sub_matches
                .get_many::<String>("tags")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            create_snippet(language, &tags, &timestamp);
        }
        Some(("--list_snippets", _)) => {
            list_snippets();
        }
        Some(("--edit_snippet", _)) => {
            edit_snippet();
        }
        Some(("--version", _)) => {
            println!("{}", "SnippetVault Version: 0.1.0".green());
        }
        Some(("--languages", _)) => {
            list_languages();
        }
        _ => println!(
            "{}",
            "Unknown command. Use --help for usage information.".red()
        ),
    }
}

/// Creates a new snippet.
/// - `language`: The programming language of the snippet.
/// - `tags`: Tags associated with the snippet.
/// - `timestamp`: A timestamp for naming the snippet.
fn create_snippet(language: &str, tags: &[&str], timestamp: &str) {
    let home_dir = env::var("HOME").unwrap();
    let snippet_dir = format!("{}/{}", home_dir, SNIPPET_DIR);

    if !Path::new(&snippet_dir).exists() {
        fs::create_dir_all(&snippet_dir).unwrap();
        println!("{} Directory created: {}", "✔".green(), snippet_dir);
    }

    let tags_str = tags.join("::");
    let filename = format!("{}/snippet_{}_{}.md", snippet_dir, timestamp, language);

    let content = format!(
        "# Title: {} - Snippet\n# ---\n### Tags: {}\n\n### Content\n\n```{}\n\n```\n### Link:\n### Note:\n",
        language, tags_str, language
    );

    fs::write(&filename, content).unwrap();
    println!("{} Snippet created: {}", "✔".green(), filename);

    // Open the file in nvim
    let editor = get_default_editor();
    let _ = ProcessCommand::new(editor).arg(&filename).status();

    // Preview the file using glow
    let _ = ProcessCommand::new("glow").arg(&filename).status();
}

/// Lists all snippets using fuzzy search and preview tools.
fn list_snippets() {
    let home_dir = env::var("HOME").unwrap();
    let snippet_dir = format!("{}/{}", home_dir, SNIPPET_DIR);

    if Path::new(&snippet_dir).exists() {
        let editor = get_default_editor();

        // Delegate to bash commands for listing snippets
        let args = format!(
            r#"
            if [[ "$2" != "" ]]; then
                rga --files-with-matches $2 | fzf --sort --preview-window down:80%:wrap --preview 'glow --style=dark {{}}'
            else
                if [[ -d "{}" ]]; then
                    cd "{}" &&
                        selected_article=$(fzf --exact --info=inline --border --margin=1 --padding=1 --sort --preview-window down:80%:wrap --preview 'glow --style=dark {{}}')
                    {} $selected_article
                fi
            fi
            "#,
            snippet_dir, snippet_dir, editor
        );

        let status = ProcessCommand::new("bash")
            .arg("-c")
            .arg(args)
            .status()
            .expect("Failed to execute shell commands");

        if !status.success() {
            println!("{} Failed to list snippets.", "✘".red());
        }
    } else {
        println!("{} Snippet directory does not exist.", "✘".red());
    }
}

/// Opens a snippet for editing using fuzzy search to locate the file.
fn edit_snippet() {
    let home_dir = env::var("HOME").unwrap();
    let snippet_dir = format!("{}/{}", home_dir, SNIPPET_DIR);

    if Path::new(&snippet_dir).exists() {
        let editor = get_default_editor();

        // Delegate to bash commands for editing snippets
        let args = format!(
            r#"
            cd "{}"
            IFS=$'\n' files=($(fzf --exact --info=inline --border --margin=1 --padding=1 --sort --preview-window down:80%:wrap --preview 'glow --style=dark {{}}'))
            [[ -n "$files" ]] && {} "${{files[@]}}"
            "#,
            snippet_dir, editor
        );

        let status = ProcessCommand::new("bash")
            .arg("-c")
            .arg(args)
            .status()
            .expect("Failed to execute shell commands");

        if !status.success() {
            println!("{} Failed to edit snippets.", "✘".red());
        }
    } else {
        println!("{} Snippet directory does not exist.", "✘".red());
    }
}

/// Displays a list of supported programming languages.
fn list_languages() {
    let languages = vec![
        "python",
        "cpp",
        "bash",
        "terminal",
        "shell",
        "zsh",
        "php",
        "typescript",
        "scala",
        "nvim",
        "neovim",
        "pdf",
        "markdown",
        "org",
        "text",
        "shell",
        "powerShell",
        "perl",
        "haskell",
        "kotlin",
        "sql",
        "matlap",
        "groovy",
        "lua",
        "rust",
        "ruby",
        "html and css",
        "ruby",
        "java",
        "javascript",
        "swift",
        "c++",
        "c#",
        "docker",
        "kubernetes",
        "docker-compose",
        "rlang(R)",
        "golang(Go)",
        "vim",
        "apple",
        "mac",
        "macos",
        "applescript",
        "git",
        "gnuplot",
        "github",
        "linux",
        "gnu-linux",
        "ubuntu",
        "note",
        "memo",
        "awk",
        "sed",
        "tr",
        "cat",
        "jupyter",
        "jupyterlab",
        "lab",
        "bat",
        "latex",
        "emacs",
    ];

    for lang in languages {
        println!("{}", lang.cyan());
    }
}

/// Retrieves the default editor for editing snippets.
/// Tries a list of known paths for `nvim` or defaults to `nvim`.
fn get_default_editor() -> String {
    let editor_paths = vec![
        "$HOME/dev/nvim/bin/nvim",
        "$HOME/dev/neovim/build/bin/nvim",
        "$HOME/dev/neovim/bin/nvim",
        "/usr/local/bin/nvim",
    ];

    for path in editor_paths {
        let expanded_path = shellexpand::tilde(path).to_string();
        if Path::new(&expanded_path).exists() {
            return expanded_path;
        }
    }

    "nvim".to_string()
}
