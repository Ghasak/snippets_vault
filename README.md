# SnippetVault

**SnippetVault** is a command-line tool written in Rust for securely and
efficiently managing code snippets. It provides utilities to create, edit,
list, and organize code snippets with support for fuzzy searching and markdown
previewing.

## Features

- **Create Snippets**: Create a new snippet with language and optional tags.
- **List Snippets**: Fuzzy-search and preview existing snippets.
- **Edit Snippets**: Use a fuzzy finder to locate and edit snippets.
- **Supported Languages**: View all supported programming languages.
- **Customization**: Choose your preferred editor and directory for storing snippets.
- **Integrated Tools**: Utilizes tools like `fzf` and `glow` for an enhanced CLI experience.

## Prerequisites

- Rust (for building the binary)
  Install Rust via [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Dependencies**:
  - `fzf`: Fuzzy finder
    ```bash
    brew install fzf
    ```
  - `glow`: Markdown preview tool
    ```bash
    brew install glow
    ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/<your-username>/SnippetVault.git
   cd SnippetVault
   ```
2. Build the binary:
   ```bash
   cargo build --release
   ```
3. Add the binary to your PATH:
   ```bash
   echo 'export PATH="$PATH:$(pwd)/target/release"' >> ~/.zshrc
   source ~/.zshrc
   ```
4. Verify the installation:
   ```bash
   snippet_vault --help
   ```

## Usage

Below are the primary commands provided by SnippetVault:

### Create a Snippet

Create a new snippet with a specific programming language and optional tags:

```bash
snippet_vault --create_snippet <language> [tags]
```

**Example:**

```bash
snippet_vault --create_snippet rust utility cli-tool
```

### List Snippets

List and fuzzy-search through existing snippets:

```bash
snippet_vault --list_snippets
```

### Edit a Snippet

Locate a snippet using fuzzy search and open it for editing:

```bash
snippet_vault --edit_snippet
```

### Show Supported Languages

List all supported programming languages:

```bash
snippet_vault --languages
```

### Show Version

Display the current version of SnippetVault:

```bash
snippet_vault --version
```

## Configuration

### Default Snippet Directory

By default, snippets are stored in:

```
$HOME/Documents/myObsidianDoc/mysnippetsCollection
```

To customize the directory, modify the `SNIPPET_DIR` constant in the source code.

### Default Editor

SnippetVault attempts to use `nvim` as the default editor. You can configure
your preferred editor by modifying the `get_default_editor` function in the
source code.

## Example Workflow

1. **Create a Snippet:**

   ```bash
   snippet_vault --create_snippet python flask api
   ```

   Opens the snippet in `nvim` for editing.

2. **List Snippets:**

   ```bash
   snippet_vault --list_snippets
   ```

   Use `fzf` to locate and preview snippets.

3. **Edit Snippets:**

   ```bash
   snippet_vault --edit_snippet
   ```

   Locate and edit a snippet directly.

4. **Check Supported Languages:**
   ```bash
   snippet_vault --languages
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---
