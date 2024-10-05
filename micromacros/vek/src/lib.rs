#[macro_export]
macro_rules! vek {
    ( $( $x:expr ),* ) => {{
        Vec::from([$($x,)*])
    }};
}
