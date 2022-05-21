use std::ops::Sub;

use crate::Poly;

impl Poly {
    fn sub_impl(mut lhs: Self, mut rhs: Self) -> Self {
        let diff = lhs.partial_degree() - rhs.partial_degree();
        if diff < 0 {
            // flip the order. we want bigger on the lhs
            Self::sub_impl(rhs, lhs)
        } else {
            let diff = diff as usize;

            lhs.normalise_with(&mut rhs);

            for i in diff..lhs.coefs.len() {
                lhs.coefs[i] += rhs.coefs[i - diff];
            }

            lhs.reduce();
            lhs
        }
    }
}

impl Sub for Poly {
    type Output = Poly;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            self
        } else if self.is_zero() {
            rhs
        } else {
            Poly::sub_impl(self, rhs)
        }
    }
}

impl Sub for &Poly {
    type Output = Poly;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            Poly::clone(self)
        } else if self.is_zero() {
            Poly::clone(rhs)
        } else {
            Poly::sub_impl(Poly::clone(self), Poly::clone(rhs))
        }
    }
}

impl Sub<Poly> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        if rhs.is_zero() {
            Poly::clone(self)
        } else if self.is_zero() {
            rhs
        } else {
            Poly::sub_impl(Poly::clone(self), rhs)
        }
    }
}

impl Sub<&Poly> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        if rhs.is_zero() {
            self
        } else if self.is_zero() {
            Poly::clone(rhs)
        } else {
            Poly::sub_impl(self, Poly::clone(rhs))
        }
    }
}
