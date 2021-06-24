pub fn run_input(input: String) -> i32 {
    let exit_code = 0;

    if input.trim().is_empty() {
        return 0;
    }

    println!("Executing {:?}", input);

    exit_code
}