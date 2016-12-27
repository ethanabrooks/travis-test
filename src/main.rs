extern {
    fn test();
}
fn main() {
    //println!("GAAAAAH");
    unsafe { test() };
}
