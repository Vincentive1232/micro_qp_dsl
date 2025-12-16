use core::ops::{Add, Sub};
use crate::LinExpr;

#[derive(Copy, Clone)]
pub struct Var<const N: usize> {
    pub(crate) idx: usize,
}

impl<const N: usize> Sub<Var<N>> for Var<N> {
    type Output = LinExpr<N>;
    fn sub(self, rhs: Var<N>) -> LinExpr<N> {
        LinExpr::from(self) - LinExpr::from(rhs)
    }
}

impl<const N: usize> Add<Var<N>> for Var<N> {
    type Output = LinExpr<N>;
    fn add(self, rhs: Var<N>) -> LinExpr<N> {
        LinExpr::from(self) + LinExpr::from(rhs)
    }
}