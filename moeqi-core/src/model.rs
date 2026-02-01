pub const FEAT: usize = 7;

pub struct Model {
    pub e: u16,
    // Wr always f32 for stability (router)
    pub wr: Vec<f32>, // len = e * FEAT
    // We can be quantized in file, but here stored as f32
    pub we: Vec<f32>, // len = e * FEAT
}

impl Model {
    #[inline]
    pub fn wr_row(&self, k: usize) -> &[f32] {
        let e = self.e as usize;
        debug_assert!(k < e);
        let base = k * FEAT;
        &self.wr[base..base + FEAT]
    }

    #[inline]
    pub fn we_row(&self, k: usize) -> &[f32] {
        let e = self.e as usize;
        debug_assert!(k < e);
        let base = k * FEAT;
        &self.we[base..base + FEAT]
    }
}

#[inline]
pub fn dot7(a: &[f32], f: &[f32; FEAT]) -> f32 {
    // Unrolled-ish for consistency
    a[0]*f[0] + a[1]*f[1] + a[2]*f[2] + a[3]*f[3] + a[4]*f[4] + a[5]*f[5] + a[6]*f[6]
}

#[inline]
pub fn router_argmax(model: &Model, f: &[f32; FEAT]) -> usize {
    let e = model.e as usize;
    let mut best_k = 0usize;
    let mut best = f32::NEG_INFINITY;
    for k in 0..e {
        let z = dot7(model.wr_row(k), f);
        if z > best {
            best = z;
            best_k = k;
        }
    }
    best_k
}
