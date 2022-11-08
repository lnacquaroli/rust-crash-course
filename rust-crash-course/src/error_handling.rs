#![deny(clippy::all)]

// Error handling: returning values or errors
// Like tuple but with more structure

fn gets_user_name() -> Result<String, ()> {
    Ok("Buckminster".to_string())
}

fn gets_user_name_with_err() -> Result<String, ()> {
    Err(())
}

fn get_first_name() -> Result<String, ()> {
    Ok("Mileva".to_string())
}

fn get_last_name() -> Result<String, ()> {
    Ok("Maric".to_string())
}

fn get_full_name() -> Result<String, ()> {
    let first_name = get_first_name()?;
    let last_name = get_last_name()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn get_first_name_err() -> Result<String, String> {
    Ok("Mileva".to_string())
}

fn get_last_name_err() -> Result<String, String> {
    Err("I don't know your last name".to_string())
}

fn get_full_name_err() -> Result<String, String> {
    let first_name = get_first_name_err()?;
    let last_name = get_last_name_err()?;
    Ok(format!("{} {}", first_name, last_name))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example with a value with type &str or an error
    // The error box references to memory in the heap
    // std::error::Error is a trait
    // Ok is a way to return the right value instead of the error
    let value: Result<&str, Box<dyn std::error::Error>> = Ok("Hello");

    // Matching results: try except like structure
    match value {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    };

    // Void result values or errors: it can have an error but it's void
    // let value: Result<&str, ()> = Ok("Hello");
    let value: Result<&str, ()> = Err(());
    match value {
        Ok(value) => println!("{}", value),
        Err(_) => println!("Some error occured"),
    };

    // Expecting a value from Result
    let username = gets_user_name().expect("Failed to get username");
    println!("User name is: {}", username);

    //let username = gets_user_name_with_err().expect("Failed to get username");
    //println!("User name is: {}", username);

    // Expecting an error from Result (weird)
    //let username = gets_user_name().expect_err("Didn't expect a username");

    // Check OK or Err with if-statement
    let is_ok = gets_user_name().is_ok();
    let is_err = gets_user_name().is_err();
    println!("{} {}", is_ok, is_err);

    let is_ok = gets_user_name_with_err().is_ok();
    let is_err = gets_user_name_with_err().is_err();
    println!("{} {}", is_ok, is_err);

    // Early exit from Results errors
    // If any of get_first_name or get_last_name returns Err(()), then get_full_name stops early
    let fullname = get_full_name();
    match fullname {
        Ok(name) => println!("Hello {}", name),
        Err(_) => println!("Error"),
    }

    // Map Ok in Result
    // unwrap_or_default() will return 0 if any of the functions involved return an Err(())
    let full_name = get_full_name();
    let length = full_name.map(|s| s.len()).unwrap_or_default();
    println!("Length of string: {}", length);

    // Map Err in Result
    let full_name_err = get_full_name_err();
    let error_length = full_name_err.map(|e| e.len()).unwrap_err();
    println!("Length of string: {}", error_length);

    // Even the main function can return Result, by changing its signature
    // Addded -> Result<> in the main
    let full_name = get_full_name_err()?;
    Ok(())
}
