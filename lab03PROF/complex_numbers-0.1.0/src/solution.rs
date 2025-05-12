use std::{fmt::Display, hash::{Hash, Hasher}, ops::{Add, AddAssign}};

#[derive(Debug)]
pub struct ComplexNumber {
    real: f64,
    imag: f64,
}

impl ComplexNumber {
    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real, imag }
    }

    pub fn from_real(real: f64) -> ComplexNumber {
        ComplexNumber { real, imag: 0.0 }
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }

    pub fn to_tuple(&self) -> (f64, f64) {
        (self.real, self.imag)
    }

    pub fn modulus(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Add<&ComplexNumber> for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}


impl Add<f64> for ComplexNumber {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        ComplexNumber {
            real: self.real + rhs,
            ..self
        }
    }
}

impl Add<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: &ComplexNumber) -> Self::Output {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}


impl Clone for ComplexNumber {
    fn clone(&self) -> Self {
        ComplexNumber { ..*self }
    }
}

impl Copy for ComplexNumber {}

impl Default for ComplexNumber {
    fn default() -> Self {
        ComplexNumber::new(0.0, 0.0)
    }
}

// commented out because it's covered by TryInto
//impl Into<f64> for ComplexNumber {
//
//    fn into(self) -> f64 {
//        if self.imag != 0.0 {
//            panic!("imag is not zero");
//        }
//        return self.real;
//    }
//    
//}

#[derive(Debug, PartialEq)]
pub enum ComplexNumberError {
    ImaginaryNotZero,
}

impl TryInto<f64> for ComplexNumber {
    type Error = ComplexNumberError;

    fn try_into(self) -> Result<f64, Self::Error> {
        if self.imag != 0.0 {
            return Err(ComplexNumberError::ImaginaryNotZero);
        }
        return Ok(self.real);
    }
}

impl Into<ComplexNumber> for f64 {
    fn into(self) -> ComplexNumber {
        ComplexNumber::from_real(self)
    }
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

impl Eq for ComplexNumber {}

impl PartialOrd for ComplexNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.modulus().partial_cmp(&other.modulus())
    }
}

impl Ord for ComplexNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.real.total_cmp(&other.real).then_with(|| self.imag.total_cmp(&other.imag));
    }
}

impl AsRef<f64> for ComplexNumber {
    fn as_ref(&self) -> &f64 {
        &self.real
    }
}

impl AsMut<f64> for ComplexNumber {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.real
    }
}

impl Hash for ComplexNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.real.to_bits());
        state.write_u64(self.imag.to_bits());
    }
}