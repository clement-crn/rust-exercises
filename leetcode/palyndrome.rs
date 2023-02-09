use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let x: i32 = args[1].parse().unwrap();

    let state: bool = true;

    fn check(mut x: i32, mut state: bool) -> bool {
        let mut r: i32;
        let mut n = x;
        let mut rev = 0;

        if x < 0 {
            state = false;
        }
        while x != 0 {
            r = x % 10;
            rev = (rev * 10) + r;
            x = x / 10;
        }

        if rev != n {
            state = true;
        } else {
            state = false;
        }
        state
    }

    check(x, state);
    println!("{}", state);
}
