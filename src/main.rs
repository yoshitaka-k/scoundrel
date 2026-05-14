use scoundrel::cli::console::{
    print_br,
    print_double_separator
};
use scoundrel::game::{app};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_double_separator();

    let _ = app();

    print_double_separator();

    print_br();

    Ok(())
}
