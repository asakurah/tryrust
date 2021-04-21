

pub struct ObjA {
    cnt: i64
}

impl ObjA {
    pub fn new() -> ObjA {
        ObjA {
            cnt: 0
        }
    }

    pub fn val(&self) -> i64 {
        self.cnt
    }

    #[allow(mutable_transmutes)]
    pub unsafe fn inc(&self) {
        let obj = std::mem::transmute::<&ObjA, & mut ObjA>(self);
        obj.cnt += 1;
    }
}




pub struct ObjB<'a> {
    obj_a : & 'a ObjA
}

impl<'a> ObjB<'a> {
    pub fn new(obj_a: &'a ObjA) -> ObjB {
        ObjB {
            obj_a
        }
    }

    pub fn inc(&mut self) {
        unsafe {
            self.obj_a.inc();
            self.obj_a.inc();
        }
    }
}


pub struct ObjC<'x> {
    obj_a : & 'x ObjA,
    obj_b_1 : ObjB<'x>,
    obj_b_2 : ObjB<'x>,
}

impl<'x> ObjC<'x> {
    pub fn new(obj_a_in: &'x ObjA) -> ObjC {
        ObjC {
            obj_a: obj_a_in,
            obj_b_1: ObjB::new(obj_a_in),
            obj_b_2: ObjB::new(obj_a_in),
        }
    }

    pub fn inc(&mut self) {
        self.obj_b_1.inc();
        self.obj_b_2.inc();
        unsafe {
            self.obj_a.inc();
        }
    }
}

