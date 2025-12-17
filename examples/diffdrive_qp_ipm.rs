#![allow(non_snake_case)]

use std::time::Instant;

use micro_qp::ipm::{IpmSolver};
use micro_qp_dsl::{ProblemBuilder, constraint};

const N: usize = 2;
const M: usize = 4;
const P: usize = 8; // 2*M, need this for some convenience

fn main() {
    let r = 0.05;
    let L = 0.20;
    let lambda = 1.0;
    let v0 = 0.6;
    let w0 = 1.2;

    let ur_min = -20.0;
    let ur_max =  20.0;
    let ul_min = -20.0;
    let ul_max =  20.0;

    // ===== construct QP via DSL (simple_qp-like) =====
    let mut pb = ProblemBuilder::<N, M>::new();

    let ur = pb.var();
    let ul = pb.var();

    let w = (r / L) * (ur - ul);
    let v = (r / 2.0) * (ur + ul);

    // objective
    let objective =
        (w - w0).square()
      + lambda * (v - v0).square();

    pb.minimize(objective);

    // box constraints (simple_qp-like)
    pb.constrain(constraint!(ur >= ur_min));
    pb.constrain(constraint!(ur <= ur_max));
    pb.constrain(constraint!(ul >= ul_min));
    pb.constrain(constraint!(ul <= ul_max));

    let (H, f, A, l, u) = pb.build();

    // ===== solve with micro_qp =====
    // ===== construct and configure the solver =====
    let mut solver = IpmSolver::<N,M,P>::new();

    assert!(solver.prepare(&H, &A));

    let t0 = Instant::now();
    let (x, iters) = solver.solve(&f, &l, &u);
    let elapsed = t0.elapsed();

    println!(
        "Solved in {iters} iterations, time = {:.3} ms",
        elapsed.as_secs_f64() * 1e3
    );


    let ur_val = x.data[0];
    let ul_val = x.data[1];

    println!("Solved in {iters} iterations.");
    println!("ur = {ur_val:.6}, ul = {ul_val:.6}");

    let v_res = 0.5 * r * (ur_val + ul_val);
    let w_res = (r / L) * (ur_val - ul_val);
    println!("Resulting v = {v_res:.6} m/s, w = {w_res:.6} rad/s");
}
