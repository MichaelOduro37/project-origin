fn main() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_cpu_usage(); // this is the proper way in newer sysinfo to just refresh usage
    println!("CPU load: {:?}", sys.cpus().iter().map(|c| c.cpu_usage()).collect::<Vec<_>>());
    std::thread::sleep(std::time::Duration::from_millis(1500));
    sys.refresh_cpu_usage();
    println!("CPU load: {:?}", sys.cpus().iter().map(|c| c.cpu_usage()).collect::<Vec<_>>());
}
