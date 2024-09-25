#[macro_export]
macro_rules! vek {
    ( $( $x:expr ),* ) => {{
        Vec::from([$($x,)*])
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vek_bang() {
        let v = vek![1, 3, 5];
        assert_eq!(vec![1, 3, 5], v);
    }
}
