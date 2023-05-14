//! WhyHash is a very simple and easy to understand hash function questionably derived from wyHash and MUM.
//! It should be fast, relying only on a few simple operations.
//! It should also be somewhat safe, producing well distributed hashes and being guarded against a sticky 0 state and zeroing of the internal state except for sheer bad luck.
//! The 0xda942042e4dd58b5 constant is shamelessly copied from Daniel Lemire's blog. What's good enough for him is good enough for me, right?

/// Returned by the function finish2(). It holds two u64 hashes, one of them being guarded against being 0 when cast to an u8.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Finish2 {
    pub hash: u64,
    pub as_u8_nonzero: u64,
}

/// Extension Trait for Hashers, which allows for returning an additional - possibly different - hash,
/// which is guaranteed to be non-zero when cast to an u8
pub trait HasherExt {
    /// Returns a struct of two different hashes, as_u8_nonzero being guarded against being 0 when cast to an u8.
    /// This allows for a datastructure of u8's to efficiently encode e.g. an "empty" state.
    fn finish2(&self) -> Finish2;
}

impl<T: std::hash::Hasher> HasherExt for T {
    fn finish2(&self) -> Finish2 {
        let mut tmp = self.finish();
        if (tmp as u8) == 0 {
            tmp += 1;
        }
        Finish2 {
            hash: self.finish(),
            as_u8_nonzero: tmp,
        }
    }
}

/// WhyHash has two independent states and two secrets, one derived from the other.
/// This allows for the creation of two hashes by consuming the input data only once.
pub struct WhyHash {
    state: u64,
    state2: u64,
    secret: u64,
    secret2: u64,
}

impl WhyHash {
    /// Creates a new instance of WhyHash, which can be used both as Hasher or as BuildHasher.
    /// std::collections::hash_map::Randomstate is the source of randomness.
    /// The generated secret has a somewhat balanced amount of ones and zeroes.

    pub fn new() -> WhyHash {
        let secret = poprand();
        WhyHash {
            state: 0xda942042e4dd58b5,
            state2: 0xda942042e4dd58b5,
            secret,
            secret2: whymum(0xda942042e4dd58b5, secret, 0),
        }
    }

    pub fn finish2(&self) -> Finish2 {
        Finish2 {
            hash: self.state,
            as_u8_nonzero: self.state2,
        }
    }
}

impl Default for WhyHash {
    fn default() -> Self {
        Self::new()
    }
}

impl std::hash::BuildHasher for WhyHash {
    type Hasher = WhyHash;

    /// WhyHash is its own BuildHasher. Therefore any instance of WhyHash can return an instance of itself
    /// which is able to produce the same hashes for the same input as the original.
    #[inline(always)]
    fn build_hasher(&self) -> WhyHash {
        WhyHash {
            state: 0xda942042e4dd58b5,
            state2: 0xda942042e4dd58b5,
            secret: self.secret,
            secret2: self.secret2,
        }
    }
}

impl std::hash::Hasher for WhyHash {
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        // Consume the input data in chunks of 8 Bytes. The remaining bytes are consumed later.
        // The chunks of 8 bytes are fed into a MUM random number generator, guarded against a
        // sticky 0 state and zeroing with known bad data. Not great, but good enough for me.
        // This should prevent any HashDOS.(?) State and State2 are treated identical except for their secret.
        let mut byteiter = bytes.chunks_exact(8);
        for octets in &mut byteiter {
            self.state = whymum(
                u64::from_le_bytes(octets.try_into().expect("chunks_exact failed")),
                self.state,
                self.secret,
            );
            self.state2 = whymum(
                u64::from_le_bytes(octets.try_into().expect("chunks_exact failed")),
                self.state2,
                self.secret2,
            );
        }

        // Consume any possibly remaining bytes left by ORing them onto an zeroed u64 and rotating it.
        let mut tmp: u64 = 0;
        for byte in byteiter.remainder() {
            tmp = tmp.rotate_left(8);
            tmp |= u64::from(*byte);
        }

        // Feed the remaining bytes into the MUM prng
        self.state = whymum(tmp, self.state, self.secret);
        self.state2 = whymum(tmp, self.state2, self.secret2);

        // Guard state2 against being 0 when cast to an u8
        if (self.state2 as u8) == 0 {
            self.state2 += 1;
        }
    }

    #[inline(always)]
    fn finish(&self) -> u64 {
        self.state
    }
}

// The MUM random number generator has quite amazing collision properties.
// XORing the secret onto the state and input data guards it against sticky zeroes and zeroing of its state,
// but makes collisions slightly worse in my simple tests. Nevertheless on a simplicity,
// performance and safety Venn diagram the dot representing it looks like a circle.
#[inline(always)]
fn whymum(cleardat: u64, state: u64, secret: u64) -> u64 {
    let tmp: u128 = u128::from(state ^ secret) * u128::from(cleardat ^ secret);
    ((tmp >> 64) ^ tmp) as u64
}

// A simple source of randomness, guarded against a too unbalanced population counts.
fn poprand() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut num: u64 = RandomState::new().build_hasher().finish();
    loop {
        if num.count_ones() >= 24 && num.count_ones() <= 40 {
            return num;
        } else {
            num = RandomState::new().build_hasher().finish();
        }
    }
}
