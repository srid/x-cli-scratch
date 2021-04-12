use directories::ProjectDirs;
use exitfailure::ExitFailure;
use rustbreak::{deser::Ron, PathDatabase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Entity {
    id: String,
}

fn main() -> Result<(), ExitFailure> {
    let proj_dirs =
        ProjectDirs::from("ca", "srid", "X").ok_or(Error::new(ErrorKind::Other, "No Home"))?;
    let ron_file = proj_dirs.data_dir().with_extension("ron");

    let db = PathDatabase::<HashMap<u32, Entity>, Ron>::load_from_path(ron_file)?;

    db.read(|db| {
        println!("Init: {:?}", db.get(&0));
    })?;

    db.write(|db| {
        db.insert(
            0,
            Entity {
                id: "test".to_string(),
            },
        );
    })?;
    db.save()?;

    db.read(|db| {
        println!("Just wrote: {:?}", db.get(&0));
    })?;

    Ok(())
}
