fn main() -> anyhow::Result<()>{
    let arg = std::env::args().nth(1).unwrap_or("NaN".to_string());
    let val = arg.parse::<i32>()?;

    println!("{} ^ 2 = {}", val, val * val);

    Ok(())
}

fn main2() {
    let arg = std::env::args().nth(1).unwrap_or("NaN".to_string());

    let val = if let Ok(n) = arg.parse::<i32>() {
        n
    } else {
        println!("caution!: {} is not a number, please pass a number.", arg);
        return;
    };

    println!("{} ^ 2 = {}", val, val * val);
}

