// Benjamin Altermatt CS3304-Spring2023 Project 1: Rust Sentence Diagrammer

use std::env;   // this is important so we can access the arguments
use std::fs;    // access to file system api

// we need to start with getting the file to open
fn main() {
    // command line arguments are collected here
    let argv : Vec<String> = env::args().collect();

    // let's make sure we have at least one argument
    if argv.len() < 2 {
        panic!("No filename was passed! Sorry gamer!");
    }

    // now let's try to open a file with our string
    let file_string: String = match fs::read_to_string(&argv[1]) {
        Ok(file) => file,
        Err(_) => {
            panic!("Invalid filename passed.");
        }
    };

    // break up the file into lines and handle the lines
    for line in file_string.lines() {
        // We'll start handling a sentence here, but it will be easier to work with a 
        // vector of words
        let lexemes : Vec<&str> = line.trim().split(&['\t', ' ']).collect();


        // let's go through and check as quickly as possible for a lexical error
        let mut lex_pass : bool = true;
        for ind in 0..lexemes.len() {
            // we'll check if it is any of our base words
            if noun(&lexemes, ind as u32).0 + adv(&lexemes, ind as u32).0 + verb(&lexemes, ind as u32).0 + adj(&lexemes, ind as u32).0 + prep(&lexemes, ind as u32).0 < 1 {
                lex_pass = false;
                break;
            }
        }
        if !lex_pass { 
            println!("Input has invalid tokens.");
            continue;
        }

        // now, we have to go through and break down this array of lexemes
        // by parsing it into smaller portions as defined by the use of complicated functions

        // if we had any failures, we need to properly report them
        let (size, formatted) = sentence(&lexemes, 0);
        if size == 0 {
            println!("Input is not a sentence.");
        }
        else {
            println!("{formatted}");
        }
    }
}

// Everything past here is a function specifying the parsing and
// validation of a specific construct we are trying to parse.
// All of them take in a vector of lexemes representing the ordered
// individual words in the sentence, and an index representing
// the starting index of the words being parsed in this particular
// function within the vector. These functions return a "size", allowing
// the function which called them to properly adjust the index within the
// lexeme vector it is processing. They also return Strings, which can
// be added together within the calling function to combine and create
// more complicated gramatical constructs, recursively. I chose to use
// these immutable strings and return between functions because altering a single
// string could lead to problems if I needed to back track. Some of the functions
// will be sparsely commented. This is because many of these have exceedingly similar
// functions, and I had to break them up into individual functions due
// to the project specification. I hate doing multiline comments in rust.
// It is exhausting.


// these are the basic ones

// checks and parenthesizes verb tokens
fn verb(lexs: &Vec<&str>, s_ind : u32) -> (u32, String) {
    // I make a return string here which will be built as we progress
    let mut ret_str: String = String::new();

    // check if we are out of bounds in lexemes for the sentence
    if lexs.len() as u32 <= s_ind {
        return (0, ret_str); // 0 means this is empty
    }

    // get the specific lex out of the vector
    let lex : &str = lexs[s_ind as usize]; 

    // specific values a verb can have. would have maybe made a cleaner
    // dictionary or something to do this if I had more time.
    if lex == "lifted" || lex == "saw" || lex == "found" {
        ret_str.push_str(lex); // put the word in our return string
        return (1, ret_str); // size of single word, and our string
    }
    return (0, ret_str);
}

// checks and parenthesizes adverb
fn adv(lexs: &Vec<&str>, s_ind : u32) -> (u32, String) {
    let mut ret_str: String = String::new();

    if lexs.len() as u32 <= s_ind {
        return (0, ret_str);
    }

    let lex : &str = lexs[s_ind as usize];

    if lex == "quickly" || lex == "carefully" || lex == "brilliantly" {
        ret_str.push_str(lex);
        return (1, ret_str);
    }
    return (0, ret_str);
}

// checks and parenthesizes adjective
fn adj(lexs: &Vec<&str>, s_ind : u32) -> (u32, String) {
    let mut ret_str: String = String::new();

    if lexs.len() as u32 <= s_ind {
        return (0, ret_str);
    }

    let lex : &str = lexs[s_ind as usize];

    if lex == "green" || lex == "lean" || lex == "mean" {
        ret_str.push_str(lex);
        return (1, ret_str);
    }
    return (0, ret_str);
}

// checks and parenthesizes noun
fn noun(lexs: &Vec<&str>, s_ind : u32) -> (u32, String) {
    let mut ret_str: String = String::new();

    if lexs.len() as u32 <= s_ind {
        return (0, ret_str);
    }

    let lex : &str = lexs[s_ind as usize];

    if lex == "cow" || lex == "alice" || lex == "book" {
        ret_str.push_str(lex);
        return (1, ret_str);
    }
    return (0, ret_str);
}

// checks and parenthesizes preposition
fn prep(lexs: &Vec<&str>, s_ind : u32) -> (u32, String) {
    let mut ret_str: String = String::new();

    if lexs.len() as u32 <= s_ind {
        return (0, ret_str);
    }

    let lex : &str = lexs[s_ind as usize];

    if lex == "of" || lex == "at" || lex == "with" {
        ret_str.push_str(lex);
        return (1, ret_str);
    }
    return (0, ret_str);
}

// Now we get to the second level of complexity, combining our single tokens into larger
// grammatical structures. Many of these functions follow similar strategy as well. However
// it would have been much harder (but maybe better for much more complicated grammars)
// to break down these and put them in a larger function.

// checks and parenthesizes noun phrase
// notice, we have the same function header
fn noun_phrase(lexs: &Vec<&str>, s_ind: u32) -> (u32, String) {
    // I created a mutable reference to keep track of size because
    // it is the sum of the children sizes now and will be altered
    // as we build.
    let mut cur_size : u32 = 0;
    // everything more complex than a token has a parenthesis
    let mut ret_str : String = String::from("("); 

    // these blocks are used commonly and mayube could have been made
    // into a single function and been less repetitive. 
    // essentially, we try to see if the current lex and future lexs
    // fit into the construct we're looking for
    let (size_change, child_str) = adj_phrase(lexs, s_ind + cur_size);
    if size_change != 0 { // if the size isn't 0, they do!
        ret_str.push_str(&child_str); // add the string of the children to this one
    }
    cur_size += size_change; // increment the size, to keep track of the index we and the parent can use

    // same sort of deal, except this one works differently as nouns are necessary, not optional
    let (size_change, child_str) = noun(lexs, s_ind + cur_size);
    if size_change == 0 {return (0, ret_str);}
    ret_str.push_str(&child_str);
    cur_size += size_change;

    let (size_change, child_str) = prep_phrase(lexs, s_ind + cur_size);
    if size_change != 0 {
        // ret_str.push(' ');
        ret_str.push_str(&child_str);
    }
    cur_size += size_change;
    ret_str.push(')'); // close the parenthesis

    return (cur_size, ret_str); // return our size and string
}

// checks and parenthesizes adjective phrase
fn adj_phrase(lexs: &Vec<&str>, s_ind: u32) -> (u32, String) {
    let mut cur_size : u32 = 0;
    let mut ret_str : String = String::from("(");

    let (size_change, child_str) = adj(lexs, s_ind + cur_size);
    if size_change != 0 {
        cur_size += size_change;
        ret_str.push_str(&child_str);

        let (size_change, child_str) = adj_phrase(lexs, s_ind + cur_size);
        if size_change > 0 {ret_str.push_str(&child_str);}

        ret_str.push(')');
        return (cur_size + size_change, ret_str);
    }
    return (0, ret_str);
}

// checks and parenthesizes verb phrase
fn verb_phrase(lexs: &Vec<&str>, s_ind:u32) -> (u32, String) {
    let mut cur_size: u32 = 0;
    let mut ret_str : String = String::from("(");

    let (size_change, child_str) = verb(lexs, s_ind + cur_size);
    if size_change != 0 {
        cur_size += size_change;
        ret_str.push_str(&child_str);

        let (size_change, child_str) = adv(lexs, s_ind + cur_size);
        if size_change != 0 {
            ret_str.push(' ');
            ret_str.push_str(&child_str);
        }

        ret_str.push(')');
        return (cur_size + size_change, ret_str);
    }
    return (0, ret_str);
}

// checks and parenthesizes prepositional phrase
fn prep_phrase(lexs : &Vec<&str>, s_ind:u32) -> (u32, String) {
    let mut ret_str : String = String::from("(");
    let mut cur_size : u32 = 0;

    let (size_change, child_str) = prep(lexs, s_ind + cur_size);
    
    if size_change == 0 { return (0, ret_str);}
    cur_size += size_change;
    ret_str.push_str(&child_str);
    // ret_str.push(' ');

    let (size_change, child_str) = noun_phrase(lexs, s_ind + cur_size);
    if size_change == 0 { return (0, ret_str);}

    ret_str.push_str(&child_str);
    ret_str.push(')');
    return (cur_size + size_change, ret_str);
}

// checks and parenthesizes subject
// this one feels a little silly, but it does parenthesize so that's good
fn subject(lexs: &Vec<&str>, s_ind: u32) -> (u32, String) {
    let mut ret_str : String = String::from("(");
    let (size, string) = noun_phrase(lexs, s_ind);
    ret_str.push_str(&string);
    ret_str.push(')');

    return (size, ret_str);
}

// this basically does the same thing as object and was included for scope
fn object(lexs: &Vec<&str>, s_ind: u32) -> (u32, String) {
    let mut ret_str : String = String::from("(");
    let (size, string) = noun_phrase(lexs, s_ind);
    ret_str.push_str(&string);
    ret_str.push(')');

    return (size, ret_str);
}

// This checks if the passed lexemes are syntactically and lexigraphically a proper
// sentence and builds a parenthesized string of them if they are.
// It operates similarly to the previous functions.
fn sentence(lexs: &Vec<&str>, s_ind: u32) -> (u32, String) {
    let mut ret_str : String = String::from("(");
    let mut cur_size : u32 = 0;

    let (size_change, child_str) = subject(lexs, s_ind + cur_size);
    if size_change == 0 {return (0, ret_str);}
    cur_size += size_change;
    ret_str.push_str(&child_str);
    ret_str.push(' ');

    let(size_change, child_str) = verb_phrase(lexs, s_ind + cur_size);
    if size_change == 0 {return (0, ret_str);}
    cur_size += size_change;
    ret_str.push_str(&child_str);
    ret_str.push(' ');

    let(size_change, child_str) = object(lexs, s_ind + cur_size);
    if size_change == 0 {return (0, ret_str);}
    cur_size += size_change;
    ret_str.push_str(&child_str);
    ret_str.push(')');

    return (cur_size, ret_str);
}

