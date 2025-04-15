// to warm up: the define step by step an adapter for filtering even numbers

pub mod simple_even_iter {
    use std::vec::IntoIter;

    // (1) let start with a simple iterator adapter for just one type, "i32"
    // see the adapter pattern example in the pdf "Adapter Pattern..."
    struct EvenIter<I>
    where
        I: Iterator<Item=i32>,
    {
        inner: I, // hint: it's a generic type... here we don't care about bounds yet
    }

    impl<I> EvenIter<I>
    where
        I: Iterator<Item=i32>,
    {
        fn new(iter: I) -> Self {
            EvenIter { inner: iter }
        }
    }

    impl<I: Iterator> Iterator for EvenIter<I>
    where
        I: Iterator<Item=i32>, // here we need to define the bounds for the generic type
    // T it must be an iterator over i32
    {
        type Item = i32; // <== it will work just for i32

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(elem) = self.inner.next() {
                if elem % 2 == 0 {
                    return Some(elem);
                }
            }
            None
        }
    }

    // if EvenIter works the test will compile and pass
    #[test]
    fn test_simple_even_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // why iter() does not work here?
        let it: EvenIter<IntoIter<i32>> = EvenIter::new(v.into_iter());
        for i in it {
            println!("i: {}", i);
        }
    }

    // (2) now let's add the adapter to all Iterator<Item=i32> (advanced)
    trait AddEvenIter: Iterator<Item=i32> + Sized
    where
        Self: Sized,
    {
        // add even() to anyone implementing this trait
        // usage: v.into_iter().even() ....
        fn even(self) -> EvenIter<Self> {
            EvenIter::new(self)
        }
    }

    // (3) add here the generic implementation, you can supply it for all the iterators
    // impl .... ?
    impl<T> AddEvenIter for T
    where
        T: Iterator<Item=i32>,
    {
        fn even(self) -> EvenIter<Self> {
            EvenIter::new(self)
        }
    }

    #[test]
    fn test_adapter() {
        let v = vec![1, 2, 3, 4, 5];
        for i in v.into_iter().even() {
            println!("{}", i);
        }
    }
}

pub mod even_iter {
    // (4) more advanced: implement for all integer types
    // => install the external crate "num" to have some Traits identifying all number types
    use num;

    // the generic parameters I and U are already defined for you in the struct definition
    // (5) write in a comment in plain english the meaning of the generic parameters
    // and their constraints
    struct EvenIter<I, U>
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
            while let Some(elem) = self.iter.next() {
                if (elem.is_even()) {
                    return Some(elem);
                }
            }
            None
        }
    }

    // (6) once implemented, the test will compile and pass
    #[test]
    fn test_even_iter() {
        let mut v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let mut it = EvenIter {
            iter: v.into_iter(),
        };
        for i in it {
            println!("i: {}", i);
        }
    }
}

// finally let's implement the grep command
// (1) install the "walkdir" crate for walking over directories using an iterator
// install also the "regex" crate for regular expressions

use walkdir;

// (2) define the match result
struct Match {
    file: String,
    line: usize,
    text: String,
}

// (3) test walkdir iterator, see how errors are handled
#[test]
fn test_walk_dir() {
    let wdir = walkdir::WalkDir::new("/tmp");
    for entry in wdir.into_iter() {
        if (entry.is_err()) {
            println!("Error: {}", entry.err().unwrap().to_string());
        } else {
            println!("File: {}", entry.unwrap().path().display().to_string());
        }
    }
}

// (3) define the grep adapter for the iterator
// add anything you need implement it
struct GrepIter {
    inner: walkdir::IntoIter,
}

impl GrepIter {
    fn new(iter: walkdir::IntoIter) -> Self {
        GrepIter { inner: iter }
    }
}

impl Iterator for GrepIter {
    type Item = Result<Match, walkdir::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(entry) = self.inner.next() {
            return match entry {
                Ok(dir_entry) => {
                    let file_path = dir_entry.path().display().to_string();
                    let line = dir_entry.depth();
                    let text = String::new(); // Placeholder, as text extraction is not implemented
                    Some(Ok(Match {
                        file: file_path,
                        line,
                        text,
                    }))
                }
                Err(err) => {
                    Some(Err(err))
                }
            };
        }
        None
    }
}

#[test]
fn test_grep_iter() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = GrepIter::new(wdir.into_iter());
    for entry in grep_iter {
        match entry {
            Ok(m) => {
                println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

// (5) add grep() to IntoIter  (see the first example in EvenIter for i32)

trait Grep {
    fn grep(self) -> GrepIter;
}

impl Grep for walkdir::IntoIter {
    fn grep(self) -> GrepIter {
        GrepIter::new(self)
    }
}

#[test]
fn test_grep() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = wdir.into_iter().grep();
    for entry in grep_iter {
        match entry {
            Ok(m) => {
                println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}