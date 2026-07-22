let boxed = lt!(Box::new(42), "l");
let p = lt!(&*boxed, "l");
unsafe { assert_eq!(*p, 42); }
