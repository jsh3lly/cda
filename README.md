# Why do you need this?
Do you see yourself doing `cd ..` way too many times?
Or even worse, doing `cd ../../../../../../../..` and making a guess that the directory you end up at is "close enough"?
If yes, then this tool is for you.

And before you say "just organize your directories in a more meaningful way!", sometimes I end up working with multiple parallel directory trees (which loosely mirror each other), for whatever reason.

# What does it do?
`cda <ANCESTOR>` `cd`s you to the closest ancestor.

Eg:
`CWD: /home/jshelly/my-repos/my-project/workspace1/module1/component3/feature/feature.rs`
```
$ cda my-project
$ pwd
/home/jshelly/my-repos/my-project
```

Similarly, I could've gone with any substring of "my-project" as well!
```
$ cda proj
$ pwd
/home/jshelly/my-repos/my-project
```

If multiple ancestors exist with the same name, then it takes you to the closest one. Otherwise, you can specify which occurrence of ancestor you wanna go to.

Eg:
`CWD: /home/jshelly/playground/dir/playground/dir`
```
$ cda playground 2 #Second arg is occurence. By default, it is 1.
$ pwd
/home/jshelly/playground
```
NOTE: The CWD is not considered its own ancestor.

By default, the search happens from closest to furthest ancestor (right to left). You can reverse that by using the `-r` flag.


# Installation
1. `cargo install cda`
2. Add this to the end of your shell config (.zshrc, .bashrc, etc)
```
cda() {
    path_or_error=$(cda-bin "$@")
    if [ $? -eq 0 ]; then   # Path
        cd $path_or_error
    else                    # Err
        echo $path_or_error
    fi
}
```
