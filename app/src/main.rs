use console::Term;
use lib::fibonacci::FIB_0;
use num_format::{Locale, ToFormattedString};

fn main() {
    let reset_char = 'r';
    let exit_char = 'q';

    println!(
        "Press {} to start over and '{}' to quit; any other key to display the next fib...\n",
        reset_char,
        exit_char
    );

    let mut fib = FIB_0;

    let console = Term::stdout();

    loop {
        match console.read_char() {
            Ok(char) => {
                if char == exit_char {
                    break;
                } else {
                    if char == reset_char {
                        println!(
                            "Fib is reset to: {}",
                            fib.reset().value()
                        );
                    } else {
                        println!(
                            "Fib is: {}",
                            fib.value()
                                .to_formatted_string(
                                    &Locale::en
                                )
                        );
                    }
                    fib.calc_next();
                }
            }
            Err(e) => {
                eprintln!("ERROR reading input: {:?}", e);
                break;
            }
        }
    }
}
