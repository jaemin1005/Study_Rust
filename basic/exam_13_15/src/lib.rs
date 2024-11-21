#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let v1: Vec<i32> = vec![1, 2, 3];

        //* '_' 타입추론을 의미한다 */
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}
