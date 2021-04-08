

use std::{io::Write, ops::Add, io::Read};
use std::ops::AddAssign;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::convert::From;
use serde::{Serialize, Deserialize};


#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BitBuffer {
    values: Vec<u8>,
    index:  u8
}


impl BitBuffer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.values.len() * 8 - usize::from((8 - self.index) % 8)
    }

    pub fn get(&self, index:usize) -> bool {
        (self.values[index / 8] >> (7 - (index % 8))) & 1 == 1
    }
}



impl From<&str> for BitBuffer {
    fn from(item: &str) -> Self {
        let mut buffer = BitBuffer::new();
        buffer += item;
        buffer
    }
}



pub struct BitBufferIterator<'a> {
    buffer: &'a BitBuffer,
    index: usize,
    len:   usize
}

impl<'a> Iterator for BitBufferIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.index >= self.len {
            None
        }
        else {
            let value = self.buffer.get(self.index);
            self.index += 1;
            Some(value)
        }
    }
}

impl<'a> IntoIterator for &'a BitBuffer {
    type Item = bool;
    type IntoIter = BitBufferIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitBufferIterator {
            buffer: self,
            index: 0,
            len: self.len()
        }
    }
}


impl Display for BitBuffer {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        for value in self {
            fmt.write_str(if value {"1"} else {"0"})?;
        }
        Ok(())
    }
}


impl Add<&BitBuffer> for &BitBuffer {
    type Output = BitBuffer;

    fn add(self, other: &BitBuffer) -> BitBuffer {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl AddAssign<&BitBuffer> for BitBuffer {
    fn add_assign(&mut self, buffer: &BitBuffer) {
        let self_len = self.len();
        let buffer_len = buffer.len();
        let BitBuffer {values, index} = buffer;

        if self.index == 0
            {
            self.values.reserve(self.values.len() + values.len());
            self.values.extend(values);
            self.index = *index;
            }
        else
            {
            for x in values
                {
                *self.values.last_mut().unwrap() |= x >> self.index;
                self.values.push(x << (8 - self.index));
                }

            if *index > 0 && self.index + index <= 8 {
                self.values.pop();
            }
        
        self.index = (self.index + index) % 8;

        let next_self_len = self.len();
        assert_eq!(self_len + buffer_len, next_self_len);
        }
    }
}

impl Add<bool> for &BitBuffer {
    type Output = BitBuffer;

    fn add(self, other: bool) -> BitBuffer {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl AddAssign<bool> for BitBuffer {
    fn add_assign(&mut self, value: bool) {

        let assert = self.to_string();
        let x:u8 = if value {1} else {0};
        if self.index == 0 {
            self.values.push(x << 7);
        }
        else {
            *self.values.last_mut().unwrap() |= x << (8 - 1 - self.index);
        }
        
        self.index = (self.index + 1) % 8;
        assert_eq!(assert + (if value {"1"} else {"0"}), self.to_string());
    }
}

impl Add<&str> for &BitBuffer {
    type Output = BitBuffer;

    fn add(self, other: &str) -> BitBuffer {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl AddAssign<&str> for BitBuffer {
    fn add_assign(&mut self, value: &str) {
        for ch in value.chars() {
            match ch {
                '0' => *self += false,
                '1' => *self += true,
                _   => panic!("ch")
            }
        }
    }
}



#[test]
fn empty() {
    let buffer = BitBuffer::new();
    assert_eq!(buffer.to_string(), "");
}

#[test]
fn from_string() {
    let buffer = BitBuffer::from("010");
    assert_eq!(buffer.to_string(), "010");
}

#[test]
fn byte_boundary_bool() {
    let mut buffer = BitBuffer::from("1111000");
    buffer += "1";
    assert_eq!(buffer.to_string(), "11110001");

    buffer += "0";
    assert_eq!(buffer.to_string(), "111100010");  
}

#[test]
fn add_byte() {
    let mut buffer = BitBuffer::from("00");
    buffer += &BitBuffer::from("11111111");
    assert_eq!(buffer.to_string(), "0011111111");
}

