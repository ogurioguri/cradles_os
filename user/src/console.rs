//标准输出
use super::write;
use core::fmt::{self, Write};
const STDOUT: usize = 1;
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}