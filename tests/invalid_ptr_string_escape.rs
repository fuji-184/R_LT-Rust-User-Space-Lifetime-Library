let p;
{
    let val = lt!(String::from("hello"), "l");
    p = lt!(val.as_bytes(), "l");
}
unsafe { assert_eq!(p[0], 104); }
