let val = lt!(42, "lt1");
let r = lt!(&val, "lt1");
unsafe { println!("{}", *r); }
