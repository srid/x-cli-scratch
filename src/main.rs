use directories::ProjectDirs;
use exitfailure::ExitFailure;
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug)]
struct Entity {
    id: String,
}

fn main() -> Result<(), ExitFailure> {
    let proj_dirs =
        ProjectDirs::from("ca", "srid", "X").ok_or(Error::new(ErrorKind::Other, "No Home"))?;
    let ron_file = proj_dirs.data_dir().with_extension("ron");

    let data = Entity {
        id: "test".to_string(),
    };

    println!("Writing to {}", ron_file.display());
    let pretty = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let fp = File::create(&ron_file)?;
    let () = to_writer_pretty(fp, &data, pretty.clone())?;
    Ok(())
}
