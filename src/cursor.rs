#[derive(Debug)]
pub struct Cursor {
    pub idx: usize,          // cursor idx in string
    pub cursor_y: usize,     // cursor y offset
    pub cursor_x: usize,     // cursor x offset
    desired_cursor_x: usize, // desired cursor x offset
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            idx: 0,
            cursor_y: 0,
            cursor_x: 0,
            desired_cursor_x: 0,
        }
    }

    pub fn move_left(&mut self, content: &String) {
        if self.idx > 0 {
            self.idx -= 1;
            if self.cursor_x == 0 {
                let prev_line = content.lines().nth(self.cursor_y - 1).unwrap();
                self.cursor_x = prev_line.len();
                self.desired_cursor_x = self.cursor_x;
            } else {
                self.cursor_x -= 1;
                self.desired_cursor_x = self.cursor_x;
            }
            if content.chars().nth(self.idx).unwrap() == '\n' {
                self.cursor_y -= 1;
            }
        }
    }

    pub fn move_right(&mut self, content: &String) {
        if self.idx < content.len() {
            self.idx += 1;
            self.cursor_x += 1;
            self.desired_cursor_x = self.cursor_x;
            if content.chars().nth(self.idx - 1).unwrap() == '\n' {
                self.cursor_y += 1;
                self.cursor_x = 0;
                self.desired_cursor_x = self.cursor_x;
            }
        }
    }

    pub fn move_down(&mut self, content: &String) {
        if self.cursor_y > content.split('\n').count() - 1 {
            return;
        }

        if self.cursor_y == content.split('\n').count() - 1 {
            self.cursor_y = content.split('\n').count() - 1;
            self.idx = content.len();
            self.cursor_x = content.split('\n').last().unwrap_or_else(|| "").len();
            return;
        }

        let curr_line = content.lines().nth(self.cursor_y).unwrap();
        let next_line = content.lines().nth(self.cursor_y + 1).unwrap_or_else(|| "");

        self.cursor_y += 1;
        self.idx += curr_line
            .len()
            .min(curr_line.len() - self.cursor_x + next_line.len())
            + 1;
        self.cursor_x = self.cursor_x.min(next_line.len());
    }

    pub fn move_up(&mut self, content: &String) {
        if self.cursor_y == 0 {
            self.cursor_x = 0;
            self.idx = 0;
            return;
        }

        let mut lines = content.lines().skip(self.cursor_y - 1);

        let above_line = lines.next().unwrap();

        self.cursor_y -= 1;
        self.idx -= (above_line.len()).max(self.cursor_x) + 1;
        self.cursor_x = self.cursor_x.min(above_line.len());
    }
}
