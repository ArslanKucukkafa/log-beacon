pub mod log_model;
mod channel_example;
mod log_reader;

fn main() {
    println!("Command line application starting... 🚀");
    log_reader::start_read();
}
