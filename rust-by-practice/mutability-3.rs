fn main() {
    // Fix error by modifying this line
    let s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}
