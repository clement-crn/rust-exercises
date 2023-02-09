// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for _name in &names {
        println!("{:?}", names);
    }

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for _n in numbers {
        println!("{:?}", numbers);
    }
}
