fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice"; //slice
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true
    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa: s3:hello-string s4:hello-slice
}

fn is_slug(s: &str) -> bool{
    if(slugify(s) == *s) {
        true
    }
    else{
        false
    }
}

trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}
/*
impl MySlug for &str{
    fn is_slug(&self) -> bool {
        is_slug(self)
    }

    fn to_slug(&self) -> String{
        slugify(self)
    }
}

impl MySlug for String{
    fn is_slug(&self) -> bool{
        is_slug(self)
    }

    fn to_slug(&self) -> String{
        slugify(self)
    }
}
*/

impl <T> MySlug for T where T: AsRef<str> {
    fn is_slug(&self) -> bool {
        is_slug(self.as_ref())
    }

    fn to_slug(&self) -> String{
        slugify(self.as_ref())
    }
}

fn slugify(s: &str) -> String {
    let mut res = String::new();
    let mut old = '-';
    let mut first: bool = true;
    let mut charVec: Vec<char> = Vec::new();
    for mut c in s.chars() {
        if c.is_uppercase() {
            charVec = c.to_lowercase().collect();
        }
        else{
            charVec.clear();
            charVec.push(c);
        }
        for c1 in &charVec {
            //così gestico i caratteri che da upper a lower sono più caratteri
            c = conv(*c1);
            if (first == false) {
                if (c == '-' && old == '-') {
                    continue;
                }
            }
            first = false;
            res.push(c);
            old = c;
        }
    }
    if (old == '-' && res.len() > 1) {
        res.pop();
    }
    res
}

fn conv(c: char) -> char {
    const SUBS_I: &str =
        "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str =
        "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    let vecSUBS_I: Vec<char> = SUBS_I.chars().collect();
    let vecSUBS_O: Vec<char> = SUBS_O.chars().collect();
    match vecSUBS_I.iter().position(|&a| a == c) {
        Some(i) => vecSUBS_O[i],
        None => {
            if (c.is_ascii_alphanumeric()) {
                c
            } else {
                '-'
            }
        }
    }
}