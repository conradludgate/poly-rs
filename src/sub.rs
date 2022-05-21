use std::ops::Sub;

use crate::Poly;

impl Sub for Poly {
    type Output = Poly;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        let diff = self.degree() - rhs.degree();
        if diff < 0 {
            // flip the order. we want bigger on the lhs
            return rhs - self;
        }
        let diff = diff as usize;

        self.normalise_with(&mut rhs);

        for i in diff..self.coefs.len() {
            self.coefs[i] -= rhs.coefs[i - diff];
        }

        self.reduce();
        self
    }
}
impl Sub for &Poly {
    type Output = Poly;

    fn sub(self, rhs: Self) -> Self::Output {
        Poly::clone(self) - Poly::clone(rhs)
    }
}
impl Sub<Poly> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::clone(self) - rhs
    }
}
impl Sub<&Poly> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        self - Poly::clone(rhs)
    }
}
