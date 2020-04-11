use std::{env, fs, path::PathBuf};

mod database;
mod workspace;

use database::{Blob, Database, Entry, Object, Tree};
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

    let entries: Vec<Entry> = ws
        .list_files()?
        .map(|dir_entry| {
            let path = dir_entry.unwrap().path();
            let data = String::from_utf8(Workspace::read_file(&path).unwrap()).unwrap();
            let mut blob = Blob::new(&data);
            db.store(&mut blob).unwrap();

            Entry::new(path.to_str().unwrap(), &blob.get_oid())
        })
        .collect();

    let mut tree = Tree::new(entries);
    db.store(&mut tree).unwrap();

    Ok(())
}
