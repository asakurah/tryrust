use std::alloc::{alloc, dealloc, Layout, LayoutErr};


fn try_alloc() -> Result<(), LayoutErr> {
    let layout = Layout::from_size_align(1024, 8)?;

    unsafe {
        let p1 = alloc(layout);
        let p2 = std::mem::transmute::<*mut u8, *mut i32>(p1);
        *p2 = -2;

        println!("v[0] = {}", *p1);
        println!("v[1] = {}", *(p1.offset(1)));
        println!("v[2] = {}", *(p1.offset(2)));
        println!("v[3] = {}", *(p1.offset(3)));

        dealloc(p1, layout);
    }

    Ok(())
}


fn main() {
    println!("Hello, world!");

    match try_alloc() {
        Ok(_) => println!("ok"),
        Err(_) => println!("error"),
    }
}
