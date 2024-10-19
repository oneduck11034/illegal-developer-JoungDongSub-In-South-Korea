pub fn print_unusally_string(){
    println!{"{} {} {:?}", char::MAX, char::REPLACEMENT_CHARACTER, char::UNICODE_VERSION};
}

fn main(){
    print_unusally_string();
}
