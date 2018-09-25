extern crate yaml_rust;

struct Context {
    before: Vec<String>,
    line: String,
    after: Vec<String>
}

struct Columns {
    start: usize,
    end: usize
}

impl Columns {
    fn new(&self, start: usize, end: usize) {
    }
}

struct Anchor {
    file_path: String, // TODO: Is there some Path type?
    line_number: usize,
    columns: Option<(usize, usize)>,
    context: Context,
    metadata: yaml_rust::Yaml,
}
