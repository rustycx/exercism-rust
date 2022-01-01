#[macro_export]
macro_rules! hashmap1 {
    ($($key:expr => $val:expr),+,) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val);)*
        map
    }};
    ($($key:expr => $val:expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val);)*
        map
    }};
}

// https://exercism.org/tracks/rust/exercises/macros/solutions/bobahop
#[macro_export]
macro_rules! hashmap0 {
    ($($($key: expr => $val: expr)+$(,)?)*) => {{
         let mut map = ::std::collections::HashMap::new();
         $($( map.insert($key, $val); )*)*
         map
    }}
}

#[test]
fn test0() {
    let _ = hashmap0![1=>'a', 2=>'b'];
}

// failed test_compile_fails_only_comma
#[macro_export]
macro_rules! hashmap2 {
    ($($x:literal => $y:expr),* $(,)?) => ({
        let mut hm = ::std::collections::HashMap::new();
        $(hm.insert($x, $y);)*
        hm
    });
    () => {::std::collections::HashMap::new()}
}

#[macro_export]
macro_rules! hashmap3 {
    () => {
        ::std::collections::HashMap::new()
    };
    ( $( $key:expr => $val:expr ),* ) => {
        $crate::hashmap3!($($key => $val,)*);
    };
    ( $( $key:expr => $val:expr, )* ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

// https://exercism.org/tracks/rust/exercises/macros/solutions/ocstl
#[macro_export]
macro_rules! hashmap {
    // Count the number of keys to preallocate the required size for the HashMap. Note that this
    // may yield a larger HashMap than purely necessary if some of the keys are duplicates.
    (@unit $k:expr) => { () };
    (@count $($k:expr),*) => { <[()]>::len(&[$(crate::hashmap!(@unit $k)),*]) };
    ($($key: expr => $val: expr),*) => {{
        let size = crate::hashmap!(@count $($key),*);
        let mut hm = ::std::collections::HashMap::with_capacity(size);
        $( hm.insert($key, $val); )*
        hm
    }};
    ($($key: expr => $val: expr,)*) => {{
        $crate::hashmap!($($key => $val),*)
    }};
}
