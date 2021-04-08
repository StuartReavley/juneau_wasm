

pub struct JasmRenderVisitor {
    indent:usize,
    pub value:String
}

impl JasmRenderVisitor {
    pub fn new() -> Self {
        let value = String::new();
        let indent = 0;
        Self {indent, value}
    }

    pub fn write(&mut self, value:&str) {
        self.value.push_str(value);
    }

    pub fn increase_indent(&mut self) {
        self.indent+= 1;
    }
    pub fn decrease_indent(&mut self) {
        self.indent-= 1;
    }
    pub fn get_indent(&self) -> String {
        std::iter::repeat(" ").take(self.indent * 4).collect::<String>()
    }

    pub fn join_new_line(&self, values:Vec<String>) -> String {
        values.into_iter().map(|s|format!("{}{}", self.get_indent(), s)).collect::<Vec<_>>().join("\n")
    }
}
