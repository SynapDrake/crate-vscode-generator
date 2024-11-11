use crate::prelude::*;
use super::SnippetBuilder;
use serde::Serialize;

/// # The Snippet
/// 
/// 🧩 Represents a VS Code snippet with all its properties and functionality.
/// This structure allows you to create and manage code snippets for VS Code
/// with rich customization options.
/// 
/// ## Overview
/// 
/// - 🎯 Create snippets with prefix triggers and body content
/// - 🔧 Optional description and scope settings
/// - ⚡ Priority management for suggestion list
/// - 📦 Special templates to simplify development
/// 
/// ## Usage
/// 
/// ```rust
/// // Simple snippet creation
/// let snippet = Snippet::new("fn main", vec![
///     "fn main() {",
///     "    println!(\"Hello, World!\");",
///     "}"
/// ]);
/// 
/// // Using builder pattern
/// let snippet = Snippet::builder()
///     .set_prefix("fn test")
///     .set_body(vec![
///         "#[test]",
///         "fn test_$1() {",
///         "    $0",
///         "}"
///     ])
///     .set_description("Test function template")
///     .set_scope("rust")
///     .build()
///     .unwrap();
/// ```
/// 
/// ## Using Templates
/// 
/// ```rust
/// // Text snippet
/// let text = Snippet::text("hello", "println!(\"Hello, world!\");")
///     .set_description("Prints 'Hello, world!'")
///     .build()
///     .unwrap();
/// 
/// // TODO comment
/// let todo = Snippet::todo_comment("todo", "TODO", Some("//"))
///     .build()
///     .unwrap();
/// 
/// // Function alias
/// let println = Snippet::fn_alias("pr", "println!")
///     .build()
///     .unwrap();
/// ```
/// 
/// ## Rust-specific Templates
/// 
/// ```rust
/// // Requires feature = ["rust"]
/// 
/// // Rust macro alias
/// let macro_snippet = Snippet::rust_macro_alias("fmt", "format", None)
///     .set_description("format! macro")
///     .build()
///     .unwrap();
/// 
/// // Rust attribute alias
/// let derive = Snippet::rust_attr("der", "derive", vec!["Debug", "Clone"])
///     .set_description("Common derive attributes")
///     .build()
///     .unwrap();
/// ```
/// 
/// ## JSON Conversion
/// 
/// ```rust
/// let snippet = Snippet::new("fn", vec!["function() {", "    $0", "}"]);
/// let json = snippet.to_json().unwrap();
/// println!("{}", json);
/// ```
/// 
/// ## Panics
/// 
/// The builder's `build()` method will panic if:
/// 
/// - `prefix` is empty
/// - `name` is empty
/// - `body` is empty
/// 
/// ## Notes
/// 
/// #### The syntax of VS Code snippets:
/// 
/// - 📌 Use `$0` to specify the final cursor position
/// - 📌 Use `$1`, `$2`, etc. for tabstops
/// - 📌 Use `${1:default text}` for placeholders with default values
/// - 📌 Use `${1|one,two,three|}` for dropdown choices
/// 
/// 
/// #### See Also
/// 
/// - 🔗 Structure [`SnippetFile`](super::SnippetFile) - For more flexible snippet construction
/// - 🔗 VS Code [Snippet Guide](https://code.visualstudio.com/docs/editor/userdefinedsnippets)
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Snippet {
    /// Unique identifier for the snippet (not serialized)
    #[serde(skip_serializing)]
    pub name: String,
    /// The trigger text for the snippet
    pub prefix: String,
    /// The actual content of the snippet
    pub body: Vec<String>,
    /// Optional description of what the snippet does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional language scope (e.g., "rust")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Optional flag for file templates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_file_template: Option<bool>,
    /// Optional priority in suggestion list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
}

impl Snippet {
    /// Creates a new snippet with required fields
    pub fn new<S: Into<String>>(prefix: S, body: impl IntoIterator<Item = S>) -> Self {
        SnippetBuilder::new()
            .set_prefix(prefix)
            .set_body(body.into_iter().map(Into::into).collect())
            .build()
            .unwrap()
    }

    /// Creates a new SnippetBuilder instance
    pub fn builder() -> SnippetBuilder {
        SnippetBuilder::new()
    }

    /// Converts the snippet to json string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).map_err(Error::from)
    }
}

impl From<SnippetBuilder> for Snippet {
    fn from(value: SnippetBuilder) -> Self {
        value.build().unwrap()
    }
}

/// The standart snippet templates
impl Snippet {
    /// Creates a simple text snippet
    pub fn text<S: Into<String>>(prefix: S, text: S) -> SnippetBuilder {
        Self::builder()
            .set_prefix(prefix)
            .set_body(vec![text.into()])
    }

    /// Creates various comment templates (TODO, NOTE, etc.)
    pub fn todo_comment<S: Into<String>>(prefix: S, comment_name: &str, comment_type: Option<&str>) -> SnippetBuilder {
        let comment_type = comment_type.unwrap_or("//");
        
        Self::builder()
            .set_prefix(prefix)
            .set_body(vec![format!("{comment_type} {comment_name}: ${{1:...}}")])
    }

    /// Creates a function alias template
    pub fn fn_alias<S: Into<String>>(prefix: S, fn_name: &str) -> SnippetBuilder {
        Self::builder()
            .set_prefix(prefix)
            .set_body(vec![format!("{fn_name}()")])
    }
}

/// __BONUS__: The snippet templates for Rust programming language (use crate option `features = ["rust"]`)
#[cfg(feature = "rust")]
impl Snippet {
    /// `[rust]`: Creates a simple text snippet
    pub fn rust_text<S: Into<String>>(prefix: S, text: S) -> SnippetBuilder {
        Self::text(prefix, text)
            .set_scope("rust")
    }

    /// `[rust]`: Creates various comment templates (TODO, NOTE, etc.)
    pub fn rust_todo_comment<S: Into<String>>(prefix: S, comment_name: &str, comment_type: Option<&str>) -> SnippetBuilder {
        Self::todo_comment(prefix, comment_name, comment_type)
            .set_scope("rust")
    }

    /// `[rust]`: Creates a function alias template
    pub fn rust_fn_alias<S: Into<String>>(prefix: S, fn_name: &str) -> SnippetBuilder {
        Self::fn_alias(prefix, fn_name)
            .set_scope("rust")
    }

    /// `[rust]`: Creates a macro alias template
    pub fn rust_macro_alias<S: Into<String>>(prefix: S, macro_name: &str, custom_braces: Option<(&str, &str)>) -> SnippetBuilder {
        let (lpar, rpar) = custom_braces.unwrap_or(("(", ")"));
        
        Self::builder()
            .set_prefix(prefix)
            .set_body(vec![format!("{}!{lpar}\"${{1:args}}\"{rpar}", macro_name)])
            .set_scope("rust")
    }

    /// `[rust]`: Creates a macro attribute template
    pub fn rust_attr<S: Into<String>>(prefix: S, attr_name: &str, attr_args: Vec<&str>) -> SnippetBuilder {
        Self::builder()
            .set_prefix(prefix)
            .set_body(vec![format!("#[{attr_name}(${{1:{}}})]", attr_args.join("|"))])
            .set_scope("rust")
    }
}
