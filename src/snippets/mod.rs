//! # The Snippets Module
//! 
//! 📦 This module provides a comprehensive toolkit for working with VS Code snippets.
//! It allows creating, managing, and serializing code snippets with rich customization options.
//! 
//! ## Components
//! 
//! - [`Snippet`] - Individual snippet representation
//! - [`SnippetBuilder`] - Fluent builder for snippets
//! - [`SnippetsFile`] - Collection of snippets for VS Code
//! 
//! ## Overview
//! 
//! - 📝 Create and edit snippets with rich formatting
//! - 🔧 Customize triggers, descriptions, and scopes
//! - 📦 Manage collections of snippets
//! - 💾 Serialize/deserialize to VS Code format
//! 
//! ## Examples
//! 
//! #### 🎨 Creating a Simple Snippet
//! ```rust
//! use snippets::SnippetBuilder;
//! 
//! let snippet = SnippetBuilder::new()
//!     .set_prefix("fn")
//!     .add_line("fn ${1:name}() {")
//!     .add_line("    ${0:// TODO: ${0}}")
//!     .add_line("}")
//!     .build()
//!     .unwrap();
//! ```
//! 
//! #### 📁 Managing Snippet Collections
//! ```rust
//! use snippets::{ SnippetsFile, SnippetBuilder };
//! 
//! let mut file = SnippetsFile::new(vec![
//!     SnippetBuilder::new()
//!         .set_prefix("fn")
//!         .set_body(vec![
//!             "fn main() {",
//!             "    $0",
//!             "}"
//!         ])
//!         .set_scope("rust")
//! ]);
//! 
//! // Add test snippet
//! file.add_snippet(
//!     SnippetBuilder::new()
//!         .set_prefix("test")
//!         .set_body(vec![
//!             "#[test]",
//!             "fn test_${1:name}() {",
//!             "    $0",
//!             "}"
//!         ])
//!         .set_scope("rust")
//!         .build()
//!         .unwrap()
//! );
//! ```
//! 
//! ## Panics
//! 
//! The builder's `build()` method will panic if
//! - `prefix` is empty
//! - `name` is empty
//! - `body` is empty
//! 
//! ## Notes
//! 
//! #### The syntax of VS Code snippets:
//! 
//! - 📌 Use `$0` to specify the final cursor position
//! - 📌 Use `$1`, `$2`, etc. for tabstops
//! - 📌 Use `${1:default text}` for placeholders with default values
//! - 📌 Use `${1|one,two,three|}` for dropdown choices
//! 
//! 
//! #### See Also
//! 
//! - 🔗 Structure [`SnippetFile`](super::SnippetFile) - For more flexible snippet construction
//! - 🔗 VS Code [Snippet Guide](https://code.visualstudio.com/docs/editor/userdefinedsnippets)

pub mod snippet;            pub use snippet::Snippet;
pub mod snippet_builder;    pub use snippet_builder::SnippetBuilder;
pub mod snippets_file;      pub use snippets_file::SnippetsFile;
