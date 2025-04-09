use rpn::rpn;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Error");
        return;
    }
    
    rpn(&args[1]);
}