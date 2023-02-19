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
        // vector of words so we can use slices
        let lexemes : Vec<&str> = line.split(&['\t', ' ']).collect();
        let mut pthized : String = String::new();

        //penis

        // let's go through and check as quickly as possible for a lexical error
        let mut lex_pass : bool = true;
        let subs_p = &mut pthized;
        for lex in lexemes {
            // we'll check if it is any of our base words
            if !(noun(subs_p, lex) | adv(subs_p, lex) | verb(subs_p, lex) | adj(subs_p, lex) | prep(subs_p, lex)) {
                lex_pass = false;
                break;
            }
        }
        if !lex_pass { 
            println!("Input has invalid tokens.");
            continue;
        }

        pthized = String::new();

        // now, we have to go through and break down this array of lexemes
        // by parsing it into smaller portions as defined by the use of complicated functions

        // if we had any failures, we need to properly report them
        if sentence(&mut pthized, lexemes) {
            println!("Input is not a sentence.");
        }
        else {
            println!("{pthized}");
        }
    }
}
// penisbutt
// This checks if the passed lexemes are syntactically and lexigraphically a proper
// sentence and builds a parenthesized string of them if they are.
fn sentence(pthized: &mut String, lexs: Vec<&str>) -> bool {
    // a sentence requires a subject, verb phrase, and an object
    return adj_phrase(pthized, &lexs[..]);
}

// checks and parenthesizes subject
fn subject(pthized: &mut String, lexs: &[&str]) -> bool {
    return true;
}

// checks and parenthesizes noun phrase
fn noun_phrase(pthized: &mut String, lexs: &[&str]) -> bool {
    return true;
}

// checks and parenthesizes adjective phrase
fn adj_phrase(pthized: &mut String, lexs: &[&str]) -> bool {
    // this consists of an adjective or an adjective plus an adjective phrase
    // we're gonna approach this greedily as I assume is supposed to be done,
    // and attempt to make phrases as large as possible
    pthized.push('(');
    if lexs.len() > 0 && adj(pthized, lexs[0]) {
        adj_phrase(pthized, &lexs[1..]);
        pthized.push(')');
        return true;
    }
    pthized.pop();
    return false;
}

// checks and parenthesizes adjective
fn adj(pthized: &mut String, lex:&str) -> bool {
    if lex == "green" || lex == "lean" || lex == "mean" {
        pthized.push_str(lex);
        return true;
    }
    return false;
}

// checks and parenthesizes noun
fn noun(pthized: &mut String, lex: &str) -> bool {
    if lex == "cow" || lex == "alice" || lex == "book" {
        pthized.push_str(lex);
        return true;
    }
    return false;
}

// checks and parenthesizes prepositional phrase
fn prep_phrase(pthized: &mut String, lexs: &[&str]) -> bool {
    return true;
}

// checks and parenthesizes preposition
fn prep(pthized: &mut String, lex: &str) -> bool {
    if lex == "of" || lex == "at" || lex == "with" {
        pthized.push_str(lex);
        return true;
    }
    return false;
}

// checks and parenthesizes verb phrase
fn verb_phrase(pthized: &mut String, lexs: &[&str]) -> bool {
    pthized.push('(');
    if lexs.len() > 0 && verb(pthized, &lexs[0]) {
        pthized.push(' ');

    }
    else {
        pthized.pop();

    }
}

// checks and parenthesizes verb
fn verb(pthized: &mut String, lex: &str) -> bool {
    if lex == "lifted" || lex == "saw" || lex == "found" {
        pthized.push_str(lex);
        return true;
    }
    return false;
}

// checks and parenthesizes adverb
fn adv(pthized: &mut String, lex: &str) -> bool {
    if lex == "quickly" || lex == "carefully" || lex == "brilliantly" {
        pthized.push_str(lex);
        return true;
    }
    return false;
}