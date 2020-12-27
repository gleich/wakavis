mod api;

fn main() -> anyhow::Result<()> {
    let token = api::get_token()?;
    println!("{}", token);

    Ok(())
}
