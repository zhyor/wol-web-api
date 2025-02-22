use std::env;
mod wol;
fn main() {
    let args: Vec<String> = env::args().collect();
    if !args.len() == 2 {
        println!("Invalid parameter");
        return ();
    }
    let mac = &args[1];
    match wol::wake_on_lan(mac) {
        Some(()) => println!("Success: {}", mac),
        None => println!("Mac Format Error: {}", mac),
    }
}
