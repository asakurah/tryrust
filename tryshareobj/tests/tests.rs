use tryshareobj::objx::ObjA;
use tryshareobj::objx::ObjB;
use tryshareobj::objx::ObjC;

#[test]
fn test_obj_a_initialize() {
    let obj_a = ObjA::new();
    assert_eq!(obj_a.val(), 0);
}

#[test]
fn test_obj_a_increment() {
    let obj_a = ObjA::new();
    unsafe {
        obj_a.inc();
    }
    assert_eq!(obj_a.val(), 1);
}

#[test]
fn test_obj_a_increment_2_times() {
    let obj_a = ObjA::new();
    unsafe {
        obj_a.inc();
        obj_a.inc();
    }
    assert_eq!(obj_a.val(), 2);
}




#[test]
fn test_obj_b_initialize() {
    let obj_a = ObjA::new();
    let _obj_b = ObjB::new(&obj_a);
    assert_eq!(obj_a.val(), 0);
}

#[test]
fn test_obj_b_increment() {
    let obj_a = ObjA::new();
    let mut obj_b = ObjB::new(&obj_a);
    obj_b.inc();
    assert_eq!(obj_a.val(), 2);
}

#[test]
fn test_obj_c_initialize() {
    let obj_a = ObjA::new();
    let _obj_c = ObjC::new(&obj_a);
    assert_eq!(obj_a.val(), 0);
}

#[test]
fn test_obj_c_increment() {
    let obj_a = ObjA::new();
    let mut obj_c = ObjC::new(&obj_a);
    obj_c.inc();
    assert_eq!(obj_a.val(), 5);
}

