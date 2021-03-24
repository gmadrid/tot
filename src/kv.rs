pub struct KV<'a>(&'a str, &'a str);

impl<'a> KV<'a> {}

impl<'a> KV<'a> {
    pub fn key(&self) -> &str {
        self.0
    }

    pub fn value(&self) -> &str {
        self.1
    }
}

pub fn from_str<'a, 'b>(s: &'a str, separator: &'b str) -> KV<'a> {
    if let Some(index) = s.find(separator) {
        KV(&s[..index], &s[index + 1..])
    } else {
        let (a, b) = s.split_at(s.len());
        KV(a, b)
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
