use crate::prelude::*;
use super::*;
use std::{ fs, path::Path };
use serde::Serialize;

/// # Snippets File Manager
/// 
/// 📁 A file controller for managing VS Code snippets collections and their file operations.
/// 
/// ## Overview
/// 
/// `SnippetsFile` provides a convenient way to:
/// - 📦 Create and manage collections of code snippets
/// - 🔄 Serialize snippets to VS Code compatible format
/// - 💾 Save snippets to the filesystem
/// 
/// ## Usage
/// 
/// #### ✨ Creating a new snippets file:
/// ```rust
/// let file = SnippetsFile::new(vec![
///     Snippet::new("print", vec!["println!(\"$0\");"]),
///     Snippet::new("debug", vec!["dbg!($0);"])
/// ]);
/// ```
/// 
/// #### ➕ Adding snippets dynamically:
/// ```rust
/// let mut file = SnippetsFile::new(vec![]);
/// 
/// // Add single snippet
/// file.add_snippet(Snippet::new("log", vec!["log::info!(\"$0\")"]));
/// 
/// // Add multiple snippets
/// file.add_snippets(vec![
///     Snippet::new("error", vec!["log::error!(\"$0\");"]),
///     Snippet::new("warn", vec!["log::warn!(\"$0\");"])
/// ]);
/// ```
/// 
/// #### 💾 Saving to file:
/// ```rust
/// let file = SnippetsFile::new(vec![
///     Snippet::new("test", vec![
///         "#[test]",
///         "fn test_name() {",
///         "    ",
///         "}"
///     ])
/// ]);
/// 
/// // Saves to VS Code snippets directory
/// file.write_to("./snippets/rust.code-snippets")?;
/// ```
/// 
/// ## 📋 File Format
/// 
/// The snippets are saved in VS Code compatible JSON format:
/// ```json
/// {
///     "print": {
///         "prefix": "print",
///         "body": [
///             "println!(\"Hello\");"
///         ],
///         "description": "Basic print statement"
///     }
/// }
/// ```
/// 
/// ## ⚠️ Error Handling
/// 
/// The structure uses custom `Result` type for error handling:
/// - 🔄 JSON serialization errors
/// - 📂 File system operation errors
/// - 📁 Directory creation errors
/// 
/// ## User Snippets
/// 📝 You can write this snippets to your VS Code custom user snippets folder
/// Locales:
/// * 🗂️ Windows: `%APPDATA%/Code/User/snippets`
/// * 🍎 MacOS: `~/Library/Application Support/Code/User/snippets`
/// * 🐧 Linux: `~/.config/Code/User/snippets`
///
/// 💡 Or simply access it via VS Code:
/// 1. Press `Ctrl/Cmd + Shift + P`
/// 2. Type "Snippets: Configure User Snippets"
/// 3. Select the language or create a new snippet file 
#[derive(Debug, Clone, Serialize)]
pub struct SnippetsFile {
    pub snippets: HashMap<String, Snippet>,
}

impl SnippetsFile {
    /// Creates a new snippets file controller
    pub fn new<Sn: Into<Snippet>>(snippets: impl IntoIterator<Item = Sn>) -> Self {
        Self {
            snippets: snippets
                .into_iter()
                .map(|snip| {
                    let snip = snip.into();
                    (snip.name.clone(), snip)
                })
                .collect()
        }
    }

    /// Adds a new snippet to the collection
    pub fn add_snippet<S: Into<Snippet>>(&mut self, snippet: S) {
        let snippet = snippet.into();
        self.snippets.insert(snippet.name.clone(), snippet);
    }

    /// Adds a new snippets to the collection
    pub fn add_snippets<S: Into<Snippet>>(&mut self, snippets: impl IntoIterator<Item = S>) {
        self.snippets.extend(
            snippets
                .into_iter()
                .map(|snip| {
                    let snip = snip.into();
                    (snip.name.clone(), snip)
                })
        );
    }

    /// Converts the snippets to json string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.snippets).map_err(Error::from)
    }
    
    /// Writes the snippets to file path
    pub fn write_to(&self, path: &str) -> Result<()> {
        let path = Path::new(path);

        // creating the file dir:
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).map_err(Error::from)?;
        }
        
        // convert snippets to json:
        let json = self.to_json()?;

        // create the file:
        fs::write(path, json).map_err(Error::from)?;

        Ok(())
    }
}
