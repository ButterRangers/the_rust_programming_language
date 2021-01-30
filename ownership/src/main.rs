fn main() {
    let s = String::from("hello");

    let s = takes_ownership(s);

    println!("we gotta take the power back {}", s); 

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);    
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
