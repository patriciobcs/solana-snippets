use std::{fs::read_dir, path::{PathBuf, Path}, io};

pub fn get_dirs<'a>(dir: &'a Path) -> io::Result<Vec<PathBuf>> {
	let mut paths = vec![];
	let ignore = vec!["target", "node_modules"];
	if dir.is_dir() {
			'outer: for entry in read_dir(dir)? {
					let entry = entry?;
					let path = entry.path();
					if path.is_dir() {
							for dir in &ignore {
									if path.ends_with(dir) {
											continue 'outer;
									}
							}
							paths.extend(get_dirs(&path)?)
					} else {
							paths.push(path)
					}
			}
	}
	Ok(paths)
}