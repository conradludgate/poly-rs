mod add;
mod diff;
mod display;
mod mul;
mod sub;

#[derive(Debug, Clone)]
pub struct Poly {
    coefs: Vec<f64>,
    exp: isize,
}

impl PartialEq for Poly {
    fn eq(&self, other: &Self) -> bool {
        self.coefs == other.coefs && self.exp == other.exp
    }
}

impl Default for Poly {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<f64> for Poly {
    fn from(x: f64) -> Self {
        if x == 0.0 {
            Poly::zero()
        } else {
            Self {
                coefs: vec![x],
                exp: 0,
            }
        }
    }
}

impl Poly {
    pub fn eval(&self, x: f64) -> f64 {
        let mut total = 0.0;
        let mut y = x.powi(self.exp as i32);
        for &c in self.coefs.iter().rev() {
            total += c * y;
            y *= x;
        }
        total
    }

    pub const fn zero() -> Self {
        Self {
            coefs: Vec::new(),
            exp: 1,
        }
    }

    pub fn x() -> Self {
        Self {
            coefs: vec![1.0],
            exp: 1,
        }
    }

    /// Determines the [degree of the polynomial](https://en.wikipedia.org/wiki/Degree_of_a_polynomial).
    /// Returns `None` for the zero-polynomial.
    pub fn degree(&self) -> Option<isize> {
        if self.is_zero() {
            None
        } else {
            Some(self.partial_degree())
        }
    }
    /// Determines the [degree of the polynomial](https://en.wikipedia.org/wiki/Degree_of_a_polynomial).
    /// Returns an undefined value for the zero-polynomial.
    pub fn partial_degree(&self) -> isize {
        (self.coefs.len() as isize) + self.exp - 1
    }

    /// Converts both polynomials to have the same exponent.
    fn normalise_with(&mut self, other: &mut Self) {
        self.reduce();
        other.reduce();
        match self.exp.cmp(&other.exp) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Less => {
                let len = other.coefs.len() + (other.exp - self.exp) as usize;
                other.coefs.resize(len, 0.0);
                other.exp = self.exp;
            }
            std::cmp::Ordering::Greater => {
                let len = self.coefs.len() + (self.exp - other.exp) as usize;
                self.coefs.resize(len, 0.0);
                self.exp = other.exp;
            }
        }
    }

    /// Reduces the polynomial to have the least amount of trailing and leading zeros
    fn reduce(&mut self) {
        // trim trailing 0s (increasing the exp)
        let mut len = self.coefs.len();
        for c in self.coefs.iter().rev() {
            if *c == 0.0 {
                len -= 1;
                self.exp += 1;
                continue;
            }
            break;
        }
        self.coefs.truncate(len);

        // trim leading 0s
        let mut leading = 0;
        for c in self.coefs.iter() {
            if *c == 0.0 {
                leading += 1;
                continue;
            }
            break;
        }
        self.coefs.drain(..leading);
    }

    fn is_zero(&self) -> bool {
        self.coefs.iter().all(|c| *c == 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Poly;

    #[test]
    fn from_f64() {
        let x = Poly::from(1.0);
        assert_eq!(x.partial_degree(), 0);
        assert_eq!(x.coefs, [1.0]);

        let y = Poly::from(0.0);
        assert_eq!(y.partial_degree(), 0);
        assert!(y.coefs.is_empty());
    }

    #[test]
    fn reduce() {
        // x^2 + 0 + 0
        let mut quad = Poly {
            coefs: vec![1.0, 0.0, 0.0],
            exp: 0,
        };
        quad.reduce();
        assert_eq!(quad.coefs, [1.0]);
        assert_eq!(quad.exp, 2);

        // x^4 + 0 + x^2 + 0
        let mut quart = Poly {
            coefs: vec![1.0, 0.0, 1.0, 0.0],
            exp: 1,
        };
        quart.reduce();
        assert_eq!(quart.coefs, [1.0, 0.0, 1.0]);
        assert_eq!(quart.exp, 2);
    }

    #[test]
    fn normalise() {
        // x^2
        let mut quad = Poly {
            coefs: vec![1.0, 0.0, 0.0],
            exp: 0,
        };
        // x^4 + x^2
        let mut quart = Poly {
            coefs: vec![1.0, 0.0, 1.0, 0.0],
            exp: 1,
        };

        quad.normalise_with(&mut quart);

        assert_eq!(quad.coefs, [1.0]);
        assert_eq!(quad.exp, 2);

        assert_eq!(quart.coefs, [1.0, 0.0, 1.0]);
        assert_eq!(quart.exp, 2);

        // x^2
        let mut quad = Poly {
            coefs: vec![1.0],
            exp: 2,
        };
        // x^4 + x^2
        let mut quart = Poly {
            coefs: vec![1.0, 0.0, 1.0, 0.0],
            exp: 1,
        };

        quad.normalise_with(&mut quart);

        assert_eq!(quad.coefs, [1.0]);
        assert_eq!(quad.exp, 2);

        assert_eq!(quart.coefs, [1.0, 0.0, 1.0]);
        assert_eq!(quart.exp, 2);
    }

    #[test]
    fn ops() {
        let x = Poly::x();
        assert_eq!(&x + &x, &x * 2.0);

        let y = &x * &x + &x;
        assert_eq!(y.to_string(), "x^2 + x");
    }

    #[test]
    fn eval() {
        // x^4 + 0 + 2x^2 + 0
        let quart = Poly {
            coefs: vec![1.0, 0.0, 2.0, 0.0],
            exp: 1,
        };
        assert_eq!(quart.eval(4.0), 288.0);
    }
}
