fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    let z = &mut x; // this will cause error
    *z += 1;
}