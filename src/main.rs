mod todo;
mod storage;

use clap::{Error, Parser};

fn main() -> Result<(), Error>{
    // Los args 
    //let args = Args::parse();
    let mut storage = storage::Storage::new()?;
    storage::Storage::print_tasks();
    Ok(())
}

/// Simple TODO-LIST program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Imprimir las tareas por completar
    #[arg(short, long)]
    ls: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
