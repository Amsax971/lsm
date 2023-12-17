use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;
use colored::Colorize;


fn main() {
	if let Err(ref e) = run(Path::new(".")) {
		println!("{}", e);
		process::exit(1);
	}
}

fn run(dir: &Path) -> Result<(), Box< dyn Error>> {
	if dir.is_dir() {
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
				flens = flenf.to_string();
				if Path::new(&file_name).is_file() {
					if k == 0 {
						println!("{}{}	{}",flens.green() ,"o".green() ,file_name.blue());
					} else if k == 1 {
						println!("{}{}	{}",flens.green() ,"ko".green() ,file_name.blue());
					} else if k == 2 {
						println!("{}{}	{}",flens.green() ,"mo".green() ,file_name.blue());
					}
				} else {
					println!("{}", file_name.green());
				}
			}
		}
	Ok(())
}