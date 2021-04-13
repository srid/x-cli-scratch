use directories::ProjectDirs;
use exitfailure::ExitFailure;
use rustbreak::{deser::Yaml, PathDatabase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Version {
    V1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Doc {
    // If data type changes, up the version and write migration.
    version: Version,
    data: serde_yaml::Value,
}

impl Default for Doc {
    fn default() -> Self {
        let data: V1Data = HashMap::new();
        Doc {
            version: Version::V1,
            data: serde_yaml::to_value(data).expect("yaml opps"),
        }
    }
}

type V1Data = HashMap<u32, Entity>;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Entity {
    id: String,
}

fn edit_data<F>(db: &mut Doc, f: F)
where
    F: Fn(&mut V1Data) -> (),
{
    let val: &mut V1Data = &mut serde_yaml::from_value(db.data.clone()).expect("err");
    f(val);
    db.data = serde_yaml::to_value(val).expect("err");
}

fn main() -> Result<(), ExitFailure> {
    let proj_dirs =
        ProjectDirs::from("ca", "srid", "X").ok_or(Error::new(ErrorKind::Other, "No Home"))?;
    let db_file = proj_dirs.data_dir().with_extension("yaml");

    let db = PathDatabase::<Doc, Yaml>::load_from_path_or_default(db_file)?;

    db.read(|db| {
        println!("Init: {:?}", db.data);
    })?;

    db.write(|db| {
        edit_data(db, |val: &mut V1Data| {
            val.insert(
                0,
                Entity {
                    id: "test".to_string(),
                },
            );
        });
    })?;
    db.save()?;

    db.read(|db| {
        println!("Writ: {:?}", db.data);
    })?;

    Ok(())
}
