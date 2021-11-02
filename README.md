# An implementation of a clone of the Unix command -ls


#### Contents

- This directory is organised as:
    
```
    ls
    |- src
        |- main.rs      (The main file containing the ls clone implementation)
        |- utils.rs     (The utilities required to complement the main file)
    |-target            (Auto-generated files by the Cargo)
    |- Cargo.lock       (Autogenerated file by Cargo)
    |- Cargo.toml       (Descriptive file used by Cargo)
    |- ReadMe.md        (Description of the project)
```

#### Usage

This project can be run by the command:

> cargo run -- [DIRECTORY] -[ARGUMENTS]

(Note: One-letter arguments are preceded by a single hyphen Eg., -l , -r , etc.
Longer arguments are input by 2 hyphens Eg., --time-style , --help , etc. Formatting arguments like -1, -Q, -m, etc are implemented in isolation, as in the Mac command ls)

#### Description

    ls 0.1.0

    *USAGE:*

        ls [FLAGS] [OPTIONS] [dir]

    *FLAGS:*

    1.         --atime             Sort by and display access time
    2.     -a                      Prints all entries, doesn't ignore those beginning with 
    3.     -A                      Prints almsot all entries, except those beginning with . or ..
    4.         --author            Prints tha author of each file, set by default
    5.     -m                      Prints list as comma separated values
    6.     -c                      Sort the files by the date created
    7.     -s, --size              Prints the allocated size of each file
    8.         --help              Prints help information
    9.     -h, --human-readable    Human readable format, set by default
    10.     -i, --inode             Shows the time of creation instead of last modified time
    11.     -l                      List all the details of the file
    12.     -t                      Sorts files by the last-modified time
    13.     -n, --name              Sorts files alphabetically by filename
    14.     -U                      Doesn't sort, prints in the directory order (default)
    15.     -f                      Don not sort entries
    16.     -1                      Prints file names one per line
    17.     -d, --directory         Only list directories
    18.     -Q, --quote-name        Prints file names in quotes
    19.     -R, --recursive         Print files recursively
    20.     -r, --reverse           Print files in the reverse order - best used with a sort option
    21.     -S                      Sorts files by file size
    22.     -V, --version           Prints version information

    *OPTIONS:*
            --sort <sort>                Takes as input the criteria to sort based on [default: create]
            --time-style <time-style>    Formats the time output, options are Rust time formats like %Y-%m-%d %H:%M:%S
                                        (default), %b %e %H:%M , %a %b %e %T %Y and rearranged variants [default: %Y-%m-%d
                                        %H:%M:%S]

    *ARGS:*
            <dir>    Takes the directory to be listed from cmd line [default: .]


#### Dependencies:

This code has been tested in an environment, with:
    Cargo       v 1.55.0
    chrono      = "0.4.19"
    filetime    = "0.2.14"
    humansize   = "1.1.1"
    libc        = "0.2.95"
    structopt   = "0.3.21"
    users       = "0.11.0"


#### Difficulties:

- While implementing two arguments (-r and -i, for recursive listing within directories and inode printing), I ran into errors.

- For the -r argument, the compiler throws an error while checking if a file is a directory (instead of merely returning False)

- For the -i argument, the compiler cannot find the method that Rust implements to extract the inode of a particular file.

- After thoroughly searching for a fix / workaround, to no end - I've included the pseudocode for these arguments as comments, where they should be implemented. 

#### References:

1. For the available arguments for -ls: The ls man page, replicated here: https://linux.die.net/man/1/ls

2. For an understanding of the implementation techniques in Rust: https://github.com/willdoescode/nat , https://github.com/ogham/exa

3. For error resolution and general Rust help: https://www.rust-lang.org/community 


