fn double(v: &i32) -> i32 { *v * 2 }
fn triple(v: &i32) -> i32 { *v * 3 }
fn neg(v: &i32) -> i32 { -*v }
let val = lt!(42, "l");
let a = lt!(double(&val), "l");
let b = lt!(triple(&val), "l");
let c = lt!(neg(&val), "l");
