// a declarative macro
#[macro_export]
macro_rules! victor {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = victor!["A", "B", "C"];

    dbg!(v);
}
