let arc = lt!(std::sync::Arc::new(42), "l");
let p = lt!(std::sync::Arc::as_ptr(&arc), "l");
unsafe { assert_eq!(*p, 42); }
