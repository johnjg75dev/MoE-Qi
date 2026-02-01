// moeqi-core/src/codec/quant.rs
#[derive(Debug, Clone)]
pub struct SignedUniformQuant {
    bits: u8,
    step: i16,
}

impl SignedUniformQuant {
    pub fn new(bits: u8) -> Self {
        let bits = bits.max(1).min(15); // keep sane
        let levels = 1i32 << bits;
        let half = (levels / 2) - 1;

        // pick step so half*step >= 255
        let step = ((255 + half - 1) / half).max(1) as i16;
        Self { bits, step }
    }

    #[inline]
    pub fn step(&self) -> i16 {
        self.step
    }

    #[inline]
    pub fn quantize(&self, r: i16) -> i16 {
        let s = self.step as i32;
        let ri = r as i32;
        let sign = if ri > 0 { 1 } else if ri < 0 { -1 } else { 0 };

        // mid-tread rounding toward nearest quant bin
        let q = ((ri + (s / 2) * sign) / s) * s;
        q.clamp(-32768, 32767) as i16
    }

    #[inline]
    pub fn dequantize(&self, q: i16) -> i16 {
        q
    }
}
