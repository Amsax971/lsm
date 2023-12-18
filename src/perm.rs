use std::os::unix::fs::PermissionsExt;
use std::fs;

pub fn perms(x: &str) -> std::io::Result<i32> {
    let meta = fs::metadata(x);
    let perm = meta?.permissions();
    // println!("{:?}", perm);
    // println!("{:o}", perm.mode());
    let perma = perm.mode().to_string();
    let permaint: i32 = perma.trim().parse().unwrap();
    Ok(permaint)
}