use std::fs;
use std::error::Error;
use std::env;

pub struct Configuration {
    pub query: String,
    pub filename: String,
	pub is_case_sensitive: bool,
}
impl Configuration {
    fn check_argument_count(count: usize) -> Result<(), String> {
        const ARGUMENT_COUNT: usize = 2;

        match count {
            0 => Err(format!("No arguments provided. Need {} arguments.", ARGUMENT_COUNT)),
            1 => Err(format!("Only one argument provided. Need {} arguments.", ARGUMENT_COUNT)),
            n if n < ARGUMENT_COUNT => Err(format!("Only {} arguments were provided, but {} are needed.", count, ARGUMENT_COUNT)),
            ARGUMENT_COUNT => Ok(()),
            n if n > ARGUMENT_COUNT => {
                println!("Note: only {1} arguments are needed, but {0} were provided.", count, ARGUMENT_COUNT);
                println!("Will use the first two arguments.\n");
                Ok(())
            },
            _ => Err("This shall not happen.".to_string())
        }
    }

    pub fn new() -> Result<Configuration, String> {
        // We do not need the first argument.
        let arguments: Vec<String> = env::args().collect();
        let arguments = &arguments[1..];

        match Configuration::check_argument_count(arguments.len()) {
            Ok(()) => Ok(Configuration{
                query: String::from(&arguments[0]), 
                filename: String::from(&arguments[1]),
				is_case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            }),
            Err(message) => Err(message),
        }
    }
}

fn search<'a>(query: &str, contents: &'a str, is_case_sensitive: bool) -> Vec<(usize, &'a str)> {
	if is_case_sensitive {
		contents.lines().enumerate().filter(|(_, line)| line.contains(query)).collect()
	} else {
		let query = query.to_lowercase();
		contents.lines().enumerate().filter(|(_, line)| line.to_lowercase().contains(&query)).collect()
	}
}

fn print_matching_file_lines(configuration: &Configuration) -> Result<(), Box<dyn Error>> {
	let file_string = fs::read_to_string(&configuration.filename)?;
	for (index, line) in search(&configuration.query, &file_string, configuration.is_case_sensitive) {
		println!("Line {}: {}", index, line);
	}

	Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
	let configuration = Configuration::new()?;
    
	println!("Searching for \"{}\" in file \"{}\"", configuration.query, configuration.filename);
    
	print_matching_file_lines(&configuration)?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";
		assert_eq!(
			vec![(1, "safe, fast, productive.")], 
			search(&query, &contents, true)
		);
	}
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(
			vec![(0, "Rust:"), (3, "Trust me.")],
			search(&query, &contents, false),
		);
	}
}
