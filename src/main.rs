pub mod util;
pub mod things;

fn main() {
    things::example::example();
    util::cli::cli_command();
}
