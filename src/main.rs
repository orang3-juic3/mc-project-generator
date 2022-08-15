use clap::Parser;

#[derive(Parser)]
struct Cli {

}
fn main() {
    println!("{}{:?}",LONG_STRING ,dirs::home_dir().unwrap().as_path());

}
