// Fix errors and panics to make it work
fn main() {
    let v1 = 10 + 10;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);
}
