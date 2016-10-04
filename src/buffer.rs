use std::io;
use std::io::{Read,Write};
use std::fs::File;


#[derive(Debug,PartialEq)]
pub struct Buffer {
    pub filename: Option<String>,
    pub content: Vec<String>,
    pub top_visible: usize,
}


impl Buffer {
    pub fn new() -> Self {
        Buffer { filename: None, content: vec![], top_visible: 0 }
    }

    pub fn open(filename: &str) -> io::Result<Self> {
        // TODO:
        // * Handle opening a new file
        // * Handle file permissions
        let mut file = try!(File::open(filename));
        let mut buf = String::new();
        try!(file.read_to_string(&mut buf));
        let lines: Vec<String> = buf.lines().map(|x|{ x.to_string() }).collect();
        Ok(Buffer { filename: Some(filename.to_string()), content: lines, top_visible: 0 })
    }

    pub fn write(&self, filename: Option<&str>) -> io::Result<()> {
        let filename = match filename {
            Some(filename) => filename,
            None => match self.filename {
                Some(ref filename) => filename,
                None => return Err(io::Error::new(io::ErrorKind::Other, "No file specified")),
            }
        };
        let mut file = try!(File::create(filename));
        for line in self.content.iter() {
            try!(file.write(line.as_bytes()));
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer() {
        assert_eq!(
            Buffer::new(),
            Buffer { filename: None, content: vec![], top_visible: 0 }
        );
    }

    #[test]
    fn open_buffer() {
        let buf = match Buffer::open("/etc/hostname") {
            Ok(buf) => buf,
            Err(_) => panic!("couldn't open buffer"),
        };
        assert_eq!(buf.filename, Some("/etc/hostname".to_string()));
        assert_eq!(buf.content.len(), 1);  // Actual contents depend on your hostname
    }

    #[test]
    fn write_buffer() {
        let mut buf = Buffer::new();
        buf.content.push("Hello".to_string());
        assert_eq!(buf.write(Some("/tmp/testfile")).ok(), Some(()));
        let newbuf = match Buffer::open("/tmp/testfile") {
            Ok(buf) => buf,
            Err(_) => panic!("couldn't open buffer"),
        };
        assert_eq!(newbuf.filename, Some("/tmp/testfile".to_string()));
        assert_eq!(newbuf.content, vec!["Hello".to_string()]);
        assert_eq!(newbuf.write(None).ok(), Some(()));
    }
}
