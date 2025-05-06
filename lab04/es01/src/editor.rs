// WARNING:
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples

use std::fs::File;
use std::io;
use std::io::Read;

// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        let mut le = LineEditor { lines: vec![] };
        for split in s.split("\n") {
            le.lines.push(split.to_string())
        }
        le
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, io::Error> {
        let mut file = File::open(file_name)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut le: LineEditor = LineEditor { lines: vec![] };
        for line in contents.lines() {
            le.lines.push(line.to_string());
        }
        Ok(le)
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        self.lines[line].replace_range(start..=end, subst);
    }
}

// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_matches<'a>(lines: &Vec<&'a str>, pattern: &str) -> Vec<Match<'a>> {
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
struct FindReplace<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        let matches = find_matches(&lines, pattern);
        FindReplace {
            lines,
            pattern: pattern.to_string(),
            matches,
        }
    }

    // return all the matches
    pub fn matches(&mut self) -> &Vec<Match> {
        &self.matches
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        self.matches.retain_mut(|x| { fun(x) })
    }
}

//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
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
    /*
    for m in finder.matches() {
        editor.replace();
    }
     */

    // alternate method: why this one works?

    let mut subs = Vec::new();
    for m in finder.matches() {
        if (!m.repl.is_none()) {
            subs.push((
                m.line,
                m.start,
                m.end,
                m.repl.as_deref().unwrap_or("").to_string(),
            ));
        }
    }

    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, &subst);
    }
}

// (6) sometimes it's very expensive to find all the matches at once before applying
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'b> LazyFinder<'b> {
    pub fn new(lines: Vec<&'b str>, pattern: &str) -> Self {
        LazyFinder {
            lines,
            pattern: pattern.to_string(),
            pos: Some(FinderPos { line: 0, offset: 0 }),
        }
    }

    pub fn next(&mut self) -> Option<Match> {
        // remember:
        // return None if there are no more matches
        // return Some(Match) if there is a match
        // each time save the position of the match for the next call
        let re = regex::Regex::new(&self.pattern).unwrap();
        for i in self.pos?.line..self.lines.len() {
            for mat in re.find_at(self.lines[i], self.pos?.offset) {
                self.pos.as_mut()?.offset = mat.end();
                self.pos.as_mut()?.line = i;
                return Some(Match {
                    line: i,
                    start: mat.start(),
                    end: mat.end(),
                    text: &self.lines[i][mat.start()..mat.end()],
                    repl: None,
                });
            }
        }
        None
    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}

// (8) now you have everything you need to implement the real Iterator

struct FindIter<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        FindIter {
            lines: lines,
            pattern: pattern.to_string(),
            pos: Some(FinderPos { line: 0, offset: 0 }),
        }
    }
}

impl<'b> Iterator for FindIter<'b> {
    type Item = Match<'b>; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        let re = regex::Regex::new(&self.pattern).unwrap();
        for i in self.pos?.line..self.lines.len() {
            for mat in re.find_at(self.lines[i], self.pos?.offset) {
                self.pos.as_mut()?.offset = mat.end();
                self.pos.as_mut()?.line = i;
                return Some(Match {
                    line: i,
                    start: mat.start(),
                    end: mat.end(),
                    text: &self.lines[i][mat.start()..mat.end()],
                    repl: None,
                });
            }
        }
        None
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
