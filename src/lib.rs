pub struct Throttler {
    cur: u32,
    lim: u32,
}

impl Throttler {
    pub fn new(lim: u32) -> Throttler {
        Throttler {
            cur: 0,
            lim: lim,
        }
    }

    pub fn throttle<F: FnMut()>(&mut self, mut f: F) {
        if self.cur == 0 {
            f();
        }

        self.cur += 1;
        self.cur %= self.lim;
    }
}
