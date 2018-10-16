pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
