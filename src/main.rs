mod todo;
mod storage;

use clap::{Error, Parser};

fn main() -> Result<(), Error>{
    // Los args 
    let args = Args::parse();
    let mut storage = storage::Storage::new()?;

    match args.ls.as_deref(){
        Some("all") => {storage.print_tasks()?;}
        Some("pending") => {storage.print_pending_tasks()?;}
        _ => {}
    }

    if !args.add.is_empty() {
        storage.add(&args.add)?;
        println!("Tarea {} añadida", storage::Storage::get_last_id()?);
    }

    if args.mark != 0 { 
        storage.mark_complete(args.mark)?;
        println!("Tarea {} marcada como completada.", args.mark);
    }

    if args.remove !=0{
        storage.remove_by_id(args.remove as usize)?;
        println!("Tarea {} ELIMINADA.", args.remove);
    }
    Ok(())
}

/// Simple TODO-LIST program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
 struct Args {
    /// Imprimir las tareas por completar (all, pending)
    #[arg(short='l', long, num_args= 0..=1, default_missing_value = "all")]
    ls: Option<String>,

    /// Añadir nueva tarea con "" para que no haya problemas
    #[arg(short, long, default_value_t=String::from(""))]
    add: String,

    /// Marcar la tarea como completada con id
    #[arg(short, long, default_value_t = 0)]
    mark: i32,

    /// Eliminar la tarea con id
    #[arg(short, long, default_value_t = 0)]
    remove: i32,
}
