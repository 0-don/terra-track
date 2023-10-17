use std::convert::TryInto;

pub struct RangeIterator {
    active: bool,
    normalized_end: u32,
    normalized_first_pick: u32,
    normalized_pick: u32,
    actual_start: u32,
    step: u32,
}

impl RangeIterator {
    pub fn new(start: u32, end: u32) -> Self {
        let normalized_end = end - start;
        let step = pick_coprime(normalized_end);

        let normalized_first_pick = (normalized_end / 2) % normalized_end; // A deterministic pick, replace with actual randomness if needed.

        Self {
            active: true,
            normalized_end,
            step,
            normalized_first_pick,
            normalized_pick: normalized_first_pick,
            actual_start: start,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.active {
            return None;
        }

        let current_pick = self.normalized_pick;
        let next_pick = (current_pick + self.step) % self.normalized_end;

        if next_pick == self.normalized_first_pick {
            self.active = false;
        }

        self.normalized_pick = next_pick;
        Some(
            (self.actual_start + current_pick)
                .try_into()
                .expect("Could not convert u32 to u16"),
        )
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn pick_coprime(end: u32) -> u32 {
    let mut candidate = (end / 2) % end; // A deterministic pick, replace with actual randomness if needed.

    for _ in 0..10 {
        if gcd(end, candidate) == 1 {
            return candidate;
        }
        candidate = (candidate + 1) % end; // Simply incrementing, replace with actual randomness if needed.
    }

    end - 1
}
