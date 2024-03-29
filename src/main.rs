use clap::{Parser, Subcommand};
use duct::cmd;
use git2::{BranchType, Repository};
use std::path::PathBuf;
use text_io::read;

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
    Branch {},
    /// add the staged changes to the current commit
    CommitAmmend {},
    /// stash changes and rebase from master
    Rebase {},
    /// switch to master and pull
    PullMaster {},
    /// delete all branches except master
    BranchClean {},
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
        Some(Commands::CommitAmmend {}) => {
            commit_amend();
        }
        Some(Commands::Rebase {}) => {
            rebase();
        }
        Some(Commands::PullMaster {}) => {
            pull_master();
        }
        Some(Commands::BranchClean {}) => {
            branch_clean();
        }
        None => {}
    }
}

fn branch_new() {
    println!("Ticket number:");
    let mut ticket_no: String = read!("{}\n");
    ticket_no = ticket_no.trim().replace("#", "");

    println!("Enter a short description: ");
    let desc: String = read!("{}\n");

    let master_branch = master_branch();

    let branch_name: String = [
        "chris/",
        &ticket_no,
        "-",
        &desc.trim().to_lowercase().replace(" ", "_"),
    ]
    .join("");
    println!("{}", branch_name);

    // TODO must be a better way to check exit status and handle errors
    if cmd!(
        "git",
        "stash",
        "save",
        format!("GitTool - stashed to create: {}", branch_name)
    )
    .run()
    .is_err()
    {
        println!("Error: stashing");
    }

    if cmd!("git", "checkout", &master_branch).run().is_err() {
        println!("Error: checkout master")
    }

    if cmd!("git", "pull").run().is_err() {
        println!("Error: git pull")
    }

    if cmd!("git", "checkout", "-b", &branch_name).run().is_err() {
        println!("Error: checkout branch")
    }

    if cmd!(
        "emacsclient",
        "-q",
        "--eval",
        format!("(work/log-ticket \"{}\" \"{}\")", ticket_no, desc)
    )
    .run()
    .is_err()
    {
        println!("Error: emacsclient");
    }

    if cmd!("bundle", "install").run().is_err() {
        println!("Error: bundle install")
    }
    if cmd!("bundle", "exec", "rake", "db:migrate").run().is_err() {
        println!("Error: rake db:migrate")
    }
}

fn commit_amend() {
    if cmd!("git", "commit", "--amend").run().is_err() {
        panic!("Error: checkout branch")
    }
}

fn rebase() {
    pull_master();
    let master_branch = master_branch();

    if cmd!("git", "rebase", &master_branch).run().is_err() {
        println!("Error: rebase master")
    }
    if cmd!("bundle", "install").run().is_err() {
        println!("Error: bundle install")
    }
    if cmd!("bundle", "exec", "rake", "db:migrate").run().is_err() {
        println!("Error: rake db:migrate")
    }
    // FIXME only do this if something was stashed, maybe with git status -s
    //if cmd!("git", "stash", "pop").run().is_err() {
    //    println!("Error: rebase master")
    //}
}

fn pull_master() {
    let master_branch = master_branch();
    let branch_name = cmd!("git", "branch", "--show-current").read().unwrap();
    let stash_name: String = ["GitTool - stashed for rebase:", &branch_name].join(" ");

    if cmd!("git", "stash", "save", &stash_name).run().is_err() {
        println!("Error: stash save")
    }

    if cmd!("git", "checkout", &master_branch).run().is_err() {
        println!("Error: checkout master")
    }

    if cmd!("git", "pull", "origin", &master_branch).run().is_err() {
        println!("Error: pull master")
    }

    if cmd!("git", "checkout", &branch_name).run().is_err() {
        println!("Error: checkout branch")
    }
}

fn master_branch() -> std::string::String {
    let branches = cmd!("git", "branch").read().unwrap();

    if branches.contains("master") {
        return "master".to_string();
    } else if branches.contains("main") {
        return "main".to_string();
    } else {
        panic!("could not detect master branch!");
    }
}

fn branch_clean() {
    let repo = Repository::open(".").expect("Failed to open repository");
    let branches = repo
        .branches(Some(BranchType::Local))
        .expect("Failed to get branches");
    let head = repo.head().expect("Failed to get head");

    let current_branch = head.shorthand().expect("Failed to get current branch name");
    let master_branch = master_branch();

    for branch in branches {
        let (mut branch, _) = branch.expect("Failed to get branch");
        let branch_name = branch.name().expect("Failed to get branch name");

        if branch_name != Some(&master_branch) && branch_name != Some(&current_branch) {
            branch.delete().expect("Failed to delete branch");
        }
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
//function gittool-last-branch -d "switch to the last non-master branch used"
//    echo "write me"
//end
//
//function gittool-commit-amend -d "commit the staged changes to the current commit"
//    git commit --amend
//end
