use std::ops::{Add, Div, Mul};

#[derive(Debug)]
pub struct Quaternion {
    qr: f64,
    qi: f64,
    qj: f64,
    qk: f64,
}

impl Quaternion {
    fn new(qr: f64, qi: f64, qj: f64, qk: f64) -> Self {
        Self { qr, qi, qj, qk }
    }

    fn real(&self) -> f64 {
        self.qr
    }

    fn imaginary(&self) -> (f64, f64, f64) {
        (self.qi, self.qj, self.qk)
    }
    fn square_norm(&self) -> f64 {
        self.qr * self.qr + self.qi * self.qi + self.qj * self.qj + self.qk * self.qk
    }

    fn conjugate(&self) -> Self {
        Quaternion::new(self.qr, -self.qi, -self.qj, -self.qk)
    }

    fn inverse(&self) -> Self {
        // FIXME: pfffffffff
        let q_conjugate = self.conjugate();
        let inverse_scalar = 1.0 / self.square_norm();

        Quaternion::new(
            q_conjugate.qr * inverse_scalar,
            q_conjugate.qi * inverse_scalar,
            q_conjugate.qj * inverse_scalar,
            q_conjugate.qk * inverse_scalar,
        )
    }
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
    fn new_quaternion() {
        let q = Quaternion::new(1.0, 1.1, 0.33, 0.1);
        let qk = q.qk;
        let qr = q.qr;
        let qi = q.qi;
        let qj = q.qj;
        assert_eq!(qk, 0.1);
        assert_eq!(qi, 1.1);
        assert_eq!(qr, 1.0);
        assert_eq!(qj, 0.33);
    }
    #[test]
    fn real() {
        let q = Quaternion::new(0.23, 0.0, 0.0, 0.0);
        assert_eq!(q.real(), 0.23);
    }
    #[test]
    fn imaginary() {
        let q = Quaternion::new(0.23, 0.2, 0.1, 0.5);
        let i = (0.2, 0.1, 0.5);
        assert_eq!(q.imaginary(), i);
    }
    #[test]
    fn add() {
        let q1 = Quaternion::new(0.1, 0.2, 0.3, 0.4);
        let q2 = Quaternion::new(0.0, 0.3, 0.1, 0.2);
        assert_eq!(q1 + q2, Quaternion::new(0.1, 0.5, 0.4, 0.6000000000000001));
    }
    #[test]
    fn conjugate() {
        let q = Quaternion::new(0.1, 0.2, 0.3, 0.3);
        let q_conjugate = q.conjugate();
        let q_expected = Quaternion::new(0.1, -0.2, -0.3, -0.3);
        assert_eq!(q_conjugate, q_expected);
    }
    #[test]
    fn inverse() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q_inverse = q.inverse();
        let q_expected = Quaternion::new(0.03, -0.07, -0.1, -0.13);
        assert_eq!(q_inverse.round(2), q_expected);
    }
}
