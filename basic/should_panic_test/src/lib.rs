pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got{}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, get{}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //* expected 매개변숫값이 Guess::new 함수에서 발생한 패닉 메시지 문자열의 일부임으로 통과 */
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
