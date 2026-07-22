let val = lt!(Box::new(42), "l");
let p = lt!(&*val, "l");
drop(val);
unsafe { assert!(*p == 42); }
