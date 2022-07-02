use std::path::PathBuf;
use clap::{Parser, Subcommand};
use text_io::read;
use duct::cmd;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[clap(value_parser)]
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
    /// create a new branch from a ticket number
    Branch {
    }
}


fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    //
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Branch {}) => {
            branch_new();
        }
        None => {}
    }
}


//function gittool-branch-create -a ticket -d "create a branch with the correct work format"
//    git stash save (string join ' ' "GitTool - stashed to create:" $branch_name)
//    git checkout master
//    git pull
//    git checkout -b $branch_name
//
//    set -l cmd  "(work/log-ticket \"$ticket\" \"$raw_description\")"
//    emacsclient -q --eval $cmd
//end
fn branch_new() {
    println!("Ticket number:");
    let mut ticket_no : String = read!("{}\n");
    ticket_no = ticket_no.trim().replace("#", "");

    println!("Enter a short description: ");
    let mut desc : String = read!("{}\n");
    desc = desc.trim().to_lowercase().replace(" ", "_");

    let branch_name : String = ["chris/", &ticket_no, "-", &desc].join("");
    println!("{}", branch_name);

    if cmd!("git", "stash", "save", format!("GitTool - stashed to create: {}", branch_name)).read().is_err() {
        println!("error error");
    }
}






//function gittool-branch-clean --d "clean old git branches"
//    git branch | grep -v master | grep -v save
//    read -l -P "Branches to be deleted. Proceed? (y/n) > " proceed
//
//    if test $proceed = y
//        git checkout master
//        git branch | grep -v master | grep -v save | xargs git branch -D
//    else
//        echo "operation cancelled"
//    end
//end
//
//
//function gittool-branch-save -a branch --d "add __save onto the branch name"
//    set -l branches (git branch)
//    set -l save_name (string join '' $branch '__save')
//
//    if string match -q -r (string join '' $branch '__save$') $branches
//        git branch -m $save_name $branch
//    else
//        git branch -m $branch $save_name
//    end
//end
//
//
//
//function gittool-force-push
//  set -l branch_name (git branch --show-current)
//  if test $branch_name = "master"
//    echo 'no'
//    return
//  end
//  if test $branch_name = "release"
//    echo 'no'
//    return
//  end
//
//  git push -f origin $branch_name
//end
//
//function gittool-rebase -d "rebase current branch against master"
//    set -l branch_name (git branch --show-current)
//    set -l stash_name (string join ' ' "stashed for rebase" $branch_name)
//    #echo $stash_name
//    git stash save $stash_name
//    git checkout master
//    git pull origin master
//    git checkout $branch_name
//    git rebase master
//    # FIXME only do this if something was stashed, maybe with git status -s
//    git stash pop
//    #bdb
//end
//
//function gittool-last-branch -d "switch to the last non-master branch used"
//    echo "write me"
//end
//
//function gittool-commit-amend -d "commit the staged changes to the current commit"
//    git commit --amend
//end
