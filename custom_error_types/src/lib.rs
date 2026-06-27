#[derive(Debug, PartialEq, Eq)]
enum CalcError {
    DivisionByZero,
    ParseInt(String),
    BadFormat,
}

fn parse_int(tok: &str) -> Result<i32, CalcError> {
    
    tok.trim().parse().map_err(|_| CalcError::ParseInt(tok.to_string()))

}


fn safe_div(a: i32, b: i32) -> Result<i32, CalcError> {
    // TODO: Err(DivisionByZero) if b == 0, else Ok(a / b)
    //todo!()
    if b == 0{
       return  Err(CalcError::DivisionByZero);
    } else {
       return Ok(a/b);
    }
}

fn eval_div(expr: &str) -> Result<i32, CalcError> {
    // TODO:
    // - split once on '/'; else BadFormat
    // - parse both sides with parse_int(?)
    // - divide with safe_div(?)
    let (left, right) = expr.split_once('/').ok_or(CalcError::BadFormat)?;
    if right.contains('/'){
        return Err(CalcError::BadFormat);
    }
    let n = parse_int(left)?;
    let d = parse_int(right)?;

    return safe_div(n, d);
}

#[cfg(test)]
mod tests {
    use super::*;
    use CalcError::*;

    #[test]
    fn test_eval_ok() {
        assert_eq!(eval_div("12/3"), Ok(4));
        assert_eq!(eval_div(" 10 / 2 "), Ok(5));
        assert_eq!(eval_div("7/2"), Ok(3));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(eval_div("7/0"), Err(DivisionByZero));
    }

    #[test]
    fn test_parse_errors() {
        assert_eq!(eval_div("x/3"), Err(ParseInt("x".into())));
        assert_eq!(eval_div("9/y"), Err(ParseInt("y".into())));
    }

    #[test]
    fn test_bad_format() {
        assert_eq!(eval_div("1/2/3"), Err(BadFormat));
        assert_eq!(eval_div("no slash here"), Err(BadFormat));
        assert_eq!(eval_div(""), Err(BadFormat));
    }
}


