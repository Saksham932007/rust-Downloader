# Rust Downloader 🦀

A modern, fast, and elegant command-line file downloader built with Rust. This tool is a simple wget clone that showcases the power of asynchronous Rust for network I/O, featuring a rich progress bar and a clean user interface.

    This project was created by following a tutorial and updating a 2017 blog post to use modern Rust practices, including async/await, tokio, clap (derive), and indicatif.

🚀 Features

    Asynchronous Downloads: Built on tokio and reqwest for high-performance, non-blocking downloads.

    Beautiful Progress Bar: A rich, real-time progress bar powered by indicatif shows status, speed, and ETA.

    Simple & Ergonomic CLI: Uses clap for easy and intuitive command-line argument parsing.

    Streaming Downloads: Efficiently handles large files by streaming them directly to disk without loading the entire file into memory.

    Cross-Platform: Compiles and runs on Linux, macOS, and Windows.

    Custom Output Filename: Easily specify a different name for your downloaded file.

🛠️ Installation

You can install the tool directly from source using cargo.

First, ensure you have the Rust toolchain installed. If not, get it from rustup.rs.

Then, clone the repository and install the binary:

# Clone this repository
git clone 
cd 

# Install the binary
cargo install --path .

This will compile the project and place the rust-downloader executable in your Cargo binary path (~/.cargo/bin/), making it available from anywhere in your terminal.
💡 Usage

The usage is simple and straightforward.

1. Download a file

rust-downloader "[https://proof.ovh.net/files/100Mb.dat](https://proof.ovh.net/files/100Mb.dat)"

2. Download a file with a custom output name

Use the -o or --output flag to specify a name for the saved file.

rust-downloader "[https://proof.ovh.net/files/10Mb.dat](https://proof.ovh.net/files/10Mb.dat)" -o "my_test_file.dat"

3. Get help

Use the --help flag to see all available commands and options.

rust-downloader --help

🔧 Building from Source

If you just want to build the project locally without installing it, you can follow these steps.

# Clone this repository
git clone <your-github-repo-url>
cd rust-downloader

# Build the project
cargo build

# For an optimized release build
cargo build --release

You can then run the compiled executable from the target/ directory:

# Run the development build
./target/debug/rust-downloader "URL"

# Run the release build
./target/release/rust-downloader "URL"

📜 License

This project is licensed under the MIT License. See the LICENSE file for details.

<!-- Markdown link references -->

[]: #
[rust-link]: https://www.rust-lang.org
[]: #
[license-link]: https://opensource.org/licenses/MIT
