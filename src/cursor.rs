pub struct Cursor {
    pub cursor_x: usize,
    pub newlines_seen: usize,
    pub idx_of_last_seen_newline: usize,
    pub offset_left: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            cursor_x: 0,
            newlines_seen: 0,
            idx_of_last_seen_newline: 0,
            offset_left: 0,
        }
    }

    pub fn move_left(&mut self, content: &String) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
            if self.offset_left == 0 {
                let prev_line = content.lines().nth(self.newlines_seen - 1).unwrap();
                self.offset_left = prev_line.len();
            } else {
                self.offset_left -= 1
            }
        }
        if content.chars().nth(self.cursor_x).unwrap() == '\n' {
            self.newlines_seen -= 1;
            self.idx_of_last_seen_newline = self.cursor_x;
        }
    }

    pub fn move_right(&mut self, content: &String) {
        if self.cursor_x < content.len() {
            self.cursor_x += 1;
            self.offset_left += 1;
            if content.chars().nth(self.cursor_x - 1).unwrap() == '\n' {
                self.newlines_seen += 1;
                self.idx_of_last_seen_newline = self.cursor_x - 1;
                self.offset_left = 0;
            }
        }
    }

    pub fn move_down(&mut self, content: &String) {
        let desired_offset_left = self.offset_left;
        let desired_newlines_seen = self.newlines_seen + 1;

        while self.cursor_x < content.len() && (self.newlines_seen < desired_newlines_seen) {
            self.cursor_x += 1;
            self.offset_left += 1;
            if content.chars().nth(self.cursor_x - 1).unwrap() == '\n' {
                self.newlines_seen += 1;
                self.idx_of_last_seen_newline = self.cursor_x - 1;
                self.offset_left = 0;
            }
        }

        let curr_line = content
            .lines()
            .nth(self.newlines_seen)
            .unwrap_or_else(|| "");

        self.offset_left = desired_offset_left.min(curr_line.len());
        self.cursor_x += desired_offset_left.min(curr_line.len());
    }

    pub fn move_up(&mut self, content: &String) {
        if self.newlines_seen == 0 {
            return;
        }
        let desired_offset_left = self.offset_left;
        let desired_newlines_seen = self.newlines_seen - 1;

        while self.cursor_x > 0 && (self.newlines_seen > desired_newlines_seen) {
            self.cursor_x -= 1;
            if self.offset_left == 0 {
                let prev_line = content.lines().nth(self.newlines_seen - 1).unwrap();
                self.offset_left = prev_line.len();
            } else {
                self.offset_left -= 1
            }
            if content.chars().nth(self.cursor_x).unwrap() == '\n' {
                self.newlines_seen -= 1;
                self.idx_of_last_seen_newline = self.cursor_x;
            }
        }

        let curr_line = content
            .lines()
            .nth(self.newlines_seen)
            .unwrap_or_else(|| "");

        self.offset_left = desired_offset_left.min(curr_line.len());
        self.cursor_x -= curr_line.len() - desired_offset_left.min(curr_line.len());
    }
}
