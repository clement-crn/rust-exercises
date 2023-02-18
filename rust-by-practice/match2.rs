fn main() {
    let boolean = true;

    //Difficile purée
    //
    //
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    assert_eq!(binary, 1);
    println!("Success!");
}
