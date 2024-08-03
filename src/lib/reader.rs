use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        Read,
    }
};

use anyhow::bail;

use crate::lib::config::TextSource;

pub struct TextReader<'a>(Box<dyn BufRead + 'a>);
pub struct MultilineTextReader<'a>(Box<dyn BufRead + 'a>);

pub trait ReadString {
    fn read_string_to_buff(&mut self, buff: &mut String) -> std::io::Result<usize>;
}

impl<'a> ReadString for TextReader<'a> {
    fn read_string_to_buff(&mut self, buff: &mut String) -> std::io::Result<usize> {
        self.0.read_line(buff)
    }
}

impl<'a> ReadString for MultilineTextReader<'a> {
    fn read_string_to_buff(&mut self, buff: &mut String) -> std::io::Result<usize> {
        self.0.read_to_string(buff)
    }
}

pub fn text_reader_wrap<'a>(source: &'a TextSource, multiline: bool) -> anyhow::Result<Box<dyn ReadString + 'a>> {
    use TextSource::*;
    let buf_reader: Box<dyn BufRead> = match source {
        PlainText(str) => Box::new(BufReader::new(str.as_bytes())),
        FilePath(path) => Box::new(BufReader::new(File::open(path)?)),
        Stdin          => Box::new(BufReader::new(std::io::stdin())),
        _ => bail!("Text reader for source \"{source:?}\" is not implemented")
    };
    if multiline {
        Ok(Box::new(MultilineTextReader(buf_reader)))
    } else {
        Ok(Box::new(TextReader(buf_reader)))
    }
}