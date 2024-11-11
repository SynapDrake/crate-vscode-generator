//! [<img alt="github" src="https://img.shields.io/badge/github-SynapDrake-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="30">](https://github.com/SynapDrake/crate-vscode-generator)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/vscode-generator.svg?style=for-the-badge&color=fc8d62&logo=rust" height="30">](https://crates.io/crates/vscode-generator)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-vscode--generator-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="30">](https://docs.rs/vscode-generator)
//! 
//! # VS Code Extensions Generator
//! 
//! 🛠️ A flexible toolkit for crafting VS Code extensions. This library streamlines the development process while maintaining high standards of 🚀 code quality and 🎯 user experience.
//! 
//! ## Overview
//! - ✨ Snippets generation with builder pattern
//! - 🎯 Future support for other VS Code extensions (planned)
//! - 🛠 Rich customization options
//! - ⚡ Efficient and type-safe implementation
//! 
//! ## Using
//! #### Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! vscode-generator = "<latest-version>"
//! ```
//! #### Generating Snippets
//! ```rust
//! use vscode_generator::{ Snippet, SnippetsFile };
//! 
//! // create snippets:
//! let snippet = Snippet::builder()
//!     .set_prefix("fn")
//!     .set_body(vec![
//!         "fn ${1:name}(${2:args}) ${3:-> ReturnType }{",
//!         "    ${0}",
//!         "}"
//!     ])
//!     .set_description("Create a new function")
//!     .set_scope("rust")
//!     .build()?;
//! 
//! // save snippets to file:
//! let snippets_file = SnippetsFile::new(vec![snippet]);
//! snippets_file.write_to("./snippets/rust.code-snippets")?;
//! ```
//! ## User Snippets
//! 📝 You can write this snippets to your VS Code custom user snippets folder
//! Locales:
//! * 🗂️ Windows: `%APPDATA%/Code/User/snippets`
//! * 🍎 MacOS: `~/Library/Application Support/Code/User/snippets`
//! * 🐧 Linux: `~/.config/Code/User/snippets`
//!
//! 💡 Or simply access it via VS Code:
//! 1. Press `Ctrl/Cmd + Shift + P`
//! 2. Type "Snippets: Configure User Snippets"
//! 3. Select the language or create a new snippet file 
//! 
//! For detailed snippets documentation and advanced features, see [`snippets`] module.
//! 
//! ## Future Extensions (Planned)
//! - 🎨 Color Themes
//! - 🔧 Language Support
//! - ⚙️ Custom Commands
//! - 🧩 Workspace Configuration

pub mod error;      pub use error::{ Result, Error };
pub mod prelude;

pub mod snippets;   pub use snippets::{ Snippet, SnippetBuilder, SnippetsFile };
