let opt = lt!(Some(Box::new(42)), "l");
let r = lt!(opt.as_ref(), "l");
unsafe { assert_eq!(**r.unwrap(), 42); }
