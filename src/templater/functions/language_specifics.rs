/*
 *   Copyright (c) 2024 Jack Bennett.
 *   All Rights Reserved.
 *
 *   See the LICENCE file for more information.
 */

use std::{ffi::OsStr, path::Path};

use serde::Serialize;

/// An array of language extension-id pairs.
const EXT_BY_LANG_ID: [(&str, &str); 68] = [
    ("bat", "batch"),
    ("c", "c"),
    ("h", "c"),
    ("clj", "clojure"),
    ("cmake", "cmake"),
    ("cl", "common-lisp"),
    ("cc", "cpp"),
    ("cpp", "cpp"),
    ("cxx", "cpp"),
    ("c++", "cpp"),
    ("hpp", "cpp"),
    ("hxx", "cpp"),
    ("h++", "cpp"),
    ("cs", "csharp"),
    ("css", "css"),
    ("dart", "dart"),
    ("comp", "glsl"),
    ("frag", "glsl"),
    ("geom", "glsl"),
    ("glsl", "glsl"),
    ("tesc", "glsl"),
    ("tese", "glsl"),
    ("vert", "glsl"),
    ("go", "go"),
    ("haml", "haml"),
    ("handlebars", "handlebars"),
    ("hbs", "handlebars"),
    ("hlsl", "hlsl"),
    ("html", "html"),
    ("ini", "ini"),
    ("java", "java"),
    ("js", "javascript"),
    ("cjs", "javascript"),
    ("mjs", "javascript"),
    ("jsx", "javascript"),
    ("jsx", "javascript"),
    ("jinja", "jinja"),
    ("jinja2", "jinja"),
    ("json", "json"),
    ("jsonc", "jsonc"),
    ("kt", "kotlin"),
    ("less", "less"),
    ("lua", "lua"),
    ("md", "markdown"),
    ("pl", "perl"),
    ("py", "python"),
    ("pyc", "python"),
    ("pyo", "python"),
    ("rkt", "racket"),
    ("rb", "ruby"),
    ("rs", "rust"),
    ("sass", "sass"),
    ("sc", "scala"),
    ("scala", "scala"),
    ("scss", "scss"),
    ("sh", "shell"),
    ("sql", "sql"),
    ("swift", "swift"),
    ("tex", "tex"),
    ("toml", "toml"),
    ("ts", "typescript"),
    ("cts", "typescript"),
    ("mts", "typescript"),
    ("tsx", "typescript"),
    ("xhtml", "xhtml"),
    ("xml", "xml"),
    ("yaml", "yaml"),
    ("yml", "yaml"),
];

/// An array of language filename-id pairs.
const FILENAME_BY_LANG_ID: [(&str, &str); 2] =
    [("Makefile", "makefile"), ("CMakeLists.txt", "cmake")];

/// Get the associated language ID from the specified filename.
pub fn lang_id_from_filename<S: AsRef<str>>(filename: S) -> Option<&'static str> {
    let filename = filename.as_ref();
    let ext = Path::new(&filename).extension().and_then(OsStr::to_str);

    if ext.is_none() {
        return from_standard_filename(&filename);
    } else {
        return from_ext(&ext.unwrap());
    }
}

/// Get the associated language ID from the specified file extension.
fn from_ext<S: AsRef<str>>(ext: S) -> Option<&'static str> {
    let ext = ext.as_ref().to_lowercase();
    EXT_BY_LANG_ID.iter().find(|x| x.0 == ext).map(|x| x.1)
}

/// Get the associated language ID from the specified filename, if it is a standard recognised filename.
fn from_standard_filename<S: AsRef<str>>(filename: S) -> Option<&'static str> {
    FILENAME_BY_LANG_ID
        .iter()
        .find(|x| x.0 == filename.as_ref())
        .map(|x| x.1)
}

/// A struct containing comment style information.
/// Items are in order `CommentStyle(block_start, block_prefix, block_end)`
/// E.g. `("/*", " *", "*/")` for C/C++,
/// and `("", "#", "")` for Python.
#[derive(Serialize)]
pub struct CommentStyle(&'static str, &'static str, &'static str);

macro_rules! style {
    ($id:literal, ($start:literal, $prefix:literal, $end:literal)) => {
        ($id, CommentStyle($start, $prefix, $end))
    };
}

/// An array of language id-comment style pairs.
const COMMENTS_BY_LANG_ID: [(&str, CommentStyle); 43] = [
    style!("batch", ("", "REM", "")),
    style!("c", ("/*", " *", " */")),
    style!("clojure", ("", ";;", "")),
    style!("cmake", ("", "#", "")),
    style!("common-lisp", ("", ";;", "")),
    style!("cpp", ("/*", " *", " */")),
    style!("csharp", ("/*", " *", " */")),
    style!("css", ("/*", " *", " */")),
    style!("dart", ("/*", " *", " */")),
    style!("glsl", ("/*", " *", " */")),
    style!("go", ("/*", " *", " */")),
    style!("haml", ("", "-#", "")),
    style!("handlebars", ("{{!", "   ", "}}")),
    style!("hlsl", ("/*", " *", " */")),
    style!("html", ("<!--", "   ", "-->")),
    style!("ini", ("", "#", "")),
    style!("java", ("/*", " *", " */")),
    style!("javascript", ("/*", " *", " */")),
    style!("jinja", ("{#", "   ", "#}")),
    style!("json", ("/*", " *", " */")),
    style!("jsonc", ("/*", " *", " */")),
    style!("kotlin", ("/*", " *", " */")),
    style!("less", ("/*", " *", " */")),
    style!("lua", ("", "--", "")),
    style!("makefile", ("", "#", "")),
    style!("markdown", ("", "", "")), // markdown doesn't really have comments
    style!("perl", ("", "#", "")),
    style!("python", ("", "#", "")),
    style!("racket", ("#|", "   ", "|#")),
    style!("ruby", ("", "#", "")),
    style!("rust", ("/*", " *", " */")),
    style!("sass", ("/*", " *", " */")),
    style!("scala", ("/*", " *", " */")),
    style!("scss", ("/*", " *", " */")),
    style!("shell", ("", "#", "")),
    style!("sql", ("", "--", "")),
    style!("swift", ("/*", " *", " */")),
    style!("tex", ("", "%", "")),
    style!("toml", ("", "#", "")),
    style!("typescript", ("/*", " *", " */")),
    style!("xhtml", ("<!--", "   ", "-->")),
    style!("xml", ("<!--", "   ", "-->")),
    style!("yaml", ("", "#", "")),
];

/// Get the comment style associated with the specified language id.
pub fn comment_style_from_lang<'a, S: AsRef<str>>(id: S) -> Option<&'a CommentStyle> {
    COMMENTS_BY_LANG_ID
        .binary_search_by_key(&id.as_ref(), |&(id, _)| id)
        .ok()
        .map(|i| &COMMENTS_BY_LANG_ID[i].1)
}
