use anyhow::Context;

use std::fs;
use std::path::Path;

/// Loads a yaml file with either a `.yml` or `.yaml` extension into the given type
pub(crate) fn load_yaml<T>(dir_path: &Path, base_name: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let file_path = if dir_path.join(format!("{}.yaml", base_name)).exists() {
        dir_path.join(format!("{}.yaml", base_name))
    } else {
        dir_path.join(format!("{}.yml", base_name))
    };
    if !file_path.exists() {
        anyhow::bail!(
            "Could not locate a {}.yaml file in the charm directory: {:?}",
            base_name,
            &dir_path
        );
    }
    let file_content =
        fs::read_to_string(&file_path).context(format!("Could not read file: {:?}", file_path))?;
    let data: T = serde_yaml::from_str(&file_content)
        .context(format!("Could not parse YAML: {:?}", file_path))?;

    Ok(data)
}