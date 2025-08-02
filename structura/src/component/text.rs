//!
//!
//!

pub struct Text {
    pub content: String,
    pub x: i32,
    pub y: i32,
}

pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {

    ///
    /// Constructor.
    /// 
    pub fn with_capacity(size: usize) -> Self {
        Self {
            buffer: vec!['\0'; size],
            gap_start: 0,
            gap_end: size,
        }
    }

    /// 
    /// Insert a character at the current cursor position.
    /// 
    pub fn insert(&mut self, c: char) {
        if self.gap_start == self.gap_end {
            self.grow();
        }
        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    /// 
    /// Move the cursor left.
    /// 
    pub fn move_left(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.gap_end -= 1;
            self.buffer[self.gap_end] = self.buffer[self.gap_start];
        }
    }

    /// 
    /// Move the cursor right.
    /// 
    pub fn move_right(&mut self) {
        if self.gap_end < self.buffer.len() {
            self.buffer[self.gap_start] = self.buffer[self.gap_end];
            self.gap_start += 1;
            self.gap_end += 1;
        }
    }

    /// 
    /// Delete a character before the cursor.
    /// 
    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
        }
    }

    /// 
    /// Grow the gap.
    /// 
    fn grow(&mut self) {
        let new_capacity = self.buffer.len() * 2;
        let mut new_buffer = vec!['\0'; new_capacity];
        let gap_size = self.gap_end - self.gap_start;

        // Copy before gap
        new_buffer[..self.gap_start].copy_from_slice(&self.buffer[..self.gap_start]);

        // Copy after gap
        let after_gap_len = self.buffer.len() - self.gap_end;
        let new_gap_end = new_capacity - after_gap_len;
        new_buffer[new_gap_end..].copy_from_slice(&self.buffer[self.gap_end..]);

        self.buffer = new_buffer;
        self.gap_end = new_gap_end;
    }

    /// 
    /// Get the contents as a String.
    /// 
    pub fn contents(&self) -> String {
        self.buffer[..self.gap_start]
            .iter()
            .chain(&self.buffer[self.gap_end..])
            .collect()
    }

    /// 
    /// Get current cursor position.
    /// 
    pub fn cursor(&self) -> usize {
        self.gap_start
    }
}
