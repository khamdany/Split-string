use std::error::Error;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::process;


#[derive(Clone)]
pub struct Data {
    number: i32,
    file_path: String,
}

impl Data {
    fn open(&self) -> Result<(i32, String), Box<dyn Error>> {
        let number = self.number;
        let content = fs::read_to_string(&self.file_path)?;
        let join = (number, content);
        Ok(join)
    }

    fn split_at_newline(&self) -> (String, String) {
        if let Err(e) = self.open() {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
        let (number, content) = self.open().unwrap();
        let per_line: Vec<&str> = content.lines().collect();
        let mut used_string = Vec::new();
        let mut left_string = String::new();
        for (i, value) in per_line.clone().into_iter().enumerate() {
            if i as i32 == number {
                break;
            }
            used_string.push(value);
        }
        let used_string = used_string.join("\n");
        if per_line.len() > number as usize {
            left_string = per_line[number as usize..].join("\n")
        }
        let joined = (used_string, left_string);
        joined
    }
}

pub fn arg(args: Vec<String>) -> Result<(Data, String), &'static str> {
    if args.len() < 3 {
       return Err("not enough arguments");
    } else if  args.len() > 3 {
        return Err("too much arguments");
    } else if !args[1].parse::<i32>().is_ok() {
        return Err("first argument must integer");
    }

    let file = Data {
        number: args[1].parse().unwrap(),
        file_path: args[2].clone(),
    };
    let binding = file.file_path.clone();
    let file_name = Path::new(&binding)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap();
    Ok((file, file_name.to_string()))
}

pub fn save(args: (Data, String)) -> std::io::Result<()>{
    let (file, name_file) = args;
    let split = file.split_at_newline();
    let (first, second) = split;

    let first = first.as_bytes();
    let second = second.as_bytes();
    let name_file = {
        let last_dot = name_file.rfind(".");
        match last_dot {
            Some(index) => &name_file[..index],
            None => &name_file,
        }
    };
    let new_name = format!("{}1.txt", name_file);
    let mut f = File::create(new_name)?;
    f.write_all(&first)?;
    let new_name1 = format!("{}2.txt", name_file);
    let mut c = File::create(new_name1)?;
    c.write_all(&second)?;
    Ok(())
}

