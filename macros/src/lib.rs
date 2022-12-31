#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $value: expr),*) => {
        {
            let count = $crate::count!($($key),*);
            let mut hm = crate::HashMap::with_capacity(count);
            $(hm.insert($key, $value);)*
            hm
        }
    };
    ($($key: expr => $value: expr,)*) => {
        {
            $crate::hashmap!($($key => $value),*)
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    ($($exp: expr),*) => {
        <[()]>::len(&[$($crate::count!(@SUBS $exp)),*])
    };
    (@SUBS $exp: expr) => { () };
}
