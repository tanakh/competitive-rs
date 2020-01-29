#[macro_export]
macro_rules! echo {
    ($e:expr) => {
        println!("{}", $e);
    };
    ($e:expr, $($rest:tt)+) => {
        print!("{} ", $e);
        echo!($($rest)*);
    };
}
