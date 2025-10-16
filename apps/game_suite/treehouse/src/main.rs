use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    println!("Enter your name: ");

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name.trim().to_lowercase()
}

fn main() {
    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;

    let name = what_is_your_name();

    for i in 0..visitor_list.len() {
        if visitor_list[i] == name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome.");
    } else {
        println!("Sorry, you are not on the list.")
    }
}
