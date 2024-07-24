use std::env;

fn main() {
    let home_dir = env::var("HOME").unwrap();
    //#[cfg(target_os = "windows")]
    //forward_dll::forward_dll("C:\\Windows\\System32\\version.dll").unwrap();
    forward_dll::forward_dll(&(home_dir + "/Downloads/version-orig.dll")).unwrap();
}
