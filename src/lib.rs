use std::ops::Add;

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
}
