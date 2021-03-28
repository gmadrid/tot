pub struct KvParser<'a>(&'a str);

impl<'a> KvParser<'a> {
    pub fn with_separator(separator: &str) -> KvParser {
        KvParser(separator)
    }

    pub fn parse<'b>(&self, s: &'b str) -> Kv<'b> {
        if let Some(index) = s.find(self.0) {
            Kv(&s[..index], &s[index + 1..])
        } else {
            let (a, b) = s.split_at(s.len());
            Kv(a, b)
        }
    }
}

pub struct Kv<'a>(&'a str, &'a str);

impl<'a> Kv<'a> {
    pub fn key(&self) -> &str {
        self.0
    }

    pub fn value(&self) -> &str {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_one<'a, 'b>(s: &'a str, sep: &'b str) -> Kv<'a> {
        KvParser::with_separator(sep).parse(s)
    }

    #[test]
    fn test_simple() {
        let kv = parse_one("foo:bar", ":");
        assert_eq!("foo", kv.0);
        assert_eq!("bar", kv.1);
    }

    #[test]
    fn test_empty_val() {
        let kv = parse_one("foo:", ":");
        assert_eq!("foo", kv.0);
        assert_eq!("", kv.1);
    }

    #[test]
    fn test_no_separator() {
        let kv = parse_one("foo", ":");
        assert_eq!("foo", kv.0);
        assert_eq!("", kv.1);
    }
}
