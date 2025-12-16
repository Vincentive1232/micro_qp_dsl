#![allow(non_snake_case)]

use core::ops::{Add, Mul};
use crate::LinExpr;

#[derive(Copy, Clone)]
pub struct QuadExpr<const N: usize> {
    pub H: [[f32; N]; N],
    pub f: [f32; N],
}

impl<const N: usize> QuadExpr<N> {
    pub fn zero() -> Self {
        Self { H: [[0.0; N]; N], f: [0.0; N] }
    }
}

// (a^Tx + c)^2
impl<const N:usize> LinExpr<N> {
    pub fn square(self) -> QuadExpr<N> {
        let mut q = QuadExpr::zero();

        for i in 0..N {
            for j in 0..N {
                q.H[i][j] += 2.0 * self.coeffs[i] * self.coeffs[j];
            }
            q.f[i] += 2.0 * self.constant * self.coeffs[i];
        }
        q
    }
}

// QuadExpr + QuadExpr
impl<const N: usize> Add for QuadExpr<N> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        for i in 0..N {
            for j in 0..N { self.H[i][j] += rhs.H[i][j]; }
            self.f[i] += rhs.f[i];
        }
        self
    }
}

// QuadExpr * scalar
impl<const N: usize> Mul<f32> for QuadExpr<N> {
    type Output = Self;
    fn mul(mut self, k: f32) -> Self {
        for i in 0..N {
            for j in 0..N { self.H[i][j] *= k; }
            self.f[i] *= k;
        }
        self
    }
}

//  scalar * QuadExpr
impl<const N: usize> Mul<QuadExpr<N>> for f32 {
    type Output = QuadExpr<N>;
    fn mul(self, mut q: QuadExpr<N>) -> QuadExpr<N> {
        for i in 0..N {
            for j in 0..N {
                q.H[i][j] *= self;
            }
            q.f[i] *= self;
        }
        q
    }
}