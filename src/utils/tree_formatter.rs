pub struct TreeFormatter<'out, W: std::io::Write> {
    output: &'out mut W,
    padding: String,
}

impl<'out, W: std::io::Write> TreeFormatter<'out, W> {
    pub fn new(output: &'out mut W, padding_capacity: usize) -> TreeFormatter<'out, W> {
        TreeFormatter {
            output,
            padding: String::with_capacity(padding_capacity),
        }
    }

    pub fn push_inter_branch(&mut self) -> std::io::Result<()> {
        writeln!(self.output, "")?;
        write!(self.output, "{}├─", self.padding)?;
        self.padding.push_str("│ ");
        Ok(())
    }

    pub fn next_inter_branch(&mut self) -> std::io::Result<()> {
        self.pop_branch();
        self.push_inter_branch()
    }

    pub fn push_final_branch(&mut self) -> std::io::Result<()> {
        writeln!(self.output, "")?;
        write!(self.output, "{}╰─", self.padding)?;
        self.padding.push_str("  ");
        Ok(())
    }

    pub fn next_final_branch(&mut self) -> std::io::Result<()> {
        self.pop_branch();
        self.push_final_branch()
    }

    pub fn new_line(&mut self) -> std::io::Result<()> {
        writeln!(self.output, "")?;
        write!(self.output, "{}", self.padding)?;
        Ok(())
    }

    pub fn pop_branch(&mut self) {
        self.padding.pop();
        self.padding.pop();
    }
}

impl<'out, W: std::io::Write> std::io::Write for TreeFormatter<'out, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.output.flush()
    }
}
