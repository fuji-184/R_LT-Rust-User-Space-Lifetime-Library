let val = lt!(String::from("hello"), "l");
let p = lt!(val.as_ptr(), "l");
let bytes = lt!(val.as_bytes(), "l");
unsafe { assert!(*p == 104 && bytes[0] == 104); }
