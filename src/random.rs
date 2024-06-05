use std::sync::Mutex;
use std::time::SystemTime;

// Implement xoshiro256ss from https://en.wikipedia.org/wiki/Xorshift
pub(crate) struct PseudoRandom {
    s: [u64; 4],
}

impl PseudoRandom {
    #[inline]
    fn seed(&mut self) {
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(t) => t.as_nanos(),
            Err(_) => 0,
        };
        self.s = [
            (now ^ 4690481050117892527) as u64,
            ((now * 50000) ^ 13682126131931052725) as u64,
            (now ^ 9639264971936262885) as u64,
            (now ^ 6412797481073129502) as u64,
        ];
    }

    #[inline]
    pub(crate) fn next_u64(&mut self) -> u64 {
        if self.s[0] == 0 {
            self.seed();
        }
        let next = self.s[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);
        let v = self.s[1] << 17;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= v;
        self.s[3] = self.s[3].rotate_left(45);
        next
    }

    #[inline]
    pub(crate) fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /*
    pub fn new_with_seed(seed: [u64; 4]) -> PseudoRandom {
        PseudoRandom { s: seed }
    }

    pub fn new() -> PseudoRandom {
        PseudoRandom { s: [0, 0, 0, 0] }
    }
    */
}

static RND: Mutex<PseudoRandom> = Mutex::new(PseudoRandom { s: [0, 0, 0, 0] });

#[inline]
pub(crate) fn next_u32() -> u32 {
    RND.lock().unwrap().next_u32()
}

#[inline]
pub(crate) fn next_u64() -> u64 {
    RND.lock().unwrap().next_u64()
}

/*
#[inline]
pub(crate) fn next_usize() -> usize {
    RND.lock().unwrap().next_u64() as usize
}
*/
