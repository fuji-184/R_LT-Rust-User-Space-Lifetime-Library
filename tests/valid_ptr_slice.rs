let val = lt!(vec![1u8, 2, 3], "l");
let s = lt!(val.as_slice(), "l");
let b = lt!(val.as_bytes(), "l");
unsafe { assert!(s[0] == 1 && b[0] == 1); }
