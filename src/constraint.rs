use core::f32;

use crate::{Var, LinExpr};

#[derive(Copy, Clone)]
pub struct LinearConstraint<const N: usize> {
    pub a: [f32; N],
    pub l: f32,
    pub u: f32,
    pub c: f32,
}

// Var constraint (box / eq)
impl<const N: usize> Var<N> {
    pub fn ge(self, v: f32) -> LinearConstraint<N> {
        let mut a = [0.0; N];
        a[self.idx] = 1.0;
        LinearConstraint { a, l: v, u: f32::INFINITY, c: 0.0 }
    }

    pub fn le(self, v: f32) -> LinearConstraint<N> {
        let mut a = [0.0; N];
        a[self.idx] = 1.0;
        LinearConstraint { a, l: f32::NEG_INFINITY, u: v, c: 0.0 }
    }

    pub fn eq(self, v: f32) -> LinearConstraint<N> {
        let mut a = [0.0; N];
        a[self.idx] = 1.0;
        LinearConstraint { a, l: v, u: v, c: 0.0 }
    }
}

// LinExpr constraint
impl<const N: usize> LinExpr<N> {
    pub fn le(self, rhs: f32) -> LinearConstraint<N> {
        LinearConstraint { a: self.coeffs, l: f32::NEG_INFINITY, u: rhs, c: self.constant }
    }
    pub fn ge(self, rhs: f32) -> LinearConstraint<N> {
        LinearConstraint { a: self.coeffs, l: rhs, u: f32::INFINITY, c: self.constant }
    }
    pub fn eq(self, rhs: f32) -> LinearConstraint<N> {
        LinearConstraint { a: self.coeffs, l: rhs, u: rhs, c: self.constant }
    }
}