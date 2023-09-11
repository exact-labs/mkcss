# mkcss

mkcss is a simple CSS creation utility. It locates all the classes from your HTML documents and generates a stylesheet automatically.

## Usage

```bash
# Use path from command line argument
~ $ mkcss /path/to/your/file.html
Stylesheet has been created successfully.

# Use path from console prompt
~ $ mkcss
Stylesheet has been created successfully.
```

To generate CSS with reset.css, use the `--reset` flag like so:

```bash
~ $ mkcss /path/to/your/file.html --reset
Stylesheet has been created successfully.
```

For more commands, check out `mkcss --help`

### Installation

Pre-built binaries for Linux, MacOS, and Windows can be found on the [releases](https://github.com/exact-labs/mkcss/releases) page.

#### Building

- Clone the project `git clone https://github.com/exact-labs/mkcss.git`
- Open a terminal in the project folder
- Check if you have cargo (Rust's package manager) installed, just type in `cargo`
- If cargo is installed, run `cargo build --release`
- Put the executable into one of your PATH entries
  - Linux: usually /bin/ or /usr/bin/
  - Windows: C:\Windows\System32 is good for it but don't use windows
