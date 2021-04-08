

#[macro_export]
macro_rules! hash_map(
    { $($key:expr => $value:expr),* } => {
        {
            let mut values = std::collections::HashMap::new();
            $(
                values.insert($key, $value);
            )*
            values
        }
     };
);


