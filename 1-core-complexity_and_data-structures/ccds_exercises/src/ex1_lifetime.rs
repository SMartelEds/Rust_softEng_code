fn longest_common_suffix<'a>(a: &'a str, b: &'a str) -> Option<&'a str> {
    if a.is_empty() || b.is_empty() {
        return None;
    }
    if len(a) > len(b){
        let shortest: i8 = len(b);
    } else {
        let shortest: i8 = len(a);
    }
    ar = a::rev()::chars();
    br = b::rev()::chars();
    let mut suffix: Vec<&str>;
    
    let mut i: i8 = 0;
    while ar[i] == br[i]{
        suffix.push(ar[i]);
    }
}

fn get_shortest(a: &str, b: &str)->i8{

}