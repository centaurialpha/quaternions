use std::fmt::Display;
use std::ops::{Add, Div, Mul};

pub static DEFAULT_TOLERANCE: f64 = 1e-8;

#[derive(Debug)]
pub struct Quaternion {
    qr: f64,
    qi: f64,
    qj: f64,
    qk: f64,
}

impl Quaternion {
    pub fn new(qr: f64, qi: f64, qj: f64, qk: f64) -> Self {
        Self { qr, qi, qj, qk }
    }

    pub fn real(&self) -> f64 {
        self.qr
    }

    pub fn imaginary(&self) -> (f64, f64, f64) {
        (self.qi, self.qj, self.qk)
    }
    pub fn square_norm(&self) -> f64 {
        self.qr * self.qr + self.qi * self.qi + self.qj * self.qj + self.qk * self.qk
    }

    pub fn conjugate(&self) -> Self {
        Quaternion::new(self.qr, -self.qi, -self.qj, -self.qk)
    }

    pub fn inverse(&self) -> Self {
        let q_conjugate = self.conjugate();
        let inverse_scalar = 1.0 / self.square_norm();

        Quaternion::new(
            q_conjugate.qr * inverse_scalar,
            q_conjugate.qi * inverse_scalar,
            q_conjugate.qj * inverse_scalar,
            q_conjugate.qk * inverse_scalar,
        )
    }
    pub fn norm(&self) -> f64 {
        self.square_norm().sqrt()
    }

    pub fn normalized(&self) -> Quaternion {
        let norm = self.norm();
        Quaternion {
            qr: self.qr / norm,
            qi: self.qi / norm,
            qj: self.qj / norm,
            qk: self.qk / norm,
        }
    }

    #[allow(dead_code)]
    fn round(mut self, decimals: usize) -> Self {
        let factor = 10_f64.powi(decimals as i32);
        self.qr = (self.qr * factor).round() / factor;
        self.qi = (self.qi * factor).round() / factor;
        self.qj = (self.qj * factor).round() / factor;
        self.qk = (self.qk * factor).round() / factor;
        self
    }
}

impl Add for Quaternion {
    type Output = Quaternion;
    fn add(self, other: Self) -> Self::Output {
        Quaternion::new(
            self.qr + other.qr,
            self.qi + other.qi,
            self.qj + other.qj,
            self.qk + other.qk,
        )
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{:+}i{:+}j{:+}k", self.qr, self.qi, self.qj, self.qk)
    }
}
impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        let qr = self.qr * other.qr - self.qi * other.qi - self.qj * other.qj - self.qk * other.qk;
        let qi = self.qr * other.qi + self.qi * other.qr + self.qj * other.qk - self.qk * other.qj;
        let qj = self.qr * other.qj - self.qi * other.qk + self.qj * other.qr + self.qk * other.qi;
        let qk = self.qr * other.qk + self.qi * other.qj - self.qj * other.qi + self.qk * other.qr;
        Quaternion::new(qr, qi, qj, qk)
    }
}

impl Mul<f64> for Quaternion {
    type Output = Quaternion;
    fn mul(self, p: f64) -> Self::Output {
        Quaternion::new(self.qr * p, self.qi * p, self.qj * p, self.qk * p)
    }
}
impl Div<Quaternion> for f64 {
    type Output = Quaternion;
    fn div(self, other: Quaternion) -> Self::Output {
        Quaternion::new(
            self / other.qr,
            -self / other.qi,
            -self / other.qj,
            -self / other.qk,
        )
    }
}
impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.qr == other.qr && self.qi == other.qi && self.qj == other.qj && self.qk == other.qk
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let expected = Quaternion {
            qr: 1.0,
            qi: 2.0,
            qj: 3.0,
            qk: 4.0,
        };
        let result = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(expected, result);
    }
    #[test]
    fn test_real() {
        let q = Quaternion::new(0.23, 0.0, 0.0, 0.0);
        assert_eq!(q.real(), 0.23);
    }
    #[test]
    fn test_imaginary() {
        let q = Quaternion::new(0.23, 0.2, 0.1, 0.5);
        let i = (0.2, 0.1, 0.5);
        assert_eq!(q.imaginary(), i);
    }
    #[test]
    fn test_add() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(0.5, 0.5, 0.5, 0.5);
        let expected = Quaternion::new(1.5, 2.5, 3.5, 4.5);
        let result = q1 + q2;
        assert_eq!(expected, result);
    }
    #[test]
    fn test_conjugate() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let expected = Quaternion::new(1.0, -2.0, -3.0, -4.0);
        let result = q.conjugate();
        assert_eq!(expected, result);
    }
    #[test]
    fn test_inverse() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let expected = Quaternion::new(
            0.03333333333333333,
            -0.06666666666666667,
            -0.1,
            -0.13333333333333333,
        );
        let result = q.inverse();
        assert_eq!(expected, result);
    }
    #[test]
    fn test_norm() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let expected = (30.0 as f64).sqrt();
        let result = q.norm();
        assert_eq!(expected, result);
    }
    #[test]
    fn test_normalized() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let normalized_q = q.normalized();
        let expected_norm = 1.0;
        let actual_norm = normalized_q.norm();
        assert!(
            (expected_norm - actual_norm).abs() < DEFAULT_TOLERANCE,
            "The norm of the normalized quaternion is {} but {} was expected.",
            actual_norm,
            expected_norm
        );
    }
}
