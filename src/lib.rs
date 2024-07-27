#![no_std]

use core::ops::BitXor;

pub struct BitTwidHash {
    init: [u64; 2],
    state: [u64; 2],
}

impl core::hash::Hasher for BitTwidHash {
    #[inline(always)]
    fn finish(&self) -> u64 {
        let mut s0 = self.state[0];
        let mut s1 = self.state[1];
        s1 ^= s0;
        s0 = s0.rotate_left(24).bitxor(s1).bitxor(s1 << 16);
        s1 = s1.rotate_left(37);
        s0.wrapping_add(s1).rotate_left(17).wrapping_add(s0)
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self._write_v2(bytes);
    }
}
#[cfg(feature = "std")]
impl Default for BitTwidHash {
    fn default() -> Self {
        Self::new()
    }
}

impl core::hash::BuildHasher for BitTwidHash {
    type Hasher = BitTwidHash;

    #[inline]
    fn build_hasher(&self) -> BitTwidHash {
        BitTwidHash {
            init: self.init,
            state: self.init,
        }
    }
}

impl BitTwidHash {
    #[cfg(not(feature = "std"))]
    #[inline]
    pub unsafe fn new() -> BitTwidHash {
        let init: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];

        BitTwidHash { init, state: init }
    }

    #[cfg(feature = "std")]
    #[inline]
    pub fn new() -> BitTwidHash {
        extern crate std;
        use core::hash::{BuildHasher, Hasher};
        BitTwidHash::new_with_secret(std::hash::RandomState::new().build_hasher().finish())
    }

    #[inline]
    pub fn new_with_iv(init: [u64; 2]) -> BitTwidHash {
        BitTwidHash { init, state: init }
    }

    #[inline]
    pub fn new_with_secret(secret: u64) -> BitTwidHash {
        let mut init: [u64; 2] = [0; 2];
        let mut s: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];
        s[0] ^= secret;
        for e in init.iter_mut() {
            for _i in 0..20 {
                rnd_xsr256(&mut s);
            }

            for _i in 0..1000 {
                rnd_xsr256(&mut s);
                *e = s[0].wrapping_add(s[3]).rotate_left(23).wrapping_add(s[0]);
                let co = e.count_ones();
                if co <= 40 && co >= 24 {
                    break;
                }
            }
        }
        BitTwidHash { init, state: init }
    }

    // Very simple write variant, that reads the largest possible primitive type remaining from the rest of the bytes slice
    #[inline(always)]
    fn _write_v1(&mut self, mut bytes: &[u8]) {
        while bytes.len() >= 8 {
            let tmp: [u8; 8] = bytes[..8].try_into().unwrap();
            self.gather(u64::from_ne_bytes(tmp));
            bytes = &bytes[8..];
        }

        if bytes.len() > 0 {
            let mut tmp: u64 = 0;

            if (bytes.len() & 4) > 0 {
                let a = bytes[..4].try_into().unwrap();
                tmp |= u32::from_ne_bytes(a) as u64;
                bytes = &bytes[4..];
            }
            if (bytes.len() & 2) > 0 {
                let a = bytes[..2].try_into().unwrap();
                tmp |= (u16::from_ne_bytes(a) as u64) << 32;
                bytes = &bytes[2..];
            }
            if (bytes.len() & 1) > 0 {
                tmp |= (bytes[0] as u64) << 48;
            }

            self.gather(tmp);
        }
    }

    //ZwoHash's write variant is much faster, but can overlap bytes
    #[inline(always)]
    fn _write_v2(&mut self, bytes: &[u8]) {
        if bytes.len() >= 8 {
            let mut bytes_left = bytes;

            while bytes_left.len() > 8 {
                let full_chunk: [u8; 8] = bytes_left[..8].try_into().unwrap();
                self.gather(u64::from_ne_bytes(full_chunk));
                bytes_left = &bytes_left[8..];
            }

            if bytes.len() >= 8 {
                let last_chunk: [u8; 8] = bytes[bytes.len() - 8..].try_into().unwrap();
                self.gather(u64::from_ne_bytes(last_chunk));
            } else {
                core::unreachable!();
            }
        } else if bytes.len() >= 4 {
            let chunk_low: [u8; 4] = bytes[..4].try_into().unwrap();
            let chunk_high: [u8; 4] = bytes[bytes.len() - 4..].try_into().unwrap();
            let chunk_value = (u32::from_ne_bytes(chunk_low) as u64)
                | ((u32::from_ne_bytes(chunk_high) as u64) << 32);
            self.gather(chunk_value);
        } else if bytes.len() >= 2 {
            let chunk_low: [u8; 2] = bytes[..2].try_into().unwrap();
            let chunk_high: [u8; 2] = bytes[bytes.len() - 2..].try_into().unwrap();
            let chunk_value = (u16::from_ne_bytes(chunk_low) as u64)
                | ((u16::from_ne_bytes(chunk_high) as u64) << 16);
            self.gather(chunk_value);
        } else if bytes.len() >= 1 {
            self.gather(bytes[0] as u64);
        }
    }

    #[inline(always)]
    fn gather(&mut self, cleardat: u64) {
        self.state[1] = self.state[0].wrapping_add(self.state[1]);
        self.state[0] = self.state[0].wrapping_add(cleardat);
        self.state[1] = self.state[1].rotate_left(19);
    }
}

fn rnd_xsr256(s: &mut [u64; 4]) {
    let t = s[1] << 17;

    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];

    s[2] ^= t;
    s[3] = s[3].rotate_left(45);
}
