pub fn translate(text: &str, from: char, to: char) -> String {

    let mut new = String::new();
    for i in text.chars(){
        if i == from {
            new.push_str(to.to_string().as_str());
        } else{
            new.push_str(i.to_string().as_str());
        }
    }
    new
}

pub fn delete(text: &str, ch: char) -> String {
    let mut new = String::new();
    for i in text.chars(){
        if i == ch {
            continue;
        } else{
            new.push_str(i.to_string().as_str());
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_replaces_every_match() {
        assert_eq!(translate("hello", 'l', 'L'), "heLLo");
    }

    #[test]
    fn test_translate_no_match() {
        assert_eq!(translate("abc", 'z', 'Z'), "abc");
    }

    #[test]
    fn test_delete_removes_every_match() {
        assert_eq!(delete("h.e.l.l.o", '.'), "hello");
    }

    #[test]
    fn test_delete_no_match() {
        assert_eq!(delete("abc", 'z'), "abc");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(translate("", 'a', 'b'), "");
        assert_eq!(delete("", 'a'), "");
    }
}