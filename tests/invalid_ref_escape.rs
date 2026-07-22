let r;
{
    let val = lt!(42, "lt1");
    r = lt!(&val, "lt1");
}
unsafe { println!("{}", *r); }
