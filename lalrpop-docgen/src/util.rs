use std::fs::create_dir_all;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use std::{error::Error, fs::File, path::Path};

use crate::session::Session;

pub fn maybe_mkdirp(path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        create_dir_all(path)?
    }
    Ok(())
}

pub fn default_dir(
    maybe_dir: &Option<PathBuf>,
    default_rel_path: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    if let Some(dir) = maybe_dir {
        Ok(dir.as_path().join(default_rel_path).to_owned())
    } else {
        let base_path = std::env::current_dir()?;
        let full_path = format!("{}/{}", base_path.to_string_lossy(), default_rel_path);
        Ok(Path::new(&full_path).to_owned())
    }
}

pub fn out_dir(session: &Rc<Session>) -> Result<PathBuf, Box<dyn Error>> {
    default_dir(&session.out_dir, "docs")
}

pub fn svg_dir(session: &Rc<Session>) -> Result<PathBuf, Box<dyn Error>> {
    default_dir(&session.out_dir, "svg")
}

pub fn prolog_dir(session: &Rc<Session>) -> Result<PathBuf, Box<dyn Error>> {
    let base_path = std::env::current_dir()?;
    if let Some(dir) = &session.prolog_dir {
	if dir.is_relative() {
            Ok(base_path.join(&dir.as_path().to_string_lossy().to_string()).as_path().to_owned())
	} else {
            Ok(dir.as_path().to_owned())
	}
    } else {
        let full_path = format!("{}/static/prolog", base_path.to_string_lossy());
        Ok(Path::new(&full_path).to_owned())
    }
}

pub fn epilog_dir(session: &Rc<Session>) -> Result<PathBuf, Box<dyn Error>> {
    let base_path = std::env::current_dir()?;
    if let Some(dir) = &session.epilog_dir {
	if dir.is_relative() {
            Ok(base_path.join(&dir.as_path().to_string_lossy().to_string()).as_path().to_owned())
	} else {
            Ok(dir.as_path().to_owned())
	}
    } else {
        let full_path = format!("{}/static/epilog", base_path.to_string_lossy());
        Ok(Path::new(&full_path).to_owned())
    }
}

pub fn read_to_string(path: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(why) => return Err(format!("Unable to open {}: {}", path.display(), why).into()),
        Ok(file) => file,
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => return Err(format!("Unable to read {}: {}", path.display(), why).into()),
        Ok(_) => Ok(data),
    }
}
