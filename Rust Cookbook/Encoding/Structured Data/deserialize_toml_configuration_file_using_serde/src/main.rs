use error_chain::error_chain;
use serde::Deserialize;

use std::collections::HashMap;

error_chain! {
   foreign_links {
       IOError(std::io::Error);
       TOMLError(toml::de::Error);
   }
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn main() -> Result<()> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["cassnull <cassnull@mail.ru>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(
        package_info.package.authors,
        vec!["cassnull <cassnull@mail.ru>"]
    );
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}
