use std::fs;

pub fn len(x:&str) -> f32 {
    let flen  = fs::metadata(x.clone()).unwrap().len();
				let flens = flen.to_string();
				let flenint: f32 = flens.parse().unwrap();
    return flenint;
}