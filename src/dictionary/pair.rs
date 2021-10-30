use std::fmt;


/// A KeyValue Pair for use in creating a `Dictionary` / `HashMap`.
/// ## Fields:
/// ```rust
/// pub key: K // Key of the Pair.
/// pub value: V // Value of the Pair.
/// ```
#[derive(Debug)]
pub struct Pair<K, V> {
    pub key: K,
    pub value: V,
}


impl<K, V> Pair<K, V> {
    pub fn new(key: K, value: V) -> Self {
        return Self {
            key,
            value,
        };
    }
}


impl<K: Default, V: Default> Default for Pair<K, V> {
    fn default() -> Self {
        return Self {
            key: K::default(),
            value: V::default(),
        };
    }
}


impl<K: fmt::Display, V: fmt::Display> fmt::Display for Pair<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}: {})", self.key, self.value);
    }
}


#[cfg(test)]
mod tests {
    use super::Pair;
    
    #[test]
    fn create_int_int_pair() {
        let pair: Pair<i32, i32> = Pair::new(0, 1337);
        assert_eq!(pair.key, 0);
        assert_eq!(pair.value, 1337);
    }

    #[test]
    fn create_int_str_pair() {
        let pair: Pair<i32, &str> = Pair::new(0, "Leet");
        assert_eq!(pair.key, 0);
        assert_eq!(pair.value, "Leet");
    }

    #[test]
    fn create_str_int_pair() {
        let pair: Pair<&str, i32> = Pair::new("Zero", 1337);
        assert_eq!(pair.key, "Zero");
        assert_eq!(pair.value, 1337);
    }

    #[test]
    fn create_str_str_pair() {
        let pair: Pair<&str, &str> = Pair::new("Zero", "Leet");
        assert_eq!(pair.key, "Zero");
        assert_eq!(pair.value, "Leet");
    }

    #[test]
    fn default() {
        let pair: Pair<u8, u8> = Pair::default();
        assert_eq!(pair.key, 0);
        assert_eq!(pair.value, 0);
    }

    #[test]
    fn display() {
        let pair: Pair<i32, i32> = Pair::new(300, 5);
        assert_eq!(format!("{}", pair), "(300: 5)");
    }
}