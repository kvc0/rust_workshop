use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// This is the input.
    input: String,

    /// Shout out the input.
    #[arg(long)]
    loud: bool,
}

fn main() {
    // This is a convenient way to set up the normal Rust ecosystem logging.
    env_logger::init();

    // Parse your arguments.
    let args = Args::parse();

    // Handle your argument - numbers versus strings.
    let input = args.input;
    let result = input.parse::<i64>();

    match result {
        Ok(value) => {
            println!("{}", value * 2);
        }
        Err(_) => {
            let input = if args.loud {
                input.to_uppercase()
            } else {
                input
            };

            println!("You said \"{}\"", input);
        }
    }

    // Print the result.
}
