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
		
		// getting files

		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				
				let fileorn = Path::new(&file_name).is_file();

				// getting lenght
				
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

				// getting permisions

				let perma = perms(&file_name);

				let mut permsar:&str = "----------";

				if fileorn {
					if perma == 33060 {
						permsar = "-r--r--r--";
					} else if perma == 33188 {
						permsar = "-rw-r--r--";
					} else if perma == 33204 {
						permsar = "-rw-rw-r--"
					} else if perma == 33206 {
						permsar = "-rw-rw-rw-"
					} else if perma == 33190 {
						permsar = "-rw-r--rw-"
					} else if perma == 33204 {
						permsar = "-rw-rw-r--"
					} else if perma == 33152 {
						permsar = "-r-------"
					} else if perma == 33184 {
						permsar = "-rw-r----"
					} else if perma == 33200 {
						permsar = "-rw-rw----"
					} else if perma == 33156 {
						permsar = "-rw----r--"
					} else if perma == 33158 {
						permsar = "-rw----rw-"
					} else if perma == 33056 {
						permsar = "-r--r-----"
					} else if perma == 33072 {
						permsar = "-r--rw----"
					} else if perma == 33028 {
						permsar = "-r-----r--"
					} else if perma == 33030 {
						permsar = "-r-----rw-"
					}
				}

				// printing

				if fileorn {
					if k == 0 {
						println!("{}{}	{}	{}",flens.cyan() ,"o".cyan(),permsar ,file_name.blue());
					} else if k == 1 {
						println!("{}{}	{}	{}",flens.cyan() ,"ko".cyan(),permsar ,file_name.blue());
					} else if k == 2 {
						println!("{}{}	{}	{}",flens.cyan() ,"mo".cyan(),permsar ,file_name.blue());
					}
				} else {
						println!("{}	{}","dir".cyan() ,file_name.green());
				}
			}
	Ok(())
}