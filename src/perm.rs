use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::path::Path;


pub fn perms(x: &str) -> &str {
    let meta = fs::metadata(x).unwrap();
    let perm = meta.permissions();
    let perms = perm.mode().to_string();
    let perma: i32 = perms.trim().parse().unwrap();   
    

    let fileorn = Path::new(&x).is_file();


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
        } else if perma == 33024 {
            permsar = "-r------"
        } else if perma == 33152 {
            permsar = "-rw------"
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
        } else if perma == 33133 {
            permsar = "-r-xr-xr-x";
        } else if perma == 33261 {
            permsar = "-rwxr-xr-x";
        } else if perma == 33277 {
            permsar = "-rwxrwxr-x"
        } else if perma == 33279 {
            permsar = "-rwxrwxrwx"
        } else if perma == 33263 {
            permsar = "-rwxr-xrwx"
        } else if perma == 33097 {
            permsar = "-r-x-----"
        } else if perma == 33225 {
            permsar = "-rwx------"
        } else if perma == 33257 {
            permsar = "-rwxr-x---"
        } else if perma == 33273 {
            permsar = "-rwxrwx---"
        } else if perma == 33229 {
            permsar = "-rwx---r-x"
        } else if perma == 33231 {
            permsar = "-rwx---rwx"
        } else if perma == 33129 {
            permsar = "-r-xr-x---"
        } else if perma == 33145 {
            permsar = "-r-xrx---"
        } else if perma == 33101 {
            permsar = "-r-x---r-x"
        } else if perma == 33103 {
            permsar = "-r-x---rwx"
        } 
    } else {
        permsar = "drwxrwxr-x"
    }
    
    return permsar;
}