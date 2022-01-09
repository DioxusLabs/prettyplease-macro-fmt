use crate::algorithm::{self, BeginToken, BreakToken, Breaks, Printer};
use std::borrow::Cow;

impl Printer {
    pub fn ibox(&mut self, indent: isize) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Inconsistent,
        });
    }

    pub fn cbox(&mut self, indent: isize) {
        self.scan_begin(BeginToken {
            offset: indent,
            breaks: Breaks::Consistent,
        });
    }

    pub fn end(&mut self) {
        self.scan_end();
    }

    pub fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        let s = wrd.into();
        self.scan_string(s);
    }

    fn spaces(&mut self, n: usize) {
        self.scan_break(BreakToken {
            offset: 0,
            blank_space: n,
            trailing_comma: false,
            if_nonempty: false,
        });
    }

    pub fn zerobreak(&mut self) {
        self.spaces(0);
    }

    pub fn space(&mut self) {
        self.spaces(1);
    }

    pub fn nbsp(&mut self) {
        self.word(" ");
    }

    pub fn hardbreak(&mut self) {
        self.spaces(algorithm::SIZE_INFINITY as usize);
    }

    pub fn space_if_nonempty(&mut self) {
        self.scan_break(BreakToken {
            offset: 0,
            blank_space: 1,
            trailing_comma: false,
            if_nonempty: true,
        });
    }

    pub fn hardbreak_if_nonempty(&mut self) {
        self.scan_break(BreakToken {
            offset: 0,
            blank_space: algorithm::SIZE_INFINITY as usize,
            trailing_comma: false,
            if_nonempty: true,
        });
    }

    pub fn trailing_comma(&mut self, is_last: bool) {
        if is_last {
            self.scan_break(BreakToken {
                offset: 0,
                blank_space: 0,
                trailing_comma: true,
                if_nonempty: false,
            });
        } else {
            self.word(",");
            self.space();
        }
    }

    pub fn trailing_comma_or_space(&mut self, is_last: bool) {
        if is_last {
            self.scan_break(BreakToken {
                offset: 0,
                blank_space: 1,
                trailing_comma: true,
                if_nonempty: false,
            });
        } else {
            self.word(",");
            self.space();
        }
    }
}
