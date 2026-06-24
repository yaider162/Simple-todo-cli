use clap::Error;

use crate::todo::Task;
use std::fs::{self, File};
use std::io::{BufWriter};
use std::path::Path;
use colored::Colorize;

pub struct Storage {
    id: i32,
}

impl Storage {
    pub fn new() -> Result<Self, Error>{
        let last_id = Self::get_last_id()?;
        Ok(Storage {id: last_id+1})
    }

    pub fn add(&mut self, title: &String) -> Result<(), std::io::Error>{
        if title.is_empty() {return Ok(());}
        let mut tasks = Self::load_data()?;
        let task = Task::new(self.id, title.clone(), false);
        tasks.push(task);

        let file = File::create("tasks.json")?;
        let writer = BufWriter::new(file);
        let _ = serde_json::to_writer_pretty(writer, &tasks);

        self.id+=1;
        Ok(())
    }

    pub fn save(&self, tasks: Vec<Task>) -> Result<(), std::io::Error>{
        let file = File::create("tasks.json")?;
        let writer = BufWriter::new(file);
        let _ = serde_json::to_writer_pretty(writer, &tasks);
        Ok(())
    }

    pub fn load_data() -> Result<Vec<Task>, std::io::Error>{
        if !Path::new("tasks.json").exists(){return Ok(Vec::new())}

        let file = fs::read_to_string("tasks.json")?;
        if file.is_empty() {return Ok(Vec::new())}

        let tasks: Vec<Task> = serde_json::from_str(&file)?;

        Ok(tasks)
    }

    pub fn print_tasks(&self) -> Result<(), std::io::Error>{
        let mut tasks = Self::load_data()?;

        if tasks.is_empty() {
            println!("{}", "No tasks yet. Add one with your command.".yellow().bold());
            return Ok(());
        }

        tasks.sort_by_key(|task| task.id);

        println!("{}", "TODO LIST".bright_blue().bold());
        println!("{}", "---------".bright_blue());

        for task in tasks {
            let status = if task.done {
                "[x]".green().bold()
            } else {
                "[ ]".yellow().bold()
            };

            let id = format!("#{}", task.id).cyan().bold();
            let title = if task.done {
                task.title.dimmed()
            } else {
                task.title.white()
            };

            println!("{} {} {}", id, status, title);
        }

        Ok(())
    }

    pub fn mark_complete(&self, id:i32) -> Result<(), Error>{
        let mut tasks = Self::load_data()?;
        for task in &mut tasks {
            if task.id==id {task.done=true;}
        }
        let _ = self.save(tasks);
        Ok(())
    }
    fn get_last_id() -> Result<i32, Error>{
        let tasks = Self::load_data()?;
        Ok(
            tasks.iter().map(|t| t.id).max().unwrap_or(0)
        )
    }
}