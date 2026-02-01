// crates/moeqi-core/src/color.rs

#[inline] fn clamp_u8(x: i32) -> u8 {
    if x < 0 { 0 } else if x > 255 { 255 } else { x as u8 }
}

/// Convert RGB888 -> planar Y, Cb, Cr (all full-res)
pub fn rgb_to_ycbcr_planar(rgb: &[u8], w: usize, h: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    assert_eq!(rgb.len(), w*h*3);
    let mut y  = vec![0u8; w*h];
    let mut cb = vec![0u8; w*h];
    let mut cr = vec![0u8; w*h];

    for i in 0..(w*h) {
        let r = rgb[i*3+0] as i32;
        let g = rgb[i*3+1] as i32;
        let b = rgb[i*3+2] as i32;

        // BT.601-ish integer approximation
        let yy  = (  77*r + 150*g +  29*b) >> 8;
        let cbb = ((-43*r -  85*g + 128*b) >> 8) + 128;
        let crr = ((128*r - 107*g -  21*b) >> 8) + 128;

        y[i]  = clamp_u8(yy);
        cb[i] = clamp_u8(cbb);
        cr[i] = clamp_u8(crr);
    }
    (y, cb, cr)
}

/// Convert planar YCbCr (full-res) -> RGB888
pub fn ycbcr_to_rgb_planar(y: &[u8], cb: &[u8], cr: &[u8], w: usize, h: usize) -> Vec<u8> {
    assert_eq!(y.len(), w*h);
    assert_eq!(cb.len(), w*h);
    assert_eq!(cr.len(), w*h);

    let mut rgb = vec![0u8; w*h*3];
    for i in 0..(w*h) {
        let yy = y[i] as i32;
        let cbb = cb[i] as i32 - 128;
        let crr = cr[i] as i32 - 128;

        // Inverse approx
        let r = yy + ((359 * crr) >> 8);
        let g = yy - (( 88 * cbb + 183 * crr) >> 8);
        let b = yy + ((454 * cbb) >> 8);

        rgb[i*3+0] = clamp_u8(r);
        rgb[i*3+1] = clamp_u8(g);
        rgb[i*3+2] = clamp_u8(b);
    }
    rgb
}

/// 4:2:0 downsample (box filter) Cb/Cr full-res -> half-res
pub fn downsample_420(ch: &[u8], w: usize, h: usize) -> (Vec<u8>, usize, usize) {
    let w2 = (w + 1) / 2;
    let h2 = (h + 1) / 2;
    let mut out = vec![0u8; w2*h2];

    for y2 in 0..h2 {
        for x2 in 0..w2 {
            let mut sum = 0u32;
            let mut cnt = 0u32;
            for dy in 0..2 {
                for dx in 0..2 {
                    let x = x2*2 + dx;
                    let y = y2*2 + dy;
                    if x < w && y < h {
                        sum += ch[y*w + x] as u32;
                        cnt += 1;
                    }
                }
            }
            out[y2*w2 + x2] = (sum / cnt) as u8;
        }
    }

    (out, w2, h2)
}

/// Nearest upsample half-res -> full-res
pub fn upsample_420_nn(ch_small: &[u8], w2: usize, h2: usize, w: usize, h: usize) -> Vec<u8> {
    let mut out = vec![0u8; w*h];
    for y in 0..h {
        for x in 0..w {
            out[y*w + x] = ch_small[(y/2)*w2 + (x/2)];
        }
    }
    out
}
