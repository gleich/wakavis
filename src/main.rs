mod api;

// main can also return a Result! This basically lets your contextual Errors
// from api::get_token be "passed up" in a tree of errors.
fn main() -> anyhow::Result<()> {
    // You don't need context here since you are already providing it when
    // erroring out in api::get_token
    let token = api::get_token()?;
    println!("{}", token);

    // Since the function returns a Result<()> (() essentially being an empty
    // type), you must return an Ok(()) at the end of the function to conclude
    // it
    Ok(())
}
