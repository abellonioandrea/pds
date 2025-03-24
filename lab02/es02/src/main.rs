use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    if (args.len() > 0) {
        println!("{:?}", slugify(args[0].as_str()));
    }
}

fn slugify(s: &str) -> String {
    let mut res = String::new();
    let mut old = '-';
    let mut first:bool = true;
    for mut c in s.chars(){
       if c.is_uppercase(){
           c = c.to_ascii_lowercase();
       }
        c = conv(c);
        if(first == false){
            if(c == '-' && old == '-'){
             continue;
            }
        }
        first = false;
        res.push(c);
        old = c;
    }
    if(old == '-' && res.len() > 1){
        res.pop();
    }
    res
}

fn conv(c: char) -> char {
    const SUBS_I :&str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O :&str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    let vecSUBS_I : Vec<char> = SUBS_I.chars().collect();
    let vecSUBS_O : Vec<char> = SUBS_O.chars().collect();
    match vecSUBS_I.iter().position(|&a| a == c){
        Some(i) => vecSUBS_O[i],
        None => if(c.is_ascii_alphanumeric()) {c} else {'-'}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lettera_accentata() {
        assert_eq!(slugify("ò"), "o");
    }
    #[test]
    fn test_lettera_non_accentata(){
        assert_eq!(slugify("A"), "a");
    }
    #[test]
    fn test_lettera_non_ammessa_sconosciuta(){
        assert_eq!(slugify("@"), "-");
    }
    #[test]
    fn test_lettera_accentata_non_compresa(){
        assert_eq!(slugify("ῶ"), "-");
    }
    #[test]
    fn test_stringa_con_piu_parole(){
        assert_eq!(slugify("hello world"), "hello-world");
    }
    #[test]
    fn test_stringa_con_caratteri_accentati(){
        assert_eq!(slugify("hèllò wòrld"), "hello-world");
    }
    #[test]
    fn test_stringa_vuota(){
        assert_eq!(slugify(""), "");
    }
    #[test]
    fn test_stringa_con_piu_spazi_consecutivi(){
        assert_eq!(slugify("hello   world"), "hello-world");
    }
    #[test]
    fn test_stringa_con_piu_caratteri_non_validi_consecutivi(){
        assert_eq!(slugify("hello !£$%& world"), "hello-world");
    }
    #[test]
    fn test_stringa_con_solo_caratteri_non_validi(){
        assert_eq!(slugify("£$%&/(!/()"), "-");
    }
    #[test]
    fn test_stringa_con_spazio_alla_fine(){
        assert_eq!(slugify("hello world "), "hello-world");
    }
    #[test]
    fn test_stringa_con_piu_caratteri_non_validi_alla_fine(){
        assert_eq!(slugify("hello world%&/£()$"), "hello-world");
    }
}