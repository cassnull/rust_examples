use std::slice;

fn split_at_mut<T>(values: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // доступ к сырому указателю среза

    assert!(mid <= len);

    unsafe {
        (
            // Функция slice::from_raw_parts_mut небезопасна, потому что она принимает сырой указатель и должна верить, что этот указатель действителен
            slice::from_raw_parts_mut(ptr, mid), // принимает сырой указатель, длину и создаёт срез
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
