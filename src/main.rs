fn main() {
    let pid = std::env::args().nth(1).expect("no pid given");
    let uid_path = format!("{}{}{}", "/proc/",pid,"/uid_map");
    let gid_path = format!("{}{}{}", "/proc/",pid,"/gid_map");
    std::fs::write(uid_path, "0 1000 1")
        .expect("unable to write uid_map file");
    std::fs::write(gid_path, "0 1000 65535")
        .expect("unable to write gid_map file");
}
