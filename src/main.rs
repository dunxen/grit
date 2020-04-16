use std::{
    env, fs,
    io::{self, Read},
    path::PathBuf,
};

mod database;
mod workspace;

use database::{Author, Blob, Commit, Database, Entry, Object, Refs, Tree};
use workspace::Workspace;

fn main() {
    let mut args = env::args().skip(1);

    if let Some(command) = args.next() {
        let res = match &command[..] {
            "init" => init(&mut args),
            "commit" => commit(),
            _ => unimplemented!(),
        };

        if let Err(error) = res {
            eprintln!("error: {}", error);
        }
    }
}

fn init(args: &mut std::iter::Skip<env::Args>) -> Result<(), Box<dyn std::error::Error>> {
    let git_path = args
        .next()
        .map(|s| Ok(PathBuf::from(s)))
        .unwrap_or(env::current_dir())?
        .join(".git");

    for dir in &["objects", "refs"] {
        let dir_path = git_path.join(dir);
        fs::create_dir_all(&dir_path)?;
    }

    println!(
        "Initialized empty git repository in {}",
        git_path.to_str().unwrap_or("directory")
    );

    Ok(())
}

fn commit() -> Result<(), Box<dyn std::error::Error>> {
    let root_path = env::current_dir()?;
    let git_path = root_path.join(".git");
    let db_path = git_path.join("objects");

    let ws = Workspace::new(&root_path);
    let db = Database::new(&db_path);
    let refs = Refs::new(&git_path);

    let entries: Vec<Entry> = ws
        .list_files()
        .map(|dir_entry: walkdir::DirEntry| {
            let path = dir_entry.path().to_owned();
            let data = String::from_utf8(Workspace::read_file(&path).unwrap()).unwrap();
            let mut blob = Blob::new(&data);
            db.store(&mut blob).unwrap();

            let stat = Workspace::stat_file(&path).unwrap();
            Entry::new(
                path.file_name().unwrap().to_str().unwrap(),
                &blob.get_oid(),
                stat,
            )
        })
        .collect();

    let mut tree = Tree::new(entries);
    db.store(&mut tree)?;

    // Storing commits
    let parent = refs.read_head();
    let name = env::var("GIT_AUTHOR_NAME")?;
    let email = env::var("GIT_AUTHOR_EMAIL")?;
    let author = Author::new(&name, &email);
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)?;

    let mut commit = Commit::new(&parent, &tree.get_oid(), author, &message);
    db.store(&mut commit)?;
    refs.update_head(&commit.get_oid())?;

    let is_root = if parent.is_none() {
        "(root-commit) "
    } else {
        ""
    };

    println!(
        "[{}{}] {}",
        is_root,
        commit.get_oid(),
        message.lines().next().unwrap()
    );

    Ok(())
}
