#[cfg(test)]
mod tests;

use std::env::current_dir;
use clap::{arg, command, value_parser};
use std::process::ExitCode;

#[derive(PartialEq)]
enum DirectionToSearch {
    RightToLeft,
    LeftToRight
}

struct Args {
    ancestor_to_cd: String,
    occurence: u8,
    direction_to_search: DirectionToSearch
}

fn main() -> ExitCode {
    let matches = command!()
        .arg(arg!(<ANCESTOR> "Name (possibly substring) of ancestor you want to cd to (starting from closest to farthest)"))
        .arg(arg!([OCCURENCE] "Say <ANCESTOR> occurs multiple times in CWD. Which occurence of <ANCESTOR> do you want to go to? (starting from closest to farthest)")
             .value_parser(value_parser!(u8)))
        .arg(arg!(-r --reverse "Reverse order of search (starting from farthest to closest)"))
        .get_matches();

    let ancestor_to_cd = matches.get_one::<String>("ANCESTOR").unwrap().clone();

    let occurence_number = match matches.get_one::<u8>("OCCURENCE") {
        Some(a) => *a,
        None => 1
    };

    if occurence_number == 0 {
        println!("[ERROR] OCCURENCE must be greater than 0");
        return ExitCode::from(0);
    }

    let direction_to_search : DirectionToSearch = match matches.get_one::<bool>("reverse").unwrap() {
        true => DirectionToSearch::LeftToRight,
        false => DirectionToSearch::RightToLeft
    };

    let args = Args {ancestor_to_cd, occurence: occurence_number, direction_to_search};

    let cwd = current_dir().expect("Either Current Working Directory doesn't exist or you don't have sufficient permissions to view it.");
    match cda_core(cwd.to_str().unwrap(), args) {
        Ok(path) => {
            println!("{}", path);
            ExitCode::from(0)
        }
        Err(()) => {
            println!("[ERROR] Ancestor was not found!");
            ExitCode::from(1)
        }
    }
}

fn cda_core(cwd: &str, args : Args) -> Result<String, ()> {
    let ancestors : Vec<&str> = cwd.split("/").filter(|x| !x.is_empty()).collect();
    let ancestors_len = ancestors.len();
    let mut is_ancestor_found = false;

    let ancestors_iterator : Vec<&str> = match args.direction_to_search {
        DirectionToSearch::RightToLeft => ancestors.clone().into_iter().rev().collect(),
        DirectionToSearch::LeftToRight => ancestors.clone().into_iter().collect(),
    };

    let mut idx = 0;
    let mut curr_occurence = args.occurence;
    for possible_ancestor in ancestors_iterator {
        // // special case when there is an ancestor called `node` BUT current dir is ALSO called
        // // `node`. In such case, go to the ancestor instead of just being where you are.
        // if direction_to_search == DirectionToSearch::RightToLeft
        //     && idx == 0 {
        //         idx += 1;
        //         continue;
        //     }
        //
        if possible_ancestor.contains(&args.ancestor_to_cd) {
            if curr_occurence == 1 {
                is_ancestor_found = true;
                break;
            }
            else {
                curr_occurence -= 1;
            }
        }
        idx += 1;
    }

    let path : String = match args.direction_to_search {
        DirectionToSearch::RightToLeft => {
            "/".to_string() + &ancestors[0..ancestors_len-idx].join("/")
        }
        DirectionToSearch::LeftToRight => {
            "/".to_string() + &ancestors[0..idx+1].join("/")
        }
    };

    match is_ancestor_found {
        true => Ok(path),
        false => Err(())
    }
}
