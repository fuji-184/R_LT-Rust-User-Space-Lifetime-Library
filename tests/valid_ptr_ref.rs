let val = lt!(42, "l");
let r = lt!(&val, "l");
unsafe { assert_eq!(*r, 42); }
