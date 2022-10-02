use std::env;
mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();
    utils::iterate(args, utils::to_bin, 1);
}
