fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let mut v2 = v1.iter().map(|x| x + 1);

        assert_eq!(v2.next(), Some(2));
        assert_eq!(v2.next(), Some(3));
        assert_eq!(v2.next(), Some(4));
        assert_eq!(v2.next(), None);
    }
}