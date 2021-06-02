pub fn defang_i_paddr(address: String) -> String {
    let mut result = String::with_capacity(address.len() + 6);
    for c in address.chars() {
        if c == '.' {
            result.push_str("[.]");
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defang_i_paddr_works() {
        let input = "255.100.50.0";
        let expected = "255[.]100[.]50[.]0";
        let result = defang_i_paddr(input.to_string());
        assert_eq!(expected, result);
    }
}
