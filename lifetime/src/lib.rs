#[macro_export]
macro_rules! lt {
    ($e:expr, $l:expr) => { $e };
    ($e:expr) => {
        compile_error!("missing lifetime label in lt!() — use lt!(expr, \"label\")");
    };
}
