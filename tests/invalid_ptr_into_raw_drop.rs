let val = lt!(Box::new(42), "l");
let p = lt!(Box::into_raw(val), "l");
drop(val);
unsafe { assert!(!p.is_null()); }
