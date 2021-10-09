#[macro_export]
macro_rules! try_match {
    ($expression:expr, $( $pattern:pat_param )|+ $( if $guard: expr )? => $out: expr) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => ::core::option::Option::Some($out),
            _ => ::core::option::Option::None,
        }
    };
}