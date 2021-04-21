use std::thread;
use std::time;
use std::sync;


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


struct Counter {
    n: i32,
}

impl Counter {
    fn new() -> Counter{
        Counter{ n: 0 }
    }

    fn from_i32(n: i32) -> Counter{
        Counter{ n: n }
    }

    fn inc(& mut self) {
        self.n = self.n + 1;
    }

    fn val(& self) -> i32 {
        self.n
    }
}


fn multiple_thread_sharing_immutable_object() {
    println!("start: multiple_thread_sharing_immutable_object");

    let n = 10;
    let mut thread_handles = vec![];
    let shared_obj = sync::Arc::new(Counter::from_i32(123));

    for i in 0..n {
        let shared_obj = shared_obj.clone();
        let thread_handle = thread::spawn(move || {
            let dur = time::Duration::new(1, 0);
            thread::sleep(dur);
            println!("hello: {}. shared_obj={}", i, shared_obj.val());
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


fn multiple_thread_sharing_mutable_object() {
    println!("start: multiple_thread_sharing_mutable_object");

    let n = 10;
    let mut thread_handles = vec![];
    let shared_obj = sync::Arc::new(sync::Mutex::new(Counter::new()));

    for i in 0..n {
        let shared_obj = shared_obj.clone();
        let thread_handle = thread::spawn(move || {
            let dur = time::Duration::new(1, 0);
            thread::sleep(dur);
            {
                let mut cnt = shared_obj.lock().unwrap();
                cnt.inc();
                println!("hello: {}. shared_obj={}", i, cnt.val());
            }
        });
        thread_handles.push(thread_handle);
    }

    for thread_handle in thread_handles {
        match thread_handle.join() {
            Ok(_) => println!("OK"),
            Err(_) => println!("Err"),
        }
    }

    {
        let cnt = shared_obj.lock().unwrap();
        println!("last: shared_obj={}", cnt.val());
    }
}


struct MyObj {
    n: i32,
}

impl MyObj {
    fn new() -> MyObj {
        MyObj{n:0}
    }

    fn val(&self) -> i32 {
        self.n
    }

    unsafe fn unsafe_set(&self, v:i32) {
        let n_ptr = &self.n;
        let n_mut_ptr = std::mem::transmute::<*const i32, *mut i32>(n_ptr);
        *n_mut_ptr = v;
    }
}


fn multiple_thread_sharing_unsafe_object() {
    println!("start: multiple_thread_sharing_unsafe_object");

    let n = 10;
    let mut thread_handles = vec![];
    let shared_obj = sync::Arc::new(MyObj::new());

    for i in 0..n {
        let shared_obj = shared_obj.clone();
        let thread_handle = thread::spawn(move || {
            let dur = time::Duration::new(1, 0);
            thread::sleep(dur);
            unsafe {
                shared_obj.unsafe_set(shared_obj.val() + 1);
            }
            println!("hello: {}. shared_obj={}", i, shared_obj.val());
        });
        thread_handles.push(thread_handle);
    }

    for thread_handle in thread_handles {
        match thread_handle.join() {
            Ok(_) => println!("OK"),
            Err(_) => println!("Err"),
        }
    }

    let obj = shared_obj.clone(); 
    println!("last: shared_obj={}", obj.val());
}


fn main() {
    println!("start: main");

    single_thread();
    multiple_thread();
    multiple_thread_sharing_immutable_object();
    multiple_thread_sharing_mutable_object();
    multiple_thread_sharing_unsafe_object();
}
