#[macro_export]
macro_rules! lt {
    ($e:expr, $l:expr) => { $e };
    ($e:expr) => {
        compile_error!("missing lifetime label in lt!() — use lt!(expr, \"label\")");
    };
}

include!("../analysis.rs");

pub fn check_file(path: &str) -> Vec<(usize, String)> {
    let src = std::fs::read_to_string(path).unwrap();
    check_source(&src)
}
