#[macro_export]
macro_rules! constraint {
    ($lo:tt <= $e:tt <= $hi:tt) => {{
        (
            ($e).ge($lo),
            ($e).le($hi),
        )
    }};

    ($e:tt == $rhs:tt) => {{
        ($e).eq($rhs)
    }};

    ($e:tt <= $rhs:tt) => {{
        ($e).le($rhs)
    }};

    ($e:tt >= $rhs:tt) => {{
        ($e).ge($rhs)
    }};
}
