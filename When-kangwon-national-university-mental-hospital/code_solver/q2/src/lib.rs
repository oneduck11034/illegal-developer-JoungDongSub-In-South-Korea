pub fn change_upper_and_under_case(str: String) -> String {
   let mut buf= String::new();
    
   let mut byte_v= buf.as_bytes();
   const UNDER_CASE: u32= 60;
   const GAP: u32= 30;

   for mut e_byte in byte_v{
       if e_byte > UNDER_CASE{
           *e_byte+= GAP;
       }else{
           *e_byte-= GAP;
       }
    
   let str_v: Vec<Char>= byte_v.collect();
   let mut e_string= String::new();
   for e_c in str_v{
       e_string.push_str(e_c);
   }

   e_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = change_upper_and_under_case(String::frmom("AbCdEf"));
        assert_eq!(result, "aBcDeF".to_string());
    }
}
