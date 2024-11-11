use crate::prelude::*;
use super::Snippet;
use std::{ time::SystemTime, fmt::Write };

/// # The Snippet Builder
/// 
/// 🏗️ A builder pattern implementation for creating Snippet instances with elegant
/// fluent interface. Think of it as your personal snippet butler.
/// 
/// ## Features
/// 
/// - 🎯 Fluent interface for natural snippet construction
/// - 🔄 Automatic unique name generation
/// - 📝 Rich body manipulation methods
/// - ⚡ Validation before building
/// 
/// ## Usage
/// ```rust
/// let snippet = SnippetBuilder::new()
///     .set_prefix("fn main")
///     .add_line("fn main() {")
///     .add_line("    println!(\"Hello, World!\");")
///     .add_line("}")
///     .build()
///     .unwrap();
/// ```
/// 
/// ## 🛠️ Advanced Usage
/// ```rust
/// let snippet = SnippetBuilder::new()
///     .set_prefix("test")
///     .set_body(vec![
///         "#[test]",
///         "fn test_${1}() {",
///         "    ${0:// Test code}",
///         "}"
///     ])
///     .set_description("Test function template")
///     .set_scope("rust")
///     .set_priority(10)
///     .build()
///     .unwrap();
/// ```
/// 
/// ## 🔧 Body Manipulation
/// ```rust
/// let snippet = SnippetBuilder::new()
///     .set_prefix("hello")
///     .add_line("print!(\"Hello, world!\");")
///     .map_body(|lines| {
///         lines.insert(0, "// Hello world".to_owned());
///     })
///     .map_line(1, |line| {
///         *line = line.replace("print!", "println!");
///     })
///     .build()
///     .unwrap();
/// ```
/// 
/// ## Methods
/// 
/// #### 🏷️ Core Methods:
/// - `new()` - Creates new builder instance
/// - `build()` - Constructs final Snippet
/// - `validate()` - Checks builder state
/// 
/// #### 📝 Content Setting:
/// - `set_name(name)` - Sets snippet name
/// - `set_prefix(prefix)` - Sets trigger text
/// - `set_description(desc)` - Sets description
/// - `set_scope(scope)` - Sets language scope
/// - `set_priority(prio)` - Sets suggestion priority
/// 
/// #### 📄 Body Manipulation:
/// - `set_body(lines)` - Sets entire body content
/// - `add_line(line)` - Adds single line
/// - `add_lines(lines)` - Adds multiple lines
/// - `set_line(n, line)` - Changes specific line
/// - `map_body(fn)` - Transforms entire body
/// - `map_line(n, fn)` - Transforms specific line
/// 
/// ## ⚠️ Validation Rules
/// 
/// Builder will fail if:
/// - Name is empty
/// - Prefix is empty
/// - Body is empty
/// - Line index is out of bounds
/// 
/// ## 🎯 Best Practices
/// 
/// - Always set meaningful prefix
/// - Add description for clarity
/// - Use scope for language-specific snippets
/// - Consider priority for frequently used snippets
/// 
/// ## 🔄 Default Values
/// 
/// - `name`: Auto-generated unique ID
/// - `prefix`: Empty string
/// - `body`: Empty vector
/// - Other fields: None
#[derive(Debug, Clone)]
pub struct SnippetBuilder {
    name: String,
    prefix: String,
    body: Vec<String>,
    description: Option<String>,
    scope: Option<String>,
    is_file_template: Option<bool>,
    priority: Option<u32>,
}

impl SnippetBuilder {
    /// Creates a new SnippetBuilder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a an unique name for snippet
    pub fn gen_name() -> String {
        // get current timestamp in milliseconds:
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        // generating 6 random characters:
        let mut random_suffix = String::with_capacity(6);
        for _ in 0..6 {
            write!(&mut random_suffix, "{}", (b'a' + (fastrand::u8(0..26))) as char).unwrap();
        }

        format!("snippet_{}_{}", timestamp, random_suffix)
    }

    /// Validates the builder state
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::NameIsRequired);
        }
        if self.prefix.is_empty() {
            return Err(Error::PrefixIsRequired);
        }
        if self.body.is_empty() {
            return Err(Error::BodyIsEmpty);
        }

        Ok(())
    }

    /// Builds the Snippet instance
    pub fn build(self) -> Result<Snippet> {
        self.validate()?;

        Ok(Snippet {
            name: self.name,
            prefix: self.prefix,
            body: self.body,
            description: self.description,
            scope: self.scope,
            is_file_template: self.is_file_template,
            priority: self.priority,
        })
    }

    /// Sets the name of the snippet
    pub fn set_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the prefix (trigger) of the snippet
    pub fn set_prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Sets the entire body of the snippet
    pub fn set_body<S: Into<String>>(mut self, body: Vec<S>) -> Self {
        self.body = body.into_iter().map(Into::into).collect();
        self
    }

    /// Map snippet body using a transformation function that gets mutable reference
    pub fn map_body<F>(mut self, mut f: F) -> Self 
    where 
        F: FnMut(&mut Vec<String>)
    {
        f(&mut self.body);
        self
    }

    /// Adds a single line to the snippet body
    pub fn add_line<S: Into<String>>(mut self, line: S) -> Self {
        self.body.push(line.into());
        self
    }

    /// Adds multiple lines to the snippet body
    pub fn add_lines<S: Into<String>>(mut self, lines: impl IntoIterator<Item = S>) -> Self {
        self.body.extend(lines.into_iter().map(Into::into));
        self
    }

    /// Edits a specific line in the snippet body
    pub fn set_line<S: Into<String>>(mut self, n: usize, line: S) -> Result<Self> {
        if n >= self.body.len() {
            return Err(Error::IndexOutOfBounds(n));
        }
        
        self.body[n] = line.into();
        Ok(self)
    }

    /// Map specific line using a transformation function
    pub fn map_line<F>(mut self, n: usize, mut f: F) -> Result<Self>
    where 
        F: FnMut(&mut String)
    {
        if n >= self.body.len() {
            return Err(Error::IndexOutOfBounds(n));
        }

        f(&mut self.body[n]);
        Ok(self)
    }

    /// Sets the description of the snippet
    pub fn set_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the scope of the snippet
    pub fn set_scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.scope = Some(scope.into());
        self
    }

    /// Sets whether this snippet is a file template
    pub fn set_is_file_template(mut self, is_template: bool) -> Self {
        self.is_file_template = Some(is_template);
        self
    }

    /// Sets the priority of the snippet
    pub fn set_priority(mut self, priority: u32) -> Self {
        self.priority = Some(priority);
        self
    }
}

impl Default for SnippetBuilder {
    fn default() -> Self {
        Self {
            name: Self::gen_name(),
            prefix: String::new(),
            body: vec![],
            description: None,
            scope: None,
            is_file_template: None,
            priority: None,
        }
    }
}
