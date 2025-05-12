// to warm up: the define step by step an adapter for filtering even numbers

pub mod simple_even_iter {
    // (1) let start with a simple iterator adapter for just one type, "i32"
    // see the adapter pattern example in the pdf "Adapter Pattern..."

    // sol: the EvenIter struct is a generic type, it can be used for wrapping any type.
    // We will will impose the constraint that the type must be an iterator over i32 later, when
    // implementing the Iterator trait.
    struct EvenIter<I> {
        inner: I, // hint: it's a generic type... here we don't care about bounds yet
    }

    impl<I> EvenIter<I> {
        fn new(iter: I) -> Self {
            EvenIter { inner: iter }
        }
    }

    impl<I> Iterator for EvenIter<I>
    where
    // I it must be an iterator over i32, here we define the constraint
        I: Iterator<Item=i32>,
    {
        type Item = i32; // <== it will work just for i32

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.inner.next() {
                    Some(x) => {
                        if x % 2 == 0 {
                            return Some(x);
                        }
                    }
                    None => return None,
                }
            }
        }
    }

    // if EvenIter works the test will compile and pass
    #[test]
    fn test_simple_even_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // why iter() does not work here?
        let it = EvenIter::new(v.into_iter());
        for i in it {
            println!("i: {}", i);
        }
    }

    // (2) now let's add the adapter to all Iterator<Item=i32> (adavanced)
    // any iterator over i32 can be used with the even() method
    trait AddEvenIter: Iterator
    where
        Self: Sized,
    {
        // add even() to anyone implementing this trait
        // usage: v.into_iter().even() ....
        fn even(self) -> EvenIter<Self> {
            EvenIter::new(self)
        }
    }

    // (3) add here the generic implemention, you can supply it for all the iterators
    // sol: the implementation is already done in the AddEvenIter struct, here we just mark 
    // any iterator over i32 so that we can use the even() method 
    impl<I> AddEvenIter for I
    where
        I: Iterator<Item=i32>,
    {}

    #[test]
    fn test_adapter() {
        let v = vec![1, 2, 3, 4, 5];
        for i in v.into_iter().even() {
            println!("{}", i);
        }
    }
}

pub mod even_iter {
    // (4) more adavanced: implement for all integer types 
    // => install the external crate "num" to have some Traits identifying all number types
    use num;

    // the generic parameters I and U are already defined for you in the struct definition
    // (5) write in a comment in plain english the meaning of the generic parameters 
    // and their constraints
    pub struct EvenIter<I, U>
    // sol: there are two generic parameters:
    // I: the iterator type, it must implement the Iterator trait
    // U: the type of the item returned by the iterator, it must implement the num::Integer trait
    where
        I: Iterator<Item=U>,
    {
        iter: I,
    }

    impl<I, U> Iterator for EvenIter<I, U>
    where
        U: num::Integer + Copy,
        I: Iterator<Item=U>,
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.iter.next() {
                    Some(x) => {
                        if x.is_even() {
                            return Some(x);
                        }
                    }
                    None => return None,
                }
            }
        }
    }

    // (6) once implemented, this example in the test will compile
    #[test]
    fn test_even_iter() {
        let v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let it = EvenIter { iter: v.into_iter() };
        for i in it {
            println!("i: {}", i);
        }
    }
}

// finally let's implement the grep command
// (1) install the "walkdir" crate for walking over directories using an iterator
// install also the "regex" crate for regular expressions

use std::{fs::File, io::{self, BufRead, BufReader, Lines}};

use regex::Regex;
use walkdir;

// (2) define the match result

// In this match struct we don't put any borrowed reference. You will notice that
// interfaces will be much cleaner and easier to use.
// The rule of thumb is: when you need to keep track of small chunks of data, like matches,
// copy them, so that you don't have to deal with lifetimes.
// Keep references instead for large data structures, or when you need to enforce that the
// original data is still valid. 

pub struct Match {
    file: String,
    line: usize,
    text: String,
}

// (3) test walkdir iterator, see how errors are handled
#[test]
fn test_walk_dir() {
    let wdir = walkdir::WalkDir::new("/tmp");
    for entry in wdir.into_iter() {
        // print the name of the file or an error message
    }
}


// with this tuple we will keep track of the state of the iterator inside a file
type FileIterState = (
    String, // the file name
    Lines<BufReader<File>>, // the iterator over the lines of the file
    usize, // the current line number
    String, // the current line 
    usize   // the position of the match in the line
);

// enum MyError {
//     WalkDirError(walkdir::Error),
//     FileError(io::Error),
// }

// (3) define the grep adapter for the iterator
// add anything you need implement it
pub struct GrepIter {
    // Into iter is the iterator type for walkdir
    inner: walkdir::IntoIter,
    regex: Regex,
    state: Option<FileIterState>,
}


impl GrepIter {
    pub fn new(iter: walkdir::IntoIter, pattern: &str) -> Self {
        GrepIter {
            inner: iter,
            regex: Regex::new(pattern).unwrap(),
            state: None,
        }
    }
}

impl Iterator for GrepIter {
    type Item = Result<Match, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                None => { // no file state -> next file
                    // try to get the next entry from the inner iterator 
                    match self.inner.next() {
                        Some(Ok(entry)) => {
                            // open the file and create a BufReader
                            let file = File::open(entry.path()).unwrap();
                            let reader = BufReader::new(file);
                            let mut lines = reader.lines();

                            let path = entry.path().to_str().unwrap().to_string();

                            match lines.next() {
                                Some(Ok(line)) => {
                                    // found new line
                                    self.state = Some((path, lines, 0, line, 0));
                                }
                                Some(Err(e)) => return Some(Err(e)),
                                None => {} // no lines, get to the next entry
                            }
                        }
                        Some(Err(e)) => return Some(Err(e.into())),
                        None => return None,
                    }
                }
                Some(ref mut state) => {
                    let (path, lines, line_num, line, pos) = state;
                    let mut regex = self.regex.find_iter(&line[*pos..]);
                    match regex.next() {
                        Some(m) => {
                            // found a match
                            let _m = Match {
                                file: path.clone(),
                                line: *line_num,
                                text: String::from(&line[*pos + m.start()..*pos + m.end()]),
                            };
                            // advance the position
                            *pos += m.end();
                            return Some(Ok(_m));
                        }
                        None => {
                            // no more matches in this line, go to the next line
                            match lines.next() {
                                Some(Ok(_line)) => {
                                    // found new line
                                    *line = _line;
                                    *line_num += 1;
                                    *pos = 0;
                                }
                                Some(Err(e)) => return Some(Err(e)),
                                None => self.state = None,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_grep_iter() {
    let wdir = walkdir::WalkDir::new("./src"); // find inside the project
    let grep_iter = GrepIter::new(wdir.into_iter(), "println");
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}

// (5) add grep() to IntoIter  (see the first example in EvenIter for i32)

trait Grep {
    fn grep(self, pattern: &str) -> GrepIter;
}

impl Grep for walkdir::IntoIter {
    fn grep(self, pattern: &str) -> GrepIter {
        GrepIter::new(self, pattern)
    }
}

#[test]
fn test_grep() {
    let wdir = walkdir::WalkDir::new("./src");
    let grep_iter = wdir.into_iter().grep("println");
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}


