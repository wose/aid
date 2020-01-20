use chrono::prelude::*;

use itertools::iproduct;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Widget};

#[rustfmt::skip]
const DIGITS: [[u8; 15]; 10] = [
    [
        1, 1, 1,
        1, 0, 1,
        1, 0, 1,
        1, 0, 1,
        1, 1, 1,
    ],
    [
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
    ],
    [
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
        1, 0, 0,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
    ],
    [
        1, 0, 1,
        1, 0, 1,
        1, 1, 1,
        0, 0, 1,
        0, 0, 1,
    ],
    [
        1, 1, 1,
        1, 0, 0,
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        1, 0, 0,
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
        0, 0, 1,
    ],
    [
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
    ],
    [
        1, 1, 1,
        1, 0, 1,
        1, 1, 1,
        0, 0, 1,
        1, 1, 1,
    ],
];

pub struct Clock<'a> {
    block: Option<Block<'a>>,
    center: bool,
    datetime: DateTime<Local>,
    style: Style,
}

impl<'a> Default for Clock<'a> {
    fn default() -> Clock<'a> {
        Clock::with_datetime(Local::now())
    }
}

impl<'a> Clock<'a> {
    pub fn with_datetime(datetime: DateTime<Local>) -> Clock<'a> {
        Clock {
            block: None,
            center: false,
            datetime,
            style: Default::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Clock<'a> {
        self.block = Some(block);
        self
    }

    pub fn center(mut self, center: bool) -> Clock<'a> {
        self.center = center;
        self
    }

    pub fn style(mut self, style: Style) -> Clock<'a> {
        self.style = style;
        self
    }

    fn draw_digit(&self, digit: usize, x: u16, y: u16, buf: &mut Buffer) {
        for (dx, dy) in iproduct!(0..6, 0..5) {
            if DIGITS[digit][dx / 2 + dy * 3] == 1 {
                buf.set_string(
                    x + dx as u16,
                    y + dy as u16,
                    tui::symbols::block::FULL,
                    Style::default().fg(Color::Green),
                );
            }
        }
    }
}

impl<'a> Widget for Clock<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(ref mut block) => {
                block.draw(area, buf);
                block.inner(area)
            }
            None => area,
        };

        if area.height < 5 || area.width < 51 {
            return;
        }

        if self.style.bg != Color::Reset {
            self.background(area, buf, self.style.bg);
        }

        let (left, top) = if self.center {
            let left = area.left() + (area.width - 51) / 2;
            let top = area.top() + (area.height - 5) / 2;
            (left, top)
        } else {
            (area.left(), area.top())
        };

        let dt = self.datetime;

        self.draw_digit(dt.hour() as usize / 10, left, top, buf);
        self.draw_digit(dt.hour() as usize % 10, left + 7, top, buf);

        buf.set_string(left + 15, top + 1, "  ", Style::default().bg(Color::Green));
        buf.set_string(left + 15, top + 3, "  ", Style::default().bg(Color::Green));

        self.draw_digit(dt.minute() as usize / 10, left + 19, top, buf);
        self.draw_digit(dt.minute() as usize % 10, left + 26, top, buf);

        buf.set_string(left + 34, top + 1, "  ", Style::default().bg(Color::Green));
        buf.set_string(left + 34, top + 3, "  ", Style::default().bg(Color::Green));

        self.draw_digit(dt.second() as usize / 10, left + 38, top, buf);
        self.draw_digit(dt.second() as usize % 10, left + 45, top, buf);
    }
}
