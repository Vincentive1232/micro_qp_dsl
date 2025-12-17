# micro_qp_dsl
`micro_qp_dsl` is a domain specified layer on top of `micro_qp`, which is a light weight quadratic programming solver based on **ADMM((Alternating Direction Method of Multipliers)** and **IPM (Interior Point Method)**. 

This layer is built by mimicing the coding style of another widely used but based on standard library crate called `simple_qp` so that the users are now able to easily implement their original optimization problem on embedded platforms without learning a completely different way for defining a problem.

## 1. What can be solved?
The crate now can only solve standard convex QP problems, which are described as:

$$
\begin{aligned}
\min_{x \in \mathbb{R}^n} \quad & \tfrac{1}{2}x^\top Hx + f^\top x \\
\text{s.t.} \quad & l \le Ax \le u
\end{aligned}
$$

where:

$$
\begin{aligned}
H &\succeq 0 &&: \text{Symmetric Positive Definite/Semi Positive Definite} \\
f &\in \mathbb{R}^n &&: \text{Linear term} \\
A &\in \mathbb{R}^{m \times n} &&: \text{Constraints matrix} \\
l,u &\in \mathbb{R}^m &&: \text{Lower and upper limit（could be } \pm \infty \text{）}
\end{aligned}
$$

Further derivation is written [here](https://github.com/Vincentive1232/micro_qp/blob/main/Math_Induction.md).

## 2. How to use `micro_qp_dsl`?
### STEP 0: Add `micro_qp` to your project

**Option A: Local path (recommended for development)**

Enter your project root directory and create a folder called `libs` (Optional but recommended).
```
cd path/to/your/project
mkdir libs
```

Use the following command to recursively clone `micro_qp_dsl` and `micro_qp` into `libs`.
```
git clone --recurse-submodules git@github.com:Vincentive1232/micro_qp_dsl.git
```

In your `Cargo.toml` you should add:

```toml
[dependencies]
micro_qp_dsl = { path = "../libs/micro_qp_dsl" }
micro_qp = { path = "../libs/micro_qp_dsl/libs/micro_qp" }
```

**Please make sure that `std` features has already been disabled.**

### STEP 1: Define problem dimensions and import types
- `N` = number of decision variables.
- `M` = number of constraints.
- `P` = 2 * number of constraints. Only IPM needs this.

Use `ADMM`:
```rust
use micro_qp::admm::{AdmmSolver, AdmmSettings};
use micro_qp_dsl::{ProblemBuilder, constraint};

const N: usize = 2;
const M: usize = 4;
```

Use `IPM`:
```rust
use micro_qp::ipm::{IpmSolver, IpmSettings};
use micro_qp_dsl::{ProblemBuilder, constraint};

const N: usize = 2;
const M: usize = 4;
const P: usize = 2 * M;
```

### STEP 2: Gennerate the QP matrices (H, f, A, l, u)
Different from `micro_qp`, we don't need to manually form these matrices. Users should define their problem by creating an optimization problem, some decision variables, an objective and some constraints. Here is an example showing how to do this:

```rust
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
let objective = (w - w0).square() + lambda * (v - v0).square();

pb.minimize(objective);

// box constraints (simple_qp-like)
pb.constrain(constraint!(ur >= ur_min));
pb.constrain(constraint!(ur <= ur_max));
pb.constrain(constraint!(ul >= ul_min));
pb.constrain(constraint!(ul <= ul_max));

let (H, f, A, l, u) = pb.build();
```


### STEP 3: Create solver and prepare (factorization)
- `prepare($H, &A)` forms the system matrix $P = H + \rho A^{\top}A + \sigma I$.
- It then performs a Cholesky decomposition, which will be reused each iteration for efficiency.
```rust
// ===== solve with micro_qp =====
    let mut solver = AdmmSolver::<N, M>::new();
    // Or if IPM is used:
    // let mut solver = IpmSolver::<N, M, P>::new();

    // customize settings (optional)
    solver.settings = AdmmSettings {
        rho: 0.01,
        eps_pri: 1e-7,
        eps_dual: 1e-7,
        max_iter: 300,
        sigma: 1e-9,
        mu: 10.0,
        tau_inc: 2.0,
        tau_dec: 2.0,
        rho_min: 1e-6,
        rho_max: 1e6,
        adapt_interval: 25,
    };

    assert!(solver.prepare(&H, &A));
```

### STEP 4: Solve the problem
Call `solve(f, l, u)` with your QP data.
It returns `(x, iters)` where `x` is the solution vector, and `iters` is the number of ADMM iterations used.
```rust
let (x, iters) = solver.solve(&f, &l, &u);
```

### STEP 5: Warm start for sequential problem
When solving a sequence of similar problem (in control problem or MPC), use the previous `(x, z, y) as the starting point. This usually reduces the number of iterations drastically.
```rust
solver.warm_start(&solver.x, &solver.z, &solver.y);
let (_x2, _it2) = solver.solve(&f, &l, &u);
```

**If `H` or `A` changes, call `prepare(&H, &A)` again. If only `f,l,u` change, you can directly call `solve`.**


## 4. Troubleshooting & performance tips

- **Cholesky fails**: Increase `sigma`(regularization term), check if `H` is symmetric and PSD, or rescale problem data.

- **Slow convergence**:
    - Adjust `rho`, or use adaptive `rho` (`adapt_interval > 0`).

    - Normalize problem data to avoid large scale differences.

- Use warm starting.

- Need faster convergence: Consider over-relaxation (e.g. replace `Ax` by $\alpha Ax + (1-\alpha)z$, with $\alpha \approx 1.6$).

## 5. Limitations (What micro_qp currently can't colve)
- **Non-convex** QP (i.e., $H \not\succeq 0$) -> no global optimality guarantees.
- **Nonlinear / conic constraints** beyond 
$l \leq Ax \leq u$(unless extended with proper proximal operators).
- **Mixed-integer** constraints (binary/integer variables).
- Large-scale problems requiring high-accuracy in few iterations (**interior-point method** may be preferable).

