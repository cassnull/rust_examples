#[allow(dead_code)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f1: 292 };
    let _f = unsafe { u.f1 };
}
