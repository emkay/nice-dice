use std::error::Error;
use nice_dice::parser;

#[paw::main]
fn main(args: paw::Args) -> Result<(), Box<dyn Error>> {
    let mut args = args.skip(1);
    let s = args
        .next()
        .unwrap()
        .to_owned();

    let roll = parser::parse(&s[..]);
    println!("outcome: {}", roll.outcome);
    Ok(())
}
