macro_rules! my_map {
    (
        $(
          $k:expr => $v:expr
        )
        , // 重复的分割符
        *
    ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
              map.insert($k,$v);
            )*
            map
        }

    };
}

fn main() {
    let a = my_map! {
        "1" => 1,
        "2" => 2
    };

    assert_eq!(a["1"], 1);
    assert_eq!(a["2"], 1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_map_macro() {
        let a = my_map! {
            "1" => 1,
            "2" => 2
        };

        assert_eq!(a["1"], 1);
        assert_eq!(a["2"], 2);
    }
}
