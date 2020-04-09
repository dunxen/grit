use std::{env, fs, path::PathBuf};

fn main() {
    let mut args = env::args().skip(1);

    if let Some(command) = args.next() {
        match &command[..] {
            "init" => {
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
                                panic!("Could not create {:?}: {:?}", dir_path, error)
                            }
                        }
                    }
                    Err(error) => panic!("Error forming path: {:?}", error),
                }
            }
            _ => unimplemented!(),
        }
    }

    println!("Hello, world!");
}
