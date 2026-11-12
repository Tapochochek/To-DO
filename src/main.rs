use std::fmt::{self, Display};
use std::io::{self, Write};

#[derive(Debug,PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

struct Task{
    id: u32,
    text: String,
    priority: Priority,
    done: bool,
}
impl Task {
    fn new(id:u32, text:String, priority:Priority)->Self{
        Task { id, text, priority, done: false }
    }
}

impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mark = if self.done {"x"} else { " " };
        let id = &self.id;
        let priority = &self.priority;
        let task_text = &self.text;
        write!(f,"[{mark}] {id}. ({priority:?}) {task_text}")
        
    }
}

struct TaskList{
    tasks: Vec<Task>,
    next_id: u32,
}
impl TaskList{
    fn add(&mut self, task_text:String, priority:Priority){
        let new_task = Task::new(self.next_id, task_text, priority);
        self.tasks.push(new_task);
        self.next_id += 1;
    }
    fn list(&self){
        print_all(&self.tasks);
    }
    fn done(&mut self, id:u32){
        self.tasks.iter_mut().for_each(|task| if task.id == id{ task.done = true })
    }
    fn remove(&mut self, id:u32) -> Result<(),ParseError>{
        match self.tasks.iter().position(|t| t.id == id) {
            Some(index) => {self.tasks.remove(index); Ok(())},
            None => Err(ParseError::InvalidTaskId),
        }
    }
    fn filter(&self, priority:Priority){
        let filtered_tasks:Vec<&Task> = self.tasks.iter().filter(|task| task.priority == priority).collect();
        print_all(&filtered_tasks);
    }
    fn stats(&self){
        let done_tasks:Vec<&Task> = self.tasks.iter().filter(|task| task.done).collect();
        
        println!("All tasks: ⬇️");
        print_all(&self.tasks);

        println!("Completed tasks: ⬇️");
        print_all(&done_tasks);
    }
}
fn print_all<T:Display>(items: &[T]){
    items.iter().for_each(|item| println!("{}",item));
}

#[derive(Debug)]
enum Command{
    Add(Priority, String),
    List,
    Done(u32),
    Remove(u32),
    Filter(Priority),
    Stats,
    Quit,
}

#[derive(Debug)]
enum ParseError {
    InvalidCommand,
    InvalidTaskId,
    InvalidPriority,
}

fn parse_command(input: &str) -> Result<Command, ParseError>{
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty(){
        return  Err(ParseError::InvalidCommand);
    }
    else{
        let command = parts[0];
        match &command.to_lowercase()[..] {
            "add"    => {
                if parts.len() < 3{
                    return Err(ParseError::InvalidCommand);
                }
                else{
                    let priority = match check_priority(&parts[1].to_lowercase()[..]){
                        Ok(priority) => priority,
                        Err(_) => return Err(ParseError::InvalidPriority)
                    };
                    let task_text = parts[2..].join(" ");
                    Ok(Command::Add(priority, task_text))
                }
            }
            "list"   => {
                Ok(Command::List)
            }
            "done"   => {
                if parts.len() < 2{
                    return Err(ParseError::InvalidCommand);
                }
                match check_id(parts[1]) {
                    Ok(num) => return Ok(Command::Done(num)),
                    Err(_) => return Err(ParseError::InvalidTaskId)
                }
            }
            "remove" => {
                if parts.len() < 2{
                    return Err(ParseError::InvalidCommand);
                }
                match check_id(parts[1]) {
                    Ok(num) => return Ok(Command::Remove(num)),
                    Err(_) => return Err(ParseError::InvalidTaskId)
                }
            }
            "filter" => {
                if parts.len() < 2{
                    return Err(ParseError::InvalidCommand);
                }
                else{
                    match check_priority(&parts[1].to_lowercase()[..]){
                        Ok(priority) => Ok(Command::Filter(priority)),
                        Err(_) => Err(ParseError::InvalidPriority)
                    }
                }
            }
            "stats"  => {
                Ok(Command::Stats)
            } 
            "quit"   => {
                Ok(Command::Quit)
            }
            _        => return Err(ParseError::InvalidCommand)
        }

    }

}

fn check_id(part:&str) -> Result<u32, ParseError>{
    match part.parse(){
        Ok(num) => return Ok(num),
        Err(_) => return Err(ParseError::InvalidTaskId),
    };            
    
}

fn check_priority(part:&str) -> Result<Priority, ParseError>{
    match part{
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        _ => return Err(ParseError::InvalidPriority)
    }
}

fn main() {
    let task_vector: Vec<Task> = Vec::new();
    let mut task_list:TaskList = TaskList { tasks: task_vector, next_id: 1 };
    loop{
        print!("> ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let command = parse_command(&user_input);

        match command {
            Ok(command) => match command {
                Command::Add(priority,task_text) => task_list.add(task_text, priority),
                Command::List => task_list.list(),
                Command::Done(id) => task_list.done(id),
                Command::Remove(id) => match task_list.remove(id) {
                    Ok(del) => println!("Task deleted"),
                    Err(error) => println!("{error:?}")
                },
                Command::Filter(priority) => task_list.filter(priority),
                Command::Stats => task_list.stats(),
                Command::Quit => break
            },
            Err(error) => println!("{:?}",error) 
        }
        
    }
}
