pub fn cat_n(text: &str) -> String {
    
    let mut words:Vec<String> = Vec::new();
    for (idx, line) in text.lines().enumerate(){
        let n = idx + 1;
        let tempstr = format!("{n}\t{line}");
        words.push(tempstr) ;
    }
    let sentence = words.join("\n");
    sentence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_lines() {
        assert_eq!(cat_n("foo\nbar\nbaz"), "1\tfoo\n2\tbar\n3\tbaz");
    }

    #[test]
    fn test_single_line() {
        assert_eq!(cat_n("only"), "1\tonly");
    }

    #[test]
    fn test_blank_line_kept() {
        assert_eq!(cat_n("a\n\nb"), "1\ta\n2\t\n3\tb");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(cat_n(""), "");
    }
}