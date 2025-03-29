pub mod solution {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Formatter};

    pub struct ComplexNumber {
        pub real: f64,
        pub imag: f64,
    }

    #[derive(PartialEq, Debug)]
    pub enum ComplexNumberError {
        ImaginaryNotZero,
    }

    impl TryInto<f64> for ComplexNumber {
        type Error = ComplexNumberError;

        fn try_into(self) -> Result<f64, Self::Error> {
            if self.imag != 0.0 {
                Err(ComplexNumberError::ImaginaryNotZero)
            } else {
                Ok(self.real)
            }
        }
    }

    impl std::fmt::Display for ComplexNumber {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    impl std::ops::AddAssign<ComplexNumber> for ComplexNumber {
        fn add_assign(&mut self, rhs: ComplexNumber) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl std::ops::Add<&ComplexNumber> for &ComplexNumber {
        type Output = ComplexNumber;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl std::ops::Add<&ComplexNumber> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl std::ops::Add<f64> for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: f64) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs,
                imag: self.imag,
            }
        }
    }

    impl Copy for ComplexNumber {}

    impl Clone for ComplexNumber {
        fn clone(&self) -> Self {
            ComplexNumber {
                real: self.real,
                imag: self.imag,
            }
        }
    }

    impl std::ops::Add for ComplexNumber {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            ComplexNumber {
                real: self.real + rhs.real,
                imag: self.imag + rhs.imag,
            }
        }
    }

    impl ComplexNumber {
        pub fn new(p0: f64, p1: f64) -> Self {
            ComplexNumber { real: p0, imag: p1 }
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn from_real(p0: f64) -> Self {
            ComplexNumber {
                real: p0,
                imag: 0.0,
            }
        }

        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real, self.imag)
        }

        pub fn as_slice(&self) -> &[Self] {
            std::slice::from_ref(&self)
        }
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl Debug for ComplexNumber {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "ComplexNumber {{ real: {}, imag: {} }}", self.real, self.imag)
        }
    }


    impl PartialOrd<Self> for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let self_magnitude = (self.real * self.real+self.imag*self.imag).sqrt();
            let other_magnitude = (other.real * other.real+other.imag*other.imag).sqrt();
            if self_magnitude < other_magnitude {
                Some(Ordering::Less)
            } else if self_magnitude > other_magnitude {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    impl Eq for ComplexNumber {}

    impl PartialEq<Self> for ComplexNumber {
        fn eq(&self, other: &Self) -> bool {
            if self.real == other.real && self.imag == other.imag {
                true
            } else {
                false
            }
        }
    }

    impl Ord for ComplexNumber{
        fn cmp(&self, other: &Self) -> Ordering {
            if self > other {
                Ordering::Greater
            } else if self < other {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }

    impl From<f64> for ComplexNumber {
        fn from(value: f64) -> Self {
            ComplexNumber {
                real: value,
                imag: 0.0,
            }
        }
    }

    impl Default for ComplexNumber {
        fn default() -> Self {
            ComplexNumber {
                real: 0.0,
                imag: 0.0,
            }
        }
    }
}
