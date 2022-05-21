use std::fmt;

use crate::Poly;

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.degree();
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
