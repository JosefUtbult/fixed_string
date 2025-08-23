#![no_std]

#[cfg(test)]
mod tests;

use core::{
    clone::Clone,
    cmp::{Ord, PartialEq, min},
    default::Default,
    fmt,
    iter::Iterator,
    ops::{Index, IndexMut},
    option::Option::{self, None, Some},
    panic,
    result::Result::{self, Err, Ok},
    str, write,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixedStringError {
    AlreadyAssigned,
    Overflow,
    InvalidIndex,
    FormatError,
}

pub trait FixedStringRef: fmt::Debug + fmt::Display + fmt::Write {
    /// Returns a `&str` representation of the `String`.
    fn as_str<'a>(&'a self) -> &'a str;
    /// Returns `true` if the `FixedString` is full.
    fn is_full(&self) -> bool;
    /// Returns the current length of the `FixedString`
    fn length(&self) -> usize;
    /// Returns the total capacity of the `FixedString`
    fn capacity(&self) -> usize;
    /// Clear the content of a `FixedString`
    fn clear(&mut self);
    /// Assigns a value to the `FixedString`, truncating if necessary.
    fn assign(&mut self, string: &str) -> Result<(), FixedStringError>;
    /// Appends a string slice to the `FixedString`, truncating if necessary.
    fn push(&mut self, s: &str) -> Result<(), FixedStringError>;
    /// Appends a character to the `FixedString`, if it's not full
    fn push_char(&mut self, character: char) -> Result<(), FixedStringError>;
    /// Concatinates another fixed string with self
    fn concatinate(&mut self, other: &dyn FixedStringRef) -> Result<(), FixedStringError>;
    /// Index a character
    fn get<'a>(&'a self, index: usize) -> Result<&'a CHARACTER, FixedStringError>;
    /// Index a character
    fn get_mut<'a>(&'a mut self, index: usize) -> Result<&'a mut CHARACTER, FixedStringError>;
}

type CHARACTER = u8;
const CHARACTER_NONE: CHARACTER = 0;

#[derive(Copy, Eq)]
pub struct FixedString<const N: usize> {
    buffer: [CHARACTER; N],
    length: usize,
}

impl<const N: usize> FixedString<N> {
    /// Creates a new empty `FixedString`.
    pub const fn new() -> Self {
        Self {
            buffer: [CHARACTER_NONE; N],
            length: 0,
        }
    }

    /// Creates a new empty `FixedString` with an assigned value
    pub fn new_with(string: &str) -> Result<Self, FixedStringError> {
        let mut res = Self::new();
        match res.assign(string) {
            Ok(()) => Ok(res),
            Err(err) => Err(err),
        }
    }

    /// Clear a `FixedString`
    pub fn clear(&mut self) {
        self.length = 0;
    }

    /// Retrieve the raw data in the buffer
    pub fn raw(&self) -> &[CHARACTER; N] {
        &self.buffer
    }

    /// Format a `FixedString` with provided arguments
    pub fn format(args: fmt::Arguments) -> Result<FixedString<N>, FixedStringError> {
        let mut fixed_string: FixedString<N> = FixedString::new();
        match fmt::write(&mut fixed_string, args) {
            Ok(()) => Ok(fixed_string),
            Err(fmt::Error) => Err(FixedStringError::FormatError),
        }
    }

    /// Create a `FixedString` from raw data
    pub fn from_raw(raw: &[CHARACTER; N]) -> Result<FixedString<N>, FixedStringError> {
        let mut fixed_string: FixedString<N> = FixedString::new();

        let mut length = N;
        for i in 0..N {
            if raw[i] == CHARACTER_NONE {
                length = i;
                break;
            }
            fixed_string.buffer[i] = raw[i];
        }
        fixed_string.length = length;

        Ok(fixed_string)
    }

    /// Standard iterator
    pub fn iter(&self) -> FixedStringIterator<'_, N> {
        FixedStringIterator {
            content: self,
            position: 0,
        }
    }

    /// Take the content from a `FixedString`, leaving an empty `FixedString`
    pub fn take(&mut self) -> Self {
        let mut res = Self::new();
        for i in 0..self.length {
            res.buffer[i] = self.buffer[i];
            self.buffer[i] = CHARACTER_NONE;
        }
        res.length = self.length;
        self.length = 0;
        res
    }

    pub fn get_ref<'a>(&'a self) -> &'a dyn FixedStringRef {
        self as &dyn FixedStringRef
    }

    pub fn get_ref_mut<'a>(&'a mut self) -> &'a mut dyn FixedStringRef {
        self as &mut dyn FixedStringRef
    }
}

impl<const CAPACITY: usize> FixedStringRef for FixedString<CAPACITY> {
    fn as_str<'a>(&'a self) -> &'a str
    where
        [(); CAPACITY]:,
    {
        unsafe { str::from_utf8_unchecked(&self.buffer[..self.length]) }
    }

    /// Returns `true` if the `FixedString` is full.
    fn is_full(&self) -> bool {
        self.length == CAPACITY
    }

    /// Returns the current length of the `FixedString`
    fn length(&self) -> usize {
        self.length
    }

    /// Returns the total capacity of the `FixedString`
    fn capacity(&self) -> usize {
        CAPACITY
    }

    /// Clear the content of a `FixedString`
    fn clear(&mut self) {
        for i in 0..CAPACITY {
            self.buffer[i] = CHARACTER_NONE;
        }
        self.length = 0;
    }

    /// Assigns a value to the `FixedString`, truncating if necessary.
    fn assign(&mut self, string: &str) -> Result<(), FixedStringError> {
        if self.length != 0 {
            return Err(FixedStringError::AlreadyAssigned);
        }

        self.push(string)
    }

    /// Appends a string slice to the `FixedString`, truncating if necessary.
    fn push(&mut self, string: &str) -> Result<(), FixedStringError> {
        if self.length + string.len() > CAPACITY {
            return Err(FixedStringError::Overflow);
        }

        let bytes_to_copy = string.as_bytes();
        let copy_len = string.len().min(CAPACITY - self.length);
        for i in 0..min(CAPACITY - self.length, copy_len) {
            self.buffer[self.length + i] = bytes_to_copy[i] as CHARACTER;
        }
        self.length += copy_len;

        return Ok(());
    }

    /// Appends a character to the `FixedString`, if it's not full
    fn push_char(&mut self, character: char) -> Result<(), FixedStringError> {
        if self.length + 1 > CAPACITY {
            return Err(FixedStringError::Overflow);
        }
        self.buffer[self.length] = character as CHARACTER;
        self.length += 1;

        Ok(())
    }

    fn concatinate(&mut self, other: &dyn FixedStringRef) -> Result<(), FixedStringError> {
        if self.length + other.length() > CAPACITY {
            return Err(FixedStringError::Overflow);
        }

        for index in 0..other.length() {
            let offset = self.length + index;
            self.buffer[offset] = *other.get(index)?;
        }
        self.length += other.length();

        Ok(())
    }

    fn get<'a>(&'a self, index: usize) -> Result<&'a CHARACTER, FixedStringError> {
        if index >= CAPACITY {
            return Err(FixedStringError::InvalidIndex);
        }

        Ok(&self.buffer[index])
    }

    fn get_mut<'a>(&'a mut self, index: usize) -> Result<&'a mut CHARACTER, FixedStringError> {
        if index >= CAPACITY {
            return Err(FixedStringError::InvalidIndex);
        }

        Ok(&mut self.buffer[index])
    }
}

impl<const N: usize> fmt::Debug for FixedString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> fmt::Display for FixedString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Implementing `fmt::Write` for `FixedString` to make it a custom writer.
impl<const N: usize> fmt::Write for FixedString<N> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        // Check if there's enough space in the buffer
        if self.length + string.len() > N {
            return Err(fmt::Error); // Buffer overflow
        }

        // Copy the string into the buffer
        let bytes_to_copy = string.as_bytes();
        let copy_len = string.len().min(N - self.length);
        for i in 0..min(N - self.length, copy_len) {
            self.buffer[self.length + i] = bytes_to_copy[i] as CHARACTER;
        }
        self.length += copy_len;

        Ok(())
    }

    fn write_char(&mut self, character: char) -> fmt::Result {
        // Check if there's enough space in the buffer
        if self.length + 1 > N {
            return Err(fmt::Error); // Buffer overflow
        }

        // Copy the character into the buffer
        let bytes_to_copy = [character as u8];
        self.buffer[self.length..self.length + 1].copy_from_slice(&bytes_to_copy);
        self.length += 1;

        Ok(())
    }
}

/// Iterator implementation for `FixedString`
pub struct FixedStringIterator<'a, const N: usize> {
    content: &'a FixedString<N>,
    position: usize,
}

/// Iterator implementation for `FixedString`
impl<'a, const N: usize> Iterator for FixedStringIterator<'a, N> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.content.length {
            return None;
        }

        let raw = self.content.buffer[self.position] as char;
        self.position += 1;

        Some(raw)
    }
}

/// Clone a fixed string
impl<const N: usize> Clone for FixedString<N> {
    fn clone_from(&mut self, source: &Self) {
        self.clear();
        for i in 0..source.length {
            self.buffer[i] = source.buffer[i];
        }
        self.length = source.length;
    }

    fn clone(&self) -> Self {
        let mut dest = Self::new();
        dest.clone_from(self);
        dest
    }
}

/// Indexing into a Fixed String
impl<const CAPACITY: usize> Index<usize> for FixedString<CAPACITY> {
    type Output = CHARACTER;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!("Tried to access none-existing index {}", index);
        }

        &self.buffer[index]
    }
}

impl<const CAPACITY: usize> IndexMut<usize> for FixedString<CAPACITY> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.length {
            panic!("Tried to access none-existing index {}", index);
        }

        &mut self.buffer[index]
    }
}

impl<const CAPACITY: usize> PartialEq for FixedString<CAPACITY> {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            return false;
        }

        for i in 0..self.length {
            if self.buffer[i] != other.buffer[i] {
                return false;
            }
        }

        return true;
    }
}

impl<const CAPACITY: usize> Default for FixedString<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}
