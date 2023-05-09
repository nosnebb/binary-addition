use clap::Parser;

mod models;

fn main() {
    println!("hello, ben!");
    models::args::Args::parse();
}
