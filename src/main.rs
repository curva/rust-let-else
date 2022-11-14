
enum MyState {
    Samumi,
    Nemumi,
    Other(String),
}

fn main() {
    usecase1();
    usecase2();
}

fn usecase1() {
    let my_state = MyState::Other("Hello".to_string());

    let MyState::Other(state_inner) = my_state else {
        println!("Your state is Samumi or Nemumi.");
        return;
    };

    println!("Your state is Other({})", state_inner);
}

fn usecase2() {
    let maybe_numbers = vec!["0", "1", "2", "fizz", "4", "buzz"];

    for maybe_number in maybe_numbers {
        let Ok(n) = maybe_number.parse::<i32>() else {
            continue;
        };
        println!("{}^2 = {}", n, n * n);
    }

    // functional call pattern
    let maybe_numbers = vec!["0", "1", "2", "fizz", "4", "buzz"];
    let pow2numbers: Vec<_> = maybe_numbers
        .into_iter()
        .filter_map(|maybe_num| maybe_num.parse::<i32>().ok())
        .map(|n| n * n)
        .collect();
    println!("result: {:?}", pow2numbers);
}
