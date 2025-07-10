pub fn start(){

    let age_to_check = 18;

    println!("Enter your age:");
    let mut input = String::new();

    while true {
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let res: Result<i32, _> = input.trim().parse();

        match res {
            Ok(age) => {
                input.clear(); // Clear the input for the next read
                check_age(age, age_to_check);
                break; // Exit the loop after a valid input
            },
            Err(_) => {
                println!("Invalid input. Please enter a valid age:");
                input.clear(); // Clear the input for the next read
            }
        }

    }
}

pub fn check_age(age: i32, age_to_check: i32) {
    if age >= age_to_check {
        println!("You are old enough to vote.");
    } else {
        println!("You are not old enough to vote.");
    }
}