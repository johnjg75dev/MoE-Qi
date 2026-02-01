pub fn mse_psnr(a: &[u8], b: &[u8]) -> (f64, f64) {
    assert_eq!(a.len(), b.len());
    let mut sum = 0f64;
    for i in 0..a.len() {
        let d = (a[i] as f64) - (b[i] as f64);
        sum += d*d;
    }
    let mse = sum / (a.len() as f64);
    if mse == 0.0 { return (0.0, f64::INFINITY); }
    let psnr = 20.0 * (255.0 / mse.sqrt()).log10();
    (mse, psnr)
}
