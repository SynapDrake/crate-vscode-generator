extern crate vscode_generator;
use vscode_generator::{ prelude::*, Snippet, SnippetsFile };

#[test]
fn test_snippets() -> Result<()> {
    // create snippets:
    let snippet = Snippet::builder()
        .set_prefix("fn")
        .set_body(vec![
            "fn ${1:name}(${2:args}) ${3:-> ReturnType }{",
            "    ${0}",
            "}"
        ])
        .set_description("Create a new function")
        .set_scope("rust")
        .build()?;
    
    // save snippets to file:
    let snippets_file = SnippetsFile::new(vec![snippet]);
    // snippets_file.write_to("./snippets/rust.code-snippets")?;
    snippets_file.write_to("C:/Users/Admin/AppData/Roaming/Code/User/snippets/test.code-snippets")?;

    Ok(())
}
