// 
// Sysinfo
// 
// Copyright (c) 2017 Guillaume Gomez
//

extern crate sysinfo;

#[test]
fn test_disks() {
    use sysinfo::SystemExt;

    let s = sysinfo::System::new();
    println!("total memory: {}", s.get_total_memory());
    println!("total cpu cores: {}", s.get_processor_list().len());
}

fn fib(n: u64) -> u64 {
    if n > 2 {
        fib(n - 1) + fib(n - 2)
    } else {
        1
    }
}

#[test]
fn test_process() {
    use sysinfo::SystemExt;
    use sysinfo::ProcessExt;
    let mut system = sysinfo::System::new();
    system.refresh_all();

    let pid = sysinfo::get_current_pid();

    let mut i = 30;
    loop {
        system.refresh_process(pid);
        let process = system.get_process(pid).unwrap();

        i += 1;
        if i > 40 {
            i = 30;
        }
        fib(i);
        println!("CPU: {}%", process.cpu_usage());
        println!("Memory: {}", process.memory());

        ::std::thread::sleep_ms(1000);
    }
}
