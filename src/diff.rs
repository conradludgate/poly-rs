use crate::Poly;

pub trait Differentiate {
    type Output;
    fn diff(self) -> Self::Output;
}

impl Differentiate for Poly {
    type Output = Poly;
    fn diff(mut self) -> Self::Output {
        let d = self.partial_degree();
        for (i, c) in self.coefs.iter_mut().enumerate() {
            *c *= (d - i as isize) as f64;
        }
        self.exp -= 1;
        self.reduce();
        self
    }
}

impl Differentiate for &Poly {
    type Output = Poly;
    fn diff(self) -> Self::Output {
        Poly::clone(self).diff()
    }
}

#[cfg(test)]
mod tests {
    use crate::{diff::Differentiate, Poly};

    #[test]
    fn diff() {
        // x^4 + 2x^2
        let quart = Poly {
            coefs: vec![1.0, 0.0, 2.0, 0.0],
            exp: 1,
        };
        // 4x^3 + 4x
        let cube = Poly {
            coefs: vec![4.0, 0.0, 4.0],
            exp: 1,
        };
        assert_eq!(quart.diff(), cube);
    }
}
