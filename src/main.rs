use cal_rs::*;

fn main() -> eyre::Result<()> {
    let x = Date::now();
    println!("{x:?}");
    Ok(())
}

