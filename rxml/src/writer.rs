use heck::ToSnakeCase;
use std::fmt::Write;

pub struct Writer {
    name: Option<String>,
    inner: String,
    indentation: u8,
}

impl Writer {
    const INDENT: &'static str = "    ";
    const COLUMN_LENGTH: usize = 100;

    pub fn inner(&self) -> &str {
        &self.inner
    }

    pub fn file_name(&self) -> String {
        format!("{}.rs", self.module_name())
    }

    pub fn module_name(&self) -> String {
        self.name.as_ref().unwrap().to_snake_case()
    }

    pub fn new_no_name() -> Self {
        Self {
            name: None,
            inner: String::with_capacity(8000),
            indentation: 0,
        }
    }

    pub fn new(name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            inner: String::with_capacity(8000),
            indentation: 0,
        }
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        for _ in 0..self.indentation {
            self.inner.write_str(Self::INDENT).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn newline(&mut self) {
        self.inner.write_char('\n').unwrap();
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s.as_ref());
        self.newline();
    }

    fn get_column(&self) -> usize {
        self.inner.len() - (self.inner.rfind('\n').unwrap() + 1)
    }

    pub(crate) fn space(&mut self) {
        self.inner.write_str(" ").unwrap();
    }

    pub(crate) fn w_break_at(&mut self, s: impl AsRef<str>) {
        let column = self.get_column();
        if column >= Self::COLUMN_LENGTH {
            self.newline();
            self.w(s.as_ref());
        } else if column == 0 {
            self.w(s.as_ref());
        } else {
            self.w_no_indent(s.as_ref());
        }
    }

    pub fn w_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn wln_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }

    pub fn new_struct(&mut self, name: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(format!("pub struct {}", name.as_ref()));
        f(self);
        self.closing_curly_newline();
    }

    pub fn open_curly(&mut self, s: impl AsRef<str>) {
        self.wln(format!("{} {{", s.as_ref()));
        self.inc_indent();
    }

    pub fn body(&mut self, s: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(s);
        f(self);
        self.closing_curly();
    }

    pub fn bodyn(&mut self, s: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(s);
        f(self);
        self.closing_curly_newline();
    }

    pub fn closing_curly(&mut self) {
        self.dec_indent();
        self.wln("}");
    }

    pub fn closing_curly_newline(&mut self) {
        self.dec_indent();
        self.wln("}");
        self.newline();
    }

    pub fn closing_curly_with(&mut self, s: impl AsRef<str>) {
        self.dec_indent();
        self.wln(format!("}}{}", s.as_ref()));
    }

    pub fn inc_indent(&mut self) {
        if self.indentation == 0xff {
            panic!("attempting to overflow indentation")
        }

        self.indentation += 1;
    }

    pub fn dec_indent(&mut self) {
        if self.indentation == 0 {
            panic!("attempting to underflow indentation")
        }

        self.indentation -= 1;
    }
}
