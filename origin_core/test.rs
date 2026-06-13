fn main() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let cpus = sys.cpus();
    println!("{}", cpus.len());
}
