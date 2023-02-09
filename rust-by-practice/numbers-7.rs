// Fill the blank to make it work
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), type_of(&_z).to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
