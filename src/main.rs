use std::{env::{self, current_dir}, path::Path};
use clap::{arg, command, value_parser, ArgAction, Command};
use std::process::ExitCode;

enum DirectionToSearch {
    RightToLeft,
    LeftToRight
}

fn main() -> ExitCode {
    let matches = command!()
        .arg(arg!(<ANCESTOR> "Name (possibly substring) of ancestor you want to cd to (starting from closest to farthest)"))
        .arg(arg!([OCCURENCE] "Say <ANCESTOR> occurs multiple times in CWD. Which occurence of <ANCESTOR> do you want to go to? (starting from closest to farthest)")
             .value_parser(value_parser!(u8)))
        .arg(arg!(-r --reverse "Reverse order of search (starting from farthest to closest)"))
        .get_matches();

    let ancestor_to_cd = matches.get_one::<String>("ANCESTOR").unwrap();

    let mut occurence_number : u8;
    match matches.get_one::<u8>("OCCURENCE") {
        Some(a) => {
            occurence_number = *a;
        }
        None => {
            occurence_number = 1;
        }
    }

    let direction_to_search : DirectionToSearch = match matches.get_one::<bool>("reverse").unwrap() {
        true => DirectionToSearch::LeftToRight,
        false => DirectionToSearch::RightToLeft
    };


    let cwd = current_dir().expect("Either Current Working Directory doesn't exist or you don't have sufficient permissions to view it.");
    let mut ancestors : Vec<&str> = cwd.to_str().unwrap().split("/").filter(|x| !x.is_empty()).collect();
    let ancestors_len = ancestors.len();
    let mut idx = 0;
    let mut is_ancestor_found = false;

    let ancestors_iterator : Vec<&str> = match direction_to_search {
        DirectionToSearch::RightToLeft => ancestors.clone().into_iter().rev().collect(),
        DirectionToSearch::LeftToRight => ancestors.clone().into_iter().collect(),
    };
    for possible_ancestor in ancestors_iterator {
        if possible_ancestor.contains(ancestor_to_cd) {
            if occurence_number == 1 {
                is_ancestor_found = true;
                break;
            }
            else {
                occurence_number -= 1;
            }
        }
        idx += 1;
    }

    let path : String = match direction_to_search {
        DirectionToSearch::RightToLeft => {
            "/".to_string() + &ancestors[0..ancestors_len-idx].join("/")
        }
        DirectionToSearch::LeftToRight => {
            "/".to_string() + &ancestors[0..idx+1].join("/")
        }
    };

    if is_ancestor_found == false {
        println!("Ancestor was not found!");
        return ExitCode::from(1)
    }

    println!("{}", path);
    return ExitCode::from(0);
}
