use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;
use colored::Colorize;
use perm::perms;

mod perm;

fn main() {
	if let Err(ref e) = run(Path::new(".")) {
		println!("{}", e);
		process::exit(1);
	}
}

fn run(dir: &Path) -> Result<(), Box< dyn Error>> {
		
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				let flen  = fs::metadata(file_name.clone())?.len();
				let mut flens = flen.to_string();
				let flenint: f32 = flens.parse().unwrap();

				let mut flenf: f32 = flenint;

				let mut k:i32  = 0;

				if flen > 1024 {
					flenf = flenf / 1024.;
					
					flenf = flenf.round();


					if flenf > 1024. {
						flenf = flenf / 1024.;

						k+=1; 
						flenf = flenf.round();
					}
				}

				let perma = perms(&file_name);
				flens = flenf.to_string();
				if Path::new(&file_name).is_file() {
					if k == 0 {
						println!("{}{}	{:?}	{}",flens.cyan() ,"o".cyan(),perma ,file_name.blue());
					} else if k == 1 {
						println!("{}{}	{:?}	{}",flens.cyan() ,"ko".cyan(),perma ,file_name.blue());
					} else if k == 2 {
						println!("{}{}	{:?}	{}",flens.cyan() ,"mo".cyan(),perma ,file_name.blue());
					}
				} else {
						println!("{}	{}","dir".cyan() ,file_name.green());
				}
			}
	Ok(())
}