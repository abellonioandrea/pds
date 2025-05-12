// WARNING: 
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples 

use std::io;
use std::io::BufRead;

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        let lines = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
        LineEditor { lines }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let file = std::fs::File::open(file_name)?;
        let reader = io::BufReader::new(file);
        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(LineEditor { lines })
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        self.lines[line].replace_range(start..end, subst);
    }
}


// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped 

// sol: text is a reference to the matched text, it must live at least as long as the Match
pub struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern

fn find_matches<'a>(lines: &Vec<&'a str>, pattern: &str) -> Vec<Match<'a>> {
    // sol: the lifetime 'a is necessary since the result is a Match and it contains a 
    // reference to the matched lines
    // pay attention: the lifetime must be the same as the **strings** not as the enclosing vector

    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}

// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
pub struct FindReplace<'a> {
    // commenting out lines and pattern since we don't need them after the constructor
    // lines: Vec<&'a str>,
    // pattern: String,
    matches: Vec<Match<'a>>,
}

// sol: there is only one lifetime 'a: the matches are references to the lines, and they should live
// at least as long as the find replace object

impl<'a> FindReplace<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        let matches = find_matches(&lines, pattern);
        FindReplace {
            // lines: lines,
            // pattern: pattern.to_string(),
            matches: matches,
        }
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match> {
        &self.matches
    }

    // apply a function to all matches, if fun returns true, the match is kept
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {

        // you can try to make an iterative solution with the for loop

        // RUST 100% idiomatic solution
        self.matches.retain_mut(|m| {
            fun(m)
        });

        // without retain how to solve this functionally with iterators?

        // (1) use map, to a) apply the function and b) keep the match together with the result
        // (2) use filter to skip the matches that are not kept
        // (3) use map to get the match alone back
        // (4) collect the result into a new vector

        // we had also to add Clone to the Match struct in order to be able to clone it
        // otherwise there is a problem with the borrow checker 

        // self.matches = self.matches.iter_mut()
        //     .map(|m| { (m, fun(m)) })
        //     .filter(|(m, keep)| *keep)
        //     .map(|(m, _)| m.clone()) // without clone the borrow checker complains
        //     .collect();
    }
}


//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]

// This is not a real test, but just an example of how to use the FindReplace with the editor
// we supply it as a test, so that we can run it and see if it works without writing a main
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindReplace::new(lines, "ll");

    // find all the matches and accept them 
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    // now let's replace the matches
    // why this loop won't work?

    // ANSWER: the matches have a reference to the lines, which are borrowed and belong to the editor
    // Therefore we cannot get a mutable reference to the editor. 
    // We have to get rid of the references to the lines before modifying the editor

    // for m: Match in finder.matches() {
    //     editor.replace(/* add match */);
    // }    

    // alternate method: why this one works? 
    // we create a new vector of match tuples without references to the lines
    // so that in the second loop we can modify the editor not having any borrowed references

    let mut subs = Vec::new();
    for m in finder.matches() {
        subs.push((m.line, m.start, m.end, m.repl.clone().unwrap()));
    }

    // from this point there are no references to the lines anymore

    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, subst.as_str());
    }

    // note: if we'd use the fider here or after nothing would work and any modification would still
    // be impossible

}


// (6) sometimes it's very expensive to find all the matches at once before applying 
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
pub struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

pub struct LazyFinder<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

// implement the Lazy find_next as function, so that we can re-use it in the iterator
fn find_next<'a>(lines: &Vec<&'a str>, pos: FinderPos, pattern: &str) -> Option<Match<'a>> {

    // it will return None if there are no more matches
    let start_line = pos.line;
    let mut offset = pos.offset;

    for i in start_line..lines.len() {
        let line = &lines[i][offset..];
        let re = regex::Regex::new(pattern).unwrap();

        if let Some(mat) = re.find(line) {
            return Some(Match {
                line: i,
                start: offset + mat.start(),
                end: offset + mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }

        offset = 0; // reset offset for the next line
    }
    None
}

impl<'a> LazyFinder<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        LazyFinder {
            lines: lines,
            pattern: pattern.to_string(),
            pos: Some(FinderPos { line: 0, offset: 0 }),
        }
    }

    pub fn next(&mut self) -> Option<Match> {
        match self.pos {
            Some(..) => match find_next(&self.lines, self.pos?, &self.pattern) {
                Some(m) => {
                    self.pos = Some(FinderPos {
                        line: m.line,
                        offset: m.end,
                    });
                    Some(m)
                }
                None => None,
            },

            None => None,
        }
    }
}


// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text and full of matches.\nno match\none match and the end: ll";
    let editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches... 
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator

pub struct FindIter<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}
// 
impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        FindIter {
            lines: lines,
            pattern: pattern.to_string(),
            pos: Some(FinderPos { line: 0, offset: 0 }),
        }
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Match<'a>; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        match self.pos {
            Some(..) => match find_next(&self.lines, self.pos?, &self.pattern) {
                Some(m) => {
                    self.pos = Some(FinderPos {
                        line: m.line,
                        offset: m.end,
                    });
                    Some(m)
                }
                None => None,
            },

            None => None,
        }
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let finder = FindIter::new(lines, "ll");

    // find all the matches and accept them
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}

