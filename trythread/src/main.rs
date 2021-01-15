use std::thread;
use std::time;

fn single_thread() {
    println!("start: single_thread");

    let thread_handle = thread::spawn(||{
        let dur = time::Duration::new(1, 0);
        thread::sleep(dur);
        println!("hello");
    });

    match thread_handle.join() {
        Ok(_) => println!("OK"),
        Err(_) => println!("Err"),
    }
}



fn multiple_thread() {
    println!("start: multiple_thread");

    let n = 10;
    let mut thread_handles = vec![];

    for i in 0..n {
        let thread_handle = thread::spawn(move || {
            let dur = time::Duration::new(1, 0);
            thread::sleep(dur);
            println!("hello: {}", i);
        });
        thread_handles.push(thread_handle);
    }

    for thread_handle in thread_handles {
        match thread_handle.join() {
            Ok(_) => println!("OK"),
            Err(_) => println!("Err"),
        }
    }
}


fn main() {
    println!("start: main");

    single_thread();
    multiple_thread();
}
