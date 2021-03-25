pub struct Kv<'a>(&'a str, &'a str);

impl<'a> Kv<'a> {}

impl<'a> Kv<'a> {
    pub fn key(&self) -> &str {
        self.0
    }

    pub fn value(&self) -> &str {
        self.1
    }
}

pub fn from_str<'a, 'b>(s: &'a str, separator: &'b str) -> Kv<'a> {
    if let Some(index) = s.find(separator) {
        Kv(&s[..index], &s[index + 1..])
    } else {
        let (a, b) = s.split_at(s.len());
        Kv(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let kv = from_str("foo:bar", ":");
        assert_eq!("foo", kv.0);
        assert_eq!("bar", kv.1);
    }

    #[test]
    fn test_empty_val() {
        let kv = from_str("foo:", ":");
        assert_eq!("foo", kv.0);
        assert_eq!("", kv.1);
    }

    #[test]
    fn test_no_separator() {
        let kv = from_str("foo", ":");
        assert_eq!("foo", kv.0);
        assert_eq!("", kv.1);
    }
}
