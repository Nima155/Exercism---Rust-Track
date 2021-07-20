


#[macro_export]
macro_rules! hashmap {
   
    ($($key:expr => $value:expr),+ $(,)?) => { // can have one trailing comma at most
        //.. * matches only the pattern inside the parentheses
        {
            use ::std::collections::HashMap;
            let mut h_map = HashMap::new();
            $(
                h_map.insert($key, $value);
            )*
            h_map
        }
    };
    () => { // empty case
        {
            use ::std::collections::HashMap;
            HashMap::new()
        }
    }
}
