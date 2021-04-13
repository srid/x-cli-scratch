use chrono::prelude::*;
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

type EntityId = String;

// Migration trick for field additions:
// When adding a new field, making it an `Option`, then load and write back the
// data. Finally remove the Option, and once again load back and write the data.
type V1Data = HashMap<EntityId, Entity>;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Entity {
    id: EntityId,
    created: DateTime<Utc>,
    content: String,
    // List of references to other entities
    refs: Vec<Ref>,
    // props: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Ref {
    // An ordinary relationship with no explicit hierarchy
    RefSee(EntityId),
    // A strong parent-child relationship
    RefChild(EntityId),
}

fn edit_data<F>(db: &mut Doc, f: F)
where
    F: Fn(&mut V1Data) -> (),
{
    let val: &mut V1Data = &mut get_data(db);
    f(val);
    db.data = serde_yaml::to_value(val).expect("err");
}

fn get_data(db: &Doc) -> V1Data {
    serde_yaml::from_value(db.data.clone()).expect("err")
}

fn main() -> Result<(), ExitFailure> {
    let proj_dirs =
        ProjectDirs::from("ca", "srid", "X").ok_or(Error::new(ErrorKind::Other, "No Home"))?;
    let db_file = proj_dirs.data_dir().with_extension("yaml");

    let db = PathDatabase::<Doc, Yaml>::load_from_path_or_default(db_file)?;

    db.read(|db| {
        let val = get_data(db);
        println!("Init: {:?}", val);
    })?;

    db.write(|db| {
        edit_data(db, |val: &mut V1Data| {
            val.insert(
                "first".to_string(),
                Entity {
                    id: "test".to_string(),
                    created: Utc::now(),
                    content: "Hello world".to_string(),
                    refs: vec![],
                    // props: None,
                },
            );
        });
    })?;
    db.save()?;

    db.read(|db| {
        let val = get_data(db);
        println!("Writ: {:?}", val);
    })?;

    Ok(())
}
