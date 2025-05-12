use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct CircularBuffer<T> {
    data: Vec<T>,
    tail: usize,
    head: usize,
    len: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity);
        for _ in 0..capacity { buf.push(T::default()); }
        CircularBuffer {
            data: buf,
            tail: 0,
            head: 0,
            len: 0,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.len == self.data.len() {
            return Err(Error::FullBuffer);
        } else {
            self.data[self.tail] = _element;
            self.tail = (self.tail + 1) % self.data.len();
            self.len += 1;
            return Ok(());
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.len == 0 {
            return Err(Error::EmptyBuffer);
        } else {
            let element = mem::take(&mut self.data[self.head]);
            self.head = (self.head + 1) % self.data.len();
            self.len -= 1;
            return Ok(element);
        }
    }

    pub fn clear(&mut self) {
        while self.len > 0 {
            self.read().unwrap();
        }
    }

    pub fn overwrite(&mut self, _element: T) {
        // if it's full, we need to read one element and discard it
        if self.len == self.data.len() {
            self.read().unwrap();
        }
        self.write(_element).unwrap();
    }

    pub fn make_contiguous(&mut self) {

        // if it's empty, we can just reset the pointers
        if self.len == 0 {
            self.head = 0;
            self.tail = 0;
        } else {
            // otherwise we need to make it contiguos: just rotate it until head is zero
            while self.head != 0 {
                let element = self.read().unwrap();
                self.write(element).unwrap();
            }
        }
    }

    fn real_index(&self, index: usize) -> usize {
        if index >= self.len {
            panic!("out of bounds");
        }
        (self.head + index) % self.data.len()
    }
}


impl<T: Default> Index<usize> for CircularBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.real_index(index)]
    }
}

impl<T: Default> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let idx = self.real_index(index);
        &mut self.data[idx]
    }
}

impl<T> Deref for CircularBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        if self.head > self.tail {
            panic!("not contiguous!!!")
        }
        &self.data[self.head..self.head + self.len]
    }
}

pub trait TryDeref {
    type Target: ?Sized;

    fn try_deref(&self) -> Result<&Self::Target, String>;
}

impl<T: Default> TryDeref for CircularBuffer<T> {
    type Target = [T];

    fn try_deref(&self) -> Result<&Self::Target, String> {
        if self.head > self.tail {
            return Err("not contiguous".to_string());
        }
        Ok(&self.data[self.head..&self.head + self.len])
    }
}

impl<T: Default> DerefMut for CircularBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.make_contiguous();
        if self.head > self.tail {
            panic!("not contiguous!!!")
        }
        &mut self.data[self.head..self.head + self.len]
    }
}
