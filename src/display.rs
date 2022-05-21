use std::fmt;

use crate::Poly;

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.partial_degree();
        if d == 0 {
            return write!(f, "{}", self.coefs.last().copied().unwrap_or_default());
        }

        let mut first = true;
        for (i, mut c) in self.coefs.iter().copied().enumerate() {
            let power = d - i as isize;
            if c == 0.0 {
                continue;
            }

            if c.is_sign_negative() && first {
                write!(f, "-")?;
                c = -c;
            } else if c.is_sign_negative() && !first {
                write!(f, " - ")?;
                c = -c;
            } else if !first {
                write!(f, " + ")?;
            }
            first = false;

            if c != 1.0 || power == 0 {
                if c.fract() == 0.0 {
                    write!(f, "{}", c as usize)?;
                } else {
                    write!(f, "{}", c)?;
                }
            }

            if power != 0 {
                write!(f, "x")?;
                if power != 1 {
                    write!(f, "^{}", power)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Poly;

    #[test]
    fn positive() {
        // x^4 - 2x^2 + 3.5x
        let quart = Poly {
            coefs: vec![1.0, 0.0, -2.0, 3.5],
            exp: 1,
        };

        assert_eq!(quart.to_string(), "x^4 - 2x^2 + 3.5x");
    }

    #[test]
    fn negative() {
        // -x + x^-1
        let quart = Poly {
            coefs: vec![-1.0, 0.0, 1.0],
            exp: -1,
        };

        assert_eq!(quart.to_string(), "-x + x^-1");
    }
}
