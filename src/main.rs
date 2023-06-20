use std::thread;

fn main() {
    // Using a thread here because we can't edit the uid/gid of the current process
    let handler = thread::spawn(|| {
        let pid = std::env::args().nth(1).expect("no pid given");

        #[link(name = "c")]
        extern "C" {
            fn geteuid() -> u32;
            fn getegid() -> u32;
        }

        let uid = unsafe { geteuid().to_string() };
        let gid = unsafe { getegid().to_string() };

        // Maps just the root user, otherwise we'd need root
        let uid_map = format!("0 {} 1", uid);
        // Maps all groups, requires cap_setgid
        let gid_map = format!("0 {} 65535", gid);

        let uid_path = format!("{}{}{}", "/proc/",pid,"/uid_map");
        let gid_path = format!("{}{}{}", "/proc/",pid,"/gid_map");

        std::fs::write(uid_path, uid_map)
            .expect("unable to write uid_map file");
        std::fs::write(gid_path, gid_map)
            .expect("unable to write gid_map file");
        });

    handler.join().unwrap();
}
