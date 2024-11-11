extern crate vscode_generator;
use vscode_generator::{ SnippetsFile, Snippet };

#[test]
#[cfg(feature = "rust")]
fn rust_snippets() {
    // 1. TODO: Создать шаблоны: let, let mut, static, const, use, mod {}, mod, mod use, mod tests 

    // generating snippets:
    let snippets = SnippetsFile::new(vec![
        // text:
        
        Snippet::rust_text("hello_world", r#"println!("Hello, world!")"#),

        // comments:

        Snippet::rust_todo_comment("/DEBUG", "DEBUG", None),
        Snippet::rust_todo_comment("/NOTE", "NOTE", None),
        Snippet::rust_todo_comment("/TODO", "TODO", None),
        Snippet::rust_todo_comment("/HACK", "HACK", None),
        Snippet::rust_todo_comment("/BUG", "BUG", None),
        Snippet::rust_todo_comment("/FIXME", "FIXME", None),
        Snippet::rust_todo_comment("/OPTIMIZE", "OPTIMIZE", None),
        Snippet::rust_todo_comment("/REVIEW", "REVIEW", None),
        Snippet::rust_todo_comment("/[i]", "[ ]", None),
        Snippet::rust_todo_comment("/[x]", "[x]", None),

        // function aliases:

        Snippet::rust_fn_alias(".to_string", ".to_string"),
        Snippet::rust_fn_alias(".to_owned", ".to_owned"),
        Snippet::rust_fn_alias(".unwrap", ".unwrap"),
        Snippet::rust_fn_alias(".expect", ".expect"),
        Snippet::rust_fn_alias(".clone", ".clone"),
        Snippet::rust_fn_alias(".collect", ".collect"),
        Snippet::rust_fn_alias(".iter", ".iter"),
        Snippet::rust_fn_alias(".into_iter", ".into_iter"),

        // macro aliases:

        Snippet::rust_macro_alias("println!", "println", None),
        Snippet::rust_macro_alias("format!", "format", None),
        Snippet::rust_macro_alias("dbg!", "dbg", None),
        Snippet::rust_macro_alias("assert!", "assert", None),
        Snippet::rust_macro_alias("assert_eq!", "assert_eq", None),
        Snippet::rust_macro_alias("panic!", "panic", None),
        Snippet::rust_macro_alias("vec!", "vec", None),

        Snippet::rust_macro_alias("str!", "str", None),
        Snippet::rust_macro_alias("re!", "re", None),
        Snippet::rust_macro_alias("deq!", "deq", None),
        Snippet::rust_macro_alias("map!", "map", Some((" {", "}"))),
        Snippet::rust_macro_alias("set!", "set", Some(("[", "]"))),
        Snippet::rust_macro_alias("list!", "list", Some(("[", "]"))),

        // macro attributes:

        Snippet::rust_attr("#derive", "derive", vec!["Debug", "Display", "Clone", "Copy", "Eq", "PartialEq", "Ord", "PartialOrd", "Hash", "Serialize", "Deserialize", "Default", "Send", "Sync"]),
        Snippet::rust_attr("#cfg", "cfg", vec!["test", "debug_assertions", "feature", "target_os"]),
        Snippet::rust_attr("#allow", "allow", vec!["dead_code", "unused_variables", "unused_imports"]),

        // operators:

        Snippet::builder()
            .set_prefix("pub ")
            .add_line("pub $1"),

        Snippet::builder()
            .set_prefix("pub(crate) ")
            .add_line("pub(crate) $1"),

        Snippet::builder()
            .set_prefix("pub(super) ")
            .add_line("pub(super) $1"),

        Snippet::builder()
            .set_prefix("pub(in crate::_) ")
            .add_line("pub(in crate::$1) $0"),

        Snippet::builder()
            .set_prefix("use _::_;")
            .add_line("use $1::$0;"),

        Snippet::builder()
            .set_prefix("use _::{ .. }")
            .add_line("use $1::{ $0 };"),

        Snippet::builder()
            .set_prefix("mod _;")
            .add_line("mod $1;"),

        Snippet::builder()
            .set_prefix("mod _;  use _::..;")
            .add_line("mod $1;  use $1::$0;"),

        Snippet::builder()
            .set_prefix("mod _;  use _::{ .. }")
            .add_line("mod $1;  use $1::{ $0 };"),

        Snippet::builder()
            .set_prefix("let _ = ..;")
            .add_line("let $1 = $0;"),

        Snippet::builder()
            .set_prefix("let mut _ = ..;")
            .add_line("let mut $1 = $0;"),

        Snippet::builder()
            .set_prefix("static _: _ = ..;")
            .add_line("static $1: $2 = $0;"),

        Snippet::builder()
            .set_prefix("const _: _ = ..;")
            .add_line("const $1: $2 = $0;"),

        Snippet::builder()
            .set_prefix("type _ = ..;")
            .add_line("type $1 = $0;"),

        // blocks:

        Snippet::builder()
            .set_prefix("mod _ { .. }")
            .set_body(vec![
                "mod $1 {",
                "    ${0:// TODO: ...}",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("mod tests { .. }")
            .set_body(vec![
                "#[cfg(test)]",
                "mod tests {",
                "    use super::*;",
                "",
                "    #[test]",
                "    fn $1() {",
                "        ${0:// TODO: ...}",
                "    }",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("if _ { .. }")
            .set_body(vec![
                "if $1 {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("if _ { .. } else { .. }")
            .set_body(vec![
                "if $1 {",
                "    ${2:// TODO: ...}",
                "} else {",
                "    ${3:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("if? _ { .. }else{ .. }")
            .set_body(vec![
                "if $1 { $2 }else{ $3 }",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("if let _ = _ { .. }")
            .set_body(vec![
                "if let ${1|Some(v),Ok(v),Err(e)|} = $2 {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("match _ { .. }")
            .set_body(vec![
                "match $1 {",
                "    $2 => $3,",
                "    _ => ${4:panic!()}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("match Result<_, _> { .. }")
            .set_body(vec![
                "match $1 {",
                "    Ok(v) => $2,",
                "    Err(e) => $3",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("match Option<_> { .. }")
            .set_body(vec![
                "match $1 {",
                "    Some(v) => $2,",
                "    None => $3",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("for _ in _ { .. }")
            .set_body(vec![
                "for $1 in $2 {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("while _ { .. }")
            .set_body(vec![
                "while $1 {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("while let _ = _ { .. }")
            .set_body(vec![
                "while let ${1|Some(v),Ok(v),Err(e)|} = $2 {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("loop { .. }")
            .set_body(vec![
                "loop {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("unsafe { .. }")
            .set_body(vec![
                "unsafe {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("async { .. }")
            .set_body(vec![
                "async {",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        // functions:

        Snippet::builder()
            .set_prefix("fn _() { .. }")
            .set_body(vec![
                "fn $1($2) ${3:-> }{",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("async fn _() { .. }")
            .set_body(vec![
                "async fn $1($2)  ${3:-> }{",
                "    ${0:// TODO: ...}",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("fn main() { .. }")
            .set_body(vec![
                "fn main() ${1:-> Result<()> }{",
                "    ${0:println!(\"Hello, world!\");}",
                "    ${2:\n    Ok(())}",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("async fn main() { .. }")
            .set_body(vec![
                "#[${1|tokio::main,async_std::main,actix_web::main,axum::main|}]",
                "async fn main() ${1:-> Result<()> }{",
                "    ${0:println!(\"Hello, world!\");}",
                "    ${2:\n    Ok(())}",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("pub fn new() -> Self { .. }")
            .set_body(vec![
                "/// Creates a new instance of $1",
                "pub fn new($2) -> Self {",
                "    Self {",
                "        ${0}",
                "    }",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("pub fn builder() -> _Builder { .. }")
            .set_body(vec![
                "/// Creates a new builder for $1",
                "pub fn builder() -> $1Builder {",
                "    $1Builder::default()",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("pub fn build() -> _ { .. }")
            .set_body(vec![
                "/// Builds the $1 instance",
                "pub fn build(self) -> Result<$1> {",
                "    Ok($1 {",
                "        ${0}",
                "    })",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("fn test_() { .. }")
            .set_body(vec![
                "#[test]",
                "fn test_$1() ${2:-> Result<()> }{",
                "    ${0:// TODO: ...}",
                "",
                "    ${3:Ok(())}",
                "}",
            ])
            .set_scope("rust"),

        // structures:
        
        Snippet::builder()
            .set_prefix("struct _ { .. }")
            .set_body(vec![
                "/// ...",
                "struct $1 {",
                "    $0",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("struct _ { .. } impl { .. }")
            .set_body(vec![
                "/// ...",
                "struct $1 {",
                "    $2",
                "}",
                "",
                "impl $1 {",
                "    $3",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("enum { .. }")
            .set_body(vec![
                "/// ...",
                "enum $1 {",
                "    $0",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("enum { .. } impl { .. }")
            .set_body(vec![
                "/// ...",
                "enum $1 {",
                "    $2",
                "}",
                "",
                "impl $1 {",
                "    $3",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("trait _ { .. }")
            .set_body(vec![
                "/// ...",
                "trait $1 {",
                "    $0",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl _ { .. }")
            .set_body(vec![
                "impl $1 {",
                "    $0",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl _ for _ { .. }")
            .set_body(vec![
                "impl $1 for $2 {",
                "    $0",
                "}",
            ])
            .set_scope("rust"),

        // implementations:
        
        Snippet::builder()
            .set_prefix("impl Debug for _ { .. }")
            .set_body(vec![
                "impl std::fmt::Debug for $1 {",
                "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {",
                "        f.debug_struct(\"$1\")",
                "            $0",
                "            .finish()",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Display for _ { .. }")
            .set_body(vec![
                "impl std::fmt::Display for $1 {",
                "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {",
                "        write!(f, \"$0\")",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Default for _ { .. }")
            .set_body(vec![
                "impl Default for $1 {",
                "    fn default() -> Self {",
                "        Self {",
                "            $0",
                "        }",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl From<_> for _ { .. }")
            .set_body(vec![
                "impl From<$1> for $2 {",
                "    fn from(v: $1) -> Self {",
                "        $0",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Into<_> for _ { .. }")
            .set_body(vec![
                "impl Into<$1> for $2 {",
                "    fn into(self) -> $1 {",
                "        $0",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Error for _ { .. }")
            .set_body(vec![
                "impl std::error::Error for $1 {}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Clone for _ { .. }")
            .set_body(vec![
                "impl Clone for $1 {",
                "    fn clone(&self) -> Self {",
                "        Self {",
                "            $0",
                "        }",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Deref for _ { .. }")
            .set_body(vec![
                "impl std::ops::Deref for $1 {",
                "    type Target = $2;",
                "",
                "    fn deref(&self) -> &Self::Target {",
                "        $0",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl AsRef<_> for _ { .. }")
            .set_body(vec![
                "impl AsRef<$1> for $2 {",
                "    fn as_ref(&self) -> &$1 {",
                "        $0",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Serialize for _ { .. }")
            .set_body(vec![
                "impl Serialize for $1 {",
                "    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>",
                "    where",
                "        S: serde::Serializer,",
                "    {",
                "        $0",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl<'de> Deserialize<'de> for _ { .. }")
            .set_body(vec![
                "impl<'de> Deserialize<'de> for $1 {",
                "    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>",
                "    where",
                "        D: serde::Deserializer<'de>,",
                "    {",
                "        use serde::de::Error;",
                "        $0",
                "    }",
                "}",
            ])
            .set_scope("rust"),

        Snippet::builder()
            .set_prefix("impl Future for _ { .. }")
            .set_body(vec![
                "impl Future for $1 {",
                "    type Output = $2;",
                "",
                "    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {",
                "        $0",
                "    }",
                "}",
            ]),

        Snippet::builder()
            .set_prefix("impl Stream for _ { .. }")
            .set_body(vec![
                "impl Stream for $1 {",
                "    type Item = $2;",
                "",
                "    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {",
                "        $0",
                "    }",
                "}",
            ])
    ]);

    // create a cnippets file:
    snippets.write_to("./snippets/rust.code-snippets").unwrap();
    snippets.write_to("C:/Users/Admin/AppData/Roaming/Code/User/snippets/rust.code-snippets").unwrap();  // NOTE: Path to the vscode custom user snippets folder!!
}


