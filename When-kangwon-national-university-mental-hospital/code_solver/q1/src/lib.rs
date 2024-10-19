pub fn add(str: String, number: u8) -> String {
    let mut v: Vec<String> = Vec::new();
    for _ in 0..number {
        v.push(str.clone());
    }

    let mut result = String::new();
    for e_str in v {
        // @todo
        &result.as_push(e_str);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add("string".to_sring(), 5);
        assert_eq!(result, "stringstringstringstringstring".to_string());
    }
}
