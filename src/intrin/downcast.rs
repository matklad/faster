use stdsimd::simd::*;
use stdsimd::vendor::*;
use intrin::transmute::*;

pub trait Downcast<T> {
    fn saturating_downcast(self, other: Self) -> T;
}

impl Downcast<i16x8> for i32x4 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> i16x8 {
        unsafe { _mm_packs_epi32(self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> i16x8 {
        i16x8::new(self.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                   self.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                   self.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                   self.extract(3).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                   other.extract(3).min(0x00007FFF).max(-0x00008000) as i16)
    }
}

impl Downcast<i8x16> for i16x8 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> i8x16 {
        unsafe { _mm_packs_epi16(self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> i8x16 {
        i8x16::new(self.extract(0).min(0x007F).max(-0x0080) as i8,
                   self.extract(1).min(0x007F).max(-0x0080) as i8,
                   self.extract(2).min(0x007F).max(-0x0080) as i8,
                   self.extract(3).min(0x007F).max(-0x0080) as i8,
                   self.extract(4).min(0x007F).max(-0x0080) as i8,
                   self.extract(5).min(0x007F).max(-0x0080) as i8,
                   self.extract(6).min(0x007F).max(-0x0080) as i8,
                   self.extract(7).min(0x007F).max(-0x0080) as i8,
                   other.extract(0).min(0x007F).max(-0x0080) as i8,
                   other.extract(1).min(0x007F).max(-0x0080) as i8,
                   other.extract(2).min(0x007F).max(-0x0080) as i8,
                   other.extract(3).min(0x007F).max(-0x0080) as i8,
                   other.extract(4).min(0x007F).max(-0x0080) as i8,
                   other.extract(5).min(0x007F).max(-0x0080) as i8,
                   other.extract(6).min(0x007F).max(-0x0080) as i8,
                   other.extract(7).min(0x007F).max(-0x0080) as i8)
    }
}

impl Downcast<u16x8> for u32x4 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn saturating_downcast(self, other: Self) -> u16x8 {
    //     unsafe { _mm_packus_epi32(self, other) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn saturating_downcast(self, other: Self) -> u16x8 {
        u16x8::new(self.extract(0).min(0x0000FFFF) as u16,
                   self.extract(1).min(0x0000FFFF) as u16,
                   self.extract(2).min(0x0000FFFF) as u16,
                   self.extract(3).min(0x0000FFFF) as u16,
                   other.extract(0).min(0x0000FFFF) as u16,
                   other.extract(1).min(0x0000FFFF) as u16,
                   other.extract(2).min(0x0000FFFF) as u16,
                   other.extract(3).min(0x0000FFFF) as u16)
    }
}

impl Downcast<u8x16> for u16x8 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> u8x16 {
        unsafe { _mm_packus_epi16(self.be_i16s(), other.be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> u8x16 {
        u8x16::new(self.extract(0).min(0x00FF) as u8,
                   self.extract(1).min(0x00FF) as u8,
                   self.extract(2).min(0x00FF) as u8,
                   self.extract(3).min(0x00FF) as u8,
                   self.extract(4).min(0x00FF) as u8,
                   self.extract(5).min(0x00FF) as u8,
                   self.extract(6).min(0x00FF) as u8,
                   self.extract(7).min(0x00FF) as u8,
                   other.extract(0).min(0x00FF) as u8,
                   other.extract(1).min(0x00FF) as u8,
                   other.extract(2).min(0x00FF) as u8,
                   other.extract(3).min(0x00FF) as u8,
                   other.extract(4).min(0x00FF) as u8,
                   other.extract(5).min(0x00FF) as u8,
                   other.extract(6).min(0x00FF) as u8,
                   other.extract(7).min(0x00FF) as u8)
    }
}

impl Downcast<i16x16> for i32x8 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> i16x16 {
        unsafe { _mm256_packs_epi32(self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> i16x16 {
        i16x16::new(self.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(3).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(4).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(5).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(6).min(0x00007FFF).max(-0x00008000) as i16,
                    self.extract(7).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(0).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(1).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(2).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(3).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(4).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(5).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(6).min(0x00007FFF).max(-0x00008000) as i16,
                    other.extract(7).min(0x00007FFF).max(-0x00008000) as i16)
    }
}

impl Downcast<i8x32> for i16x16 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> i8x32 {
        unsafe { _mm256_packs_epi16(self, other) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> i8x32 {
        i8x32::new(self.extract(0).min(0x007F).max(-0x0080) as i8,
                   self.extract(1).min(0x007F).max(-0x0080) as i8,
                   self.extract(2).min(0x007F).max(-0x0080) as i8,
                   self.extract(3).min(0x007F).max(-0x0080) as i8,
                   self.extract(4).min(0x007F).max(-0x0080) as i8,
                   self.extract(5).min(0x007F).max(-0x0080) as i8,
                   self.extract(6).min(0x007F).max(-0x0080) as i8,
                   self.extract(7).min(0x007F).max(-0x0080) as i8,
                   self.extract(8).min(0x007F).max(-0x0080) as i8,
                   self.extract(9).min(0x007F).max(-0x0080) as i8,
                   self.extract(10).min(0x007F).max(-0x0080) as i8,
                   self.extract(11).min(0x007F).max(-0x0080) as i8,
                   self.extract(12).min(0x007F).max(-0x0080) as i8,
                   self.extract(13).min(0x007F).max(-0x0080) as i8,
                   self.extract(14).min(0x007F).max(-0x0080) as i8,
                   self.extract(15).min(0x007F).max(-0x0080) as i8,
                   other.extract(0).min(0x007F).max(-0x0080) as i8,
                   other.extract(1).min(0x007F).max(-0x0080) as i8,
                   other.extract(2).min(0x007F).max(-0x0080) as i8,
                   other.extract(3).min(0x007F).max(-0x0080) as i8,
                   other.extract(4).min(0x007F).max(-0x0080) as i8,
                   other.extract(5).min(0x007F).max(-0x0080) as i8,
                   other.extract(6).min(0x007F).max(-0x0080) as i8,
                   other.extract(7).min(0x007F).max(-0x0080) as i8,
                   other.extract(8).min(0x007F).max(-0x0080) as i8,
                   other.extract(9).min(0x007F).max(-0x0080) as i8,
                   other.extract(10).min(0x007F).max(-0x0080) as i8,
                   other.extract(11).min(0x007F).max(-0x0080) as i8,
                   other.extract(12).min(0x007F).max(-0x0080) as i8,
                   other.extract(13).min(0x007F).max(-0x0080) as i8,
                   other.extract(14).min(0x007F).max(-0x0080) as i8,
                   other.extract(15).min(0x007F).max(-0x0080) as i8)
    }
}

impl Downcast<u16x16> for u32x8 {
    // Blocked by stdsimd
    // #[inline(always)]
    // #[cfg(target_feature = "sse4.1")]
    // fn saturating_downcast(self, other: Self) -> u16x16 {
    //     unsafe { _mm256_packus_epi32(self, other) }
    // }

    #[inline(always)]
    // #[cfg(not(target_feature = "sse4.1"))]
    fn saturating_downcast(self, other: Self) -> u16x16 {
        u16x16::new(self.extract(0).min(0x0000FFFF) as u16,
                    self.extract(1).min(0x0000FFFF) as u16,
                    self.extract(2).min(0x0000FFFF) as u16,
                    self.extract(3).min(0x0000FFFF) as u16,
                    self.extract(4).min(0x0000FFFF) as u16,
                    self.extract(5).min(0x0000FFFF) as u16,
                    self.extract(6).min(0x0000FFFF) as u16,
                    self.extract(7).min(0x0000FFFF) as u16,
                    other.extract(0).min(0x0000FFFF) as u16,
                    other.extract(1).min(0x0000FFFF) as u16,
                    other.extract(2).min(0x0000FFFF) as u16,
                    other.extract(3).min(0x0000FFFF) as u16,
                    other.extract(4).min(0x0000FFFF) as u16,
                    other.extract(5).min(0x0000FFFF) as u16,
                    other.extract(6).min(0x0000FFFF) as u16,
                    other.extract(7).min(0x0000FFFF) as u16)
    }
}

impl Downcast<u8x32> for u16x16 {
    #[inline(always)]
    #[cfg(target_feature = "sse2")]
    fn saturating_downcast(self, other: Self) -> u8x32 {
        unsafe { _mm256_packus_epi16(self.be_i16s(), other.be_i16s()) }
    }

    #[inline(always)]
    #[cfg(not(target_feature = "sse2"))]
    fn saturating_downcast(self, other: Self) -> u8x32 {
        u8x32::new(self.extract(0).min(0x00FF) as u8,
                   self.extract(1).min(0x00FF) as u8,
                   self.extract(2).min(0x00FF) as u8,
                   self.extract(3).min(0x00FF) as u8,
                   self.extract(4).min(0x00FF) as u8,
                   self.extract(5).min(0x00FF) as u8,
                   self.extract(6).min(0x00FF) as u8,
                   self.extract(7).min(0x00FF) as u8,
                   self.extract(8).min(0x00FF) as u8,
                   self.extract(9).min(0x00FF) as u8,
                   self.extract(10).min(0x00FF) as u8,
                   self.extract(11).min(0x00FF) as u8,
                   self.extract(12).min(0x00FF) as u8,
                   self.extract(13).min(0x00FF) as u8,
                   self.extract(14).min(0x00FF) as u8,
                   self.extract(15).min(0x00FF) as u8,
                   other.extract(0).min(0x00FF) as u8,
                   other.extract(1).min(0x00FF) as u8,
                   other.extract(2).min(0x00FF) as u8,
                   other.extract(3).min(0x00FF) as u8,
                   other.extract(4).min(0x00FF) as u8,
                   other.extract(5).min(0x00FF) as u8,
                   other.extract(6).min(0x00FF) as u8,
                   other.extract(7).min(0x00FF) as u8,
                   other.extract(8).min(0x00FF) as u8,
                   other.extract(9).min(0x00FF) as u8,
                   other.extract(10).min(0x00FF) as u8,
                   other.extract(11).min(0x00FF) as u8,
                   other.extract(12).min(0x00FF) as u8,
                   other.extract(13).min(0x00FF) as u8,
                   other.extract(14).min(0x00FF) as u8,
                   other.extract(15).min(0x00FF) as u8)
    }
}
