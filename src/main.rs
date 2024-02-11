mod notes;
mod play;
mod raw_ptr;
mod scale;

fn main() {
    match play::play() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}
