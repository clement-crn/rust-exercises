// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
        //permet de return x
    };

    assert_eq!(v, 3);

    println!("Success!");
}

// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
    //v ne return rien
    assert_eq!(v, ());

    println!("Success!");
}
