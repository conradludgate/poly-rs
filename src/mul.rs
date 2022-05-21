use std::ops::Mul;

use crate::Poly;

impl Mul<f64> for Poly {
    type Output = Poly;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.coefs.iter_mut().for_each(|c| *c *= rhs);
        self
    }
}

impl Mul<f64> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: f64) -> Self::Output {
        Poly::clone(self) * rhs
    }
}

impl Poly {
    fn mul_impl(lhs: &Self, rhs: &Self) -> Self {
        let l = lhs.coefs.len();
        let m = rhs.coefs.len();
        let mut coefs = Vec::new();
        coefs.resize(l + m - 1, 0.0);

        for i in 0..l {
            for j in 0..m {
                coefs[i + j] += lhs.coefs[i] * rhs.coefs[j];
            }
        }

        Self {
            coefs,
            exp: lhs.exp + rhs.exp,
        }
    }
}

impl Mul for Poly {
    type Output = Poly;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        self.reduce();
        rhs.reduce();
        if self.is_zero() {
            self
        } else if rhs.is_zero() {
            rhs
        } else {
            Poly::mul_impl(&self, &rhs)
        }
    }
}

impl Mul for &Poly {
    type Output = Poly;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Poly::zero();
        }
        Poly::mul_impl(self, rhs)
    }
}

impl Mul<Poly> for &Poly {
    type Output = Poly;

    fn mul(self, mut rhs: Poly) -> Self::Output {
        rhs.reduce();
        if self.is_zero() {
            Poly::zero()
        } else if rhs.is_zero() {
            rhs
        } else {
            Poly::mul_impl(self, &rhs)
        }
    }
}

impl Mul<&Poly> for Poly {
    type Output = Poly;

    fn mul(mut self, rhs: &Poly) -> Self::Output {
        self.reduce();
        if self.is_zero() {
            self
        } else if rhs.is_zero() {
            Poly::zero()
        } else {
            Poly::mul_impl(&self, rhs)
        }
    }
}
