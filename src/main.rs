use std::path::PathBuf;
use anyhow::{anyhow, Result};
use include_dir::{Dir, include_dir};
use regex::Regex;
use walkdir::WalkDir;

const SKELTON: Dir<'_> = include_dir!("./skelton");

fn validiate_project_name(s: &str) -> Result<PathBuf> {
	let valid_proj_name_re: Regex = Regex::new("^[a-zA-Z][a-zA-Z0-9_-]*")?;
	if valid_proj_name_re.is_match(s) {
		let mut ret = PathBuf::new();
		ret.push(s);
		Ok(ret)
	}
	else {
		Err(anyhow!(format!("invalid project name! {{{s}}}")))
	}
}

fn main() -> Result<()> {
	println!("input project name: ");
	let mut buf = String::new();
	std::io::stdin().read_line(&mut buf)?;
	let project_name = validiate_project_name(&buf)?;

	std::fs::create_dir(&project_name)?;
	SKELTON.extract(&project_name)?;

	std::fs::rename(&project_name.join("include/").join("skelton"), project_name.clone().join("include/").join(&project_name))?;
	for entry in WalkDir::new(&project_name)
		.into_iter()
		.filter_map(|e| match e {
			Ok(entry) => Some(entry),
			Err(err) => {
				eprintln!("Error reading entry: {}", err);
				None
			}
		})
		.filter(|e| e.file_type().is_file())
		{
			let path = entry.path();

			// ファイルを処理
			let content = std::fs::read_to_string(path)?;
			std::fs::write(path, content.replace("<stew_replace:proj_name>", project_name.as_os_str().to_str().ok_or(anyhow!("fail to convert str."))?))?;
		}

	Ok(())
}
