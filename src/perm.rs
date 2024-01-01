use std::os::unix::fs::PermissionsExt;
use std::fs;

pub fn perms(x: &str) -> i32 {
    let meta = fs::metadata(x).unwrap();
    let perm = meta.permissions();
    let perma = perm.mode().to_string();
    let permaint: i32 = perma.trim().parse().unwrap();   
    return permaint;
}
