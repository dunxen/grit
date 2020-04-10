use std::{env, fs, path::PathBuf};

fn main() {
    let mut args = env::args().skip(1);

    if let Some(command) = args.next() {
        match &command[..] {
            "init" => init(&mut args),
            _ => unimplemented!(),
        }
    }
}

fn init(args: &mut std::iter::Skip<env::Args>) {
    let path = args
        .next()
        .map(|s| Ok(PathBuf::from(s)))
        .unwrap_or(env::current_dir());

    match path {
        Ok(root_path) => {
            let git_path = root_path.join(".git");

            for dir in &["objects", "refs"] {
                let dir_path = git_path.join(dir);
                let res = fs::create_dir_all(&dir_path);

                if let Err(error) = res {
                    eprintln!("Could not create {:?}: {:?}", dir_path, error);
                    std::process::exit(1)
                }
            }

            println!(
                "Initialized empty git repository in {}",
                git_path.to_str().unwrap_or("directory")
            );
        }
        Err(error) => panic!("Error forming path: {:?}", error),
    }
}
