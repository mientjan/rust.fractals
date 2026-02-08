use rug::Float;

/// Arbitrary-precision complex number for perturbation theory reference orbits.
pub struct BigComplex {
    pub re: Float,
    pub im: Float,
}

impl BigComplex {
    pub fn new(re: Float, im: Float) -> Self {
        BigComplex { re, im }
    }

    pub fn from_str(re_str: &str, im_str: &str, precision: u32) -> Self {
        let re = Float::parse(re_str)
            .map(|parsed| Float::with_val(precision, parsed))
            .unwrap_or_else(|_| Float::with_val(precision, 0.0));
        let im = Float::parse(im_str)
            .map(|parsed| Float::with_val(precision, parsed))
            .unwrap_or_else(|_| Float::with_val(precision, 0.0));
        BigComplex { re, im }
    }

    pub fn precision(&self) -> u32 {
        self.re.prec()
    }

    /// z = z^2 + c, in place. Returns self for chaining.
    pub fn square_add_assign(&mut self, c: &BigComplex) {
        let prec = self.precision();
        // (a + bi)^2 = a^2 - b^2 + 2abi
        let re2 = Float::with_val(prec, &self.re * &self.re);
        let im2 = Float::with_val(prec, &self.im * &self.im);
        let cross = Float::with_val(prec, &self.re * &self.im);

        self.re = Float::with_val(prec, &re2 - &im2);
        self.re += &c.re;
        self.im = Float::with_val(prec, &cross * 2.0);
        self.im += &c.im;
    }

    /// |z|^2 as f64 (for escape check)
    #[allow(dead_code)]
    pub fn norm_sq_f64(&self) -> f64 {
        let re = self.re.to_f64();
        let im = self.im.to_f64();
        re * re + im * im
    }

    /// Convert to (f64, f64) pair
    pub fn to_f64_pair(&self) -> (f64, f64) {
        (self.re.to_f64(), self.im.to_f64())
    }
}
