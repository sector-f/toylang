include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        assert_eq!(number("5.0"), Ok(5.0));
        assert_eq!(number("1_234.5"), Ok(1234.5));
        assert_eq!(number("-7.2"), Ok(-7.2));
    }
}
