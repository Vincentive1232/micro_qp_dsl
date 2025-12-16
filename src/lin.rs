use core::ops::{Add, Sub, Mul, Div};
use crate::{Var};

#[derive(Copy, Clone)]
pub struct LinExpr<const N: usize> {
    pub coeffs: [f32; N],
    pub constant: f32,
}

impl<const N: usize> LinExpr<N> {
    pub fn zero() -> Self {
        Self { coeffs: [0.0; N], constant: 0.0 }
    }
}

// Var -> LinExpr
impl<const N: usize> From<Var<N>> for LinExpr<N> {
    fn from(v: Var<N>) -> Self {
        let mut e = Self::zero();
        e.coeffs[v.idx] = 1.0;
        e
    }
}

// LinExpr + LinExpr
impl<const N: usize> Add for LinExpr<N> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        for i in 0..N { self.coeffs[i] += rhs.coeffs[i];}
        self.constant += rhs.constant;
        self
    }
}

// LinExpr - LinExpr
impl<const N: usize> Sub for LinExpr<N> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..N { self.coeffs[i] -= rhs.coeffs[i];}
        self.constant -= rhs.constant;
        self
    }
}

// LinExpr * scalar
impl<const N: usize> Mul<f32> for LinExpr<N> {
    type Output = Self;
    fn mul(mut self, k: f32) -> Self {
        for i in 0..N { self.coeffs[i] *= k; }
        self.constant *= k;
        self
    }
}

// scalar * LinExpr
impl<const N: usize> Mul<LinExpr<N>> for f32 {
    type Output = LinExpr<N>;

    fn mul(self, mut e: LinExpr<N>) -> LinExpr<N> {
        for i in 0..N {
            e.coeffs[i] *= self;
        }
        e.constant *= self;
        e
    }
}


// scalar * Var
impl<const N: usize> Mul<Var<N>> for f32 {
    type Output = LinExpr<N>;
    fn mul(self, v: Var<N>) -> LinExpr<N> {
        LinExpr::from(v) * self
    }
}

// Var / scalar
impl<const N: usize> Div<f32> for Var<N> {
    type Output = LinExpr<N>;
    fn div(self, k: f32) -> LinExpr<N> {
        LinExpr::from(self) * (1.0 / k)
    }
}

// LinExpr - constant
impl<const N: usize> Sub<f32> for LinExpr<N> {
    type Output = Self;
    fn sub(mut self, c: f32) -> Self {
        self.constant -= c;
        self
    }
}