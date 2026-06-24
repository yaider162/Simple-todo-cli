mod todo;
mod storage;

use clap::{Error, Parser};

fn main() -> Result<(), Error>{
    // Los args 
    let args = Args::parse();
    let mut storage = storage::Storage::new()?;
    if args.ls {storage.print_tasks()?;}

    if args.count > 0 && !args.add.is_empty() {
        for _ in 0..args.count {
            storage.add(&args.add)?;
            println!("Tarea {} añadida", args.mark);
        }
    }

    if args.mark != 0 { 
        storage.mark_complete(args.mark)?;
        println!("Tarea {} marcada como completada.", args.mark);
    }
    Ok(())
}

/// Simple TODO-LIST program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
 struct Args {
    /// Imprimir las tareas por completar
    #[arg(short='l', long,)]
    ls: bool,

    /// Añadir nueva tarea con "" para que no haya problemas
    #[arg(short, long, default_value_t=String::from(""))]
    add: String,

    /// Marcar la tarea como completada con id
    #[arg(short, long, default_value_t = 0)]
    mark: i32,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 0)]
    count: i32,
}
