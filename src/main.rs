extern {
    fn test();
}
fn main() {
    unsafe { test() };
}
