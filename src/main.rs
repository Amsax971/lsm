use std::fs;
use std::path::Path;
use std::error::Error;
use std::process;
use colored::*;
use perm::perms;

mod perm;
mod len;

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
				
				let flen = len::len(&file_name);

				let mut flenf: f32 = flen;

				let mut k:i32  = 0;

				if flen > 1024. {
					flenf = flenf / 1024.;
					
					flenf = flenf.round();

					k+=1;

					if flenf > 1024. {
						flenf = flenf / 1024.;

						k+=1; 
						flenf = flenf.round();

						if flenf > 1024. {
							flenf = flenf / 1024.;
	
							k+=1; 
							flenf = flenf.round();
						}
					}
				}


				let flens = flenf.to_string();

				// getting permisions

				let perma = perms(&file_name);
		
				// printing

				if fileorn {
					if k == 0 {
						println!("{}{}	{}	{}",flens.cyan().bold() ,"o".cyan(),perma.truecolor(255, 121, 79) ,file_name.blue());
					} else if k == 1 {
						println!("{}{}	{}	{}",flens.cyan().bold() ,"Ko".cyan(),perma.truecolor(255, 121, 79) ,file_name.blue());
					} else if k == 2 {
						println!("{}{}	{}	{}",flens.cyan().bold() ,"Mo".cyan(),perma.truecolor(255, 121, 79) ,file_name.blue());
					} else if k == 2 {
						println!("{}{}	{}	{}",flens.cyan().bold() ,"Go".cyan(),perma.truecolor(255, 121, 79) ,file_name.blue());
					}
				} else {
						println!("{}	{}	{}","dir".cyan().bold() ,perma.truecolor(255, 121, 79) ,file_name.truecolor(79, 255, 83).bold());
				}
			}
	Ok(())
}