
use std::io;

fn main() {
    let mut s: i32 = 0;
    let mut is_ok: bool = true;
    let mut next: bool = true;

    while next {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось считать строку");

        let mut i: i8 = 0;
        for token in input.split_whitespace() {
            i += 1;
            if i == 2 {
                is_ok = false;
                break;
            }
            match token.parse::<i32>() {
                Ok(num) => {
                    if num == -1 {
                        next = false;
                    } else {
                        s += num;
                    }
                }
                Err(_) => {
                    is_ok = false;
                    break;
                }
            }
        }
    }

    if is_ok {
        println!("{}", s);
    } else {
        println!("NaN");
    }
}
    
