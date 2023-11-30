pub fn run(input: String) -> String {
    input
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::read_input;

    #[test]
    fn test_day01() {
        assert_eq!(run(read_input(1, true)), "sample");
    }
}
