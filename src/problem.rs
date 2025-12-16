#![allow(non_snake_case)]

use micro_qp::types::{MatMN, VecN};
use crate::{Var, QuadExpr, LinearConstraint};

pub struct ProblemBuilder<const N: usize, const M: usize> {
    H: [[f32; N]; N],
    f: [f32; N],
    A: [[f32; N]; M],
    l: [f32; M],
    u: [f32; M],
    var_cnt: usize,
    constr_cnt: usize,
}

impl<const N: usize, const M: usize> ProblemBuilder<N, M> {
    pub fn new() -> Self {
        Self {
            H: [[0.0; N]; N],
            f: [0.0; N],
            A: [[0.0; N]; M],
            l: [0.0; M],
            u: [0.0; M],
            var_cnt: 0,
            constr_cnt: 0,
        }
    }

    pub fn var(&mut self) -> Var<N> {
        let v = Var {idx: self.var_cnt };
        self.var_cnt += 1;
        v
    }

    pub fn minimize(&mut self, q: QuadExpr<N>) {
        for i in 0..N {
            for j in 0..N {
                self.H[i][j] += q.H[i][j];
            }
            self.f[i] += q.f[i];
        }
    }

    pub fn constrain(&mut self, c: LinearConstraint<N>) {
        let r = self.constr_cnt;
        self.A[r] = c.a;
        self.l[r] = c.l - c.c;
        self.u[r] = c.u - c.c;
        self.constr_cnt += 1;
    }

    pub fn build(self) -> (MatMN<N, N>, VecN<N>, MatMN<M, N>, VecN<M>, VecN<M>) {
        let mut Hm = MatMN::zero();
        let mut fv = VecN::zero();
        let mut Am = MatMN::zero();
        let mut lv = VecN::zero();
        let mut uv = VecN::zero();

        for i in 0..N {
            fv.data[i] = self.f[i];
            for j in 0..N {
                Hm.set(i, j, self.H[i][j]);
            }
        }
        for r in 0..M {
            lv.data[r] = self.l[r];
            uv.data[r] = self.u[r];
            for c in 0..N {
                Am.set(r, c, self.A[r][c]);
            }
        }
        (Hm, fv, Am, lv, uv)
    }
}