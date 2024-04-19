use crate::cda_core;
use crate::Args;
use crate::DirectionToSearch::*;

#[test]

// Tests where cda_core returns Ok(_)
fn ok1() {
    let cwd = "/apple/banana/pineapple/mango";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 1,
        direction_to_search: RightToLeft
    };
    assert_eq!(cda_core(cwd, args), Ok("/apple/banana".to_string()))
}

#[test]
fn ok2() {
    let cwd = "/apple/banana/banana/mango";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 1,
        direction_to_search: RightToLeft
    };
    assert_eq!(cda_core(cwd, args), Ok("/apple/banana/banana".to_string()))
}

#[test]
fn ok3() {
    let cwd = "/apple/banana/banana/mango";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 2,
        direction_to_search: RightToLeft
    };
    assert_eq!(cda_core(cwd, args), Ok("/apple/banana".to_string()))
}


#[test]
fn ok4() {
    let cwd = "/apple/banana/mango";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 1,
        direction_to_search: LeftToRight
    };
    assert_eq!(cda_core(cwd, args), Ok("/apple/banana".to_string()))
}

#[test]
fn ok5() {
    let cwd = "/apple/banana/mango/banana";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 1,
        direction_to_search: LeftToRight
    };
    assert_eq!(cda_core(cwd, args), Ok("/apple/banana".to_string()))
}

#[test]
fn ok6() {
    let cwd = "/apple/banana/mango/banana/banana";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 2,
        direction_to_search: LeftToRight
    };
    assert_eq!(cda_core(cwd, args), Ok("/apple/banana/mango/banana".to_string()))
}

// Tests where cda_core returns Err(())
#[test]
fn err1() {
    let cwd = "/apple/banana/mango";
    let args = Args {
        ancestor_to_cd: "banana".to_string(),
        occurence: 2,
        direction_to_search: RightToLeft
    };
    assert_eq!(cda_core(cwd, args), Err(()))
}

#[test]
fn err2() {
    let cwd = "/apple/banana/mango";
    let args = Args {
        ancestor_to_cd: "pineapple".to_string(),
        occurence: 1,
        direction_to_search: RightToLeft
    };
    assert_eq!(cda_core(cwd, args), Err(()))
}

#[test]
fn err3() {
    let cwd = "/apple/banana/mango";
    let args = Args {
        ancestor_to_cd: "mango".to_string(),
        occurence: 1,
        direction_to_search: RightToLeft
    };
    assert_eq!(cda_core(cwd, args), Err(()))
}


#[test]
fn err4() {
    let cwd = "/apple/banana/mango";
    let args = Args {
        ancestor_to_cd: "mango".to_string(),
        occurence: 1,
        direction_to_search: LeftToRight
    };
    assert_eq!(cda_core(cwd, args), Err(()))
}
