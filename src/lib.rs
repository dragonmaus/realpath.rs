use std::{
    env,
    ffi::{OsStr, OsString},
    io,
    path::{
        Component::Prefix,
        Path, PathBuf,
        Prefix::{DeviceNS, Disk, Verbatim, VerbatimDisk, VerbatimUNC, UNC},
    },
};

pub fn realpath<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref();
    let path = match path.canonicalize() {
        Err(error) => {
            return Err(io::Error::new(
                error.kind(),
                format!("error parsing path '{}': {}", path.to_string_lossy(), error),
            ));
        }
        Ok(path) => path,
    };
    let mut new_path = PathBuf::new();

    for component in path.components() {
        match component {
            // deal with Windows Verbatim madness
            Prefix(pc) => match pc.kind() {
                DeviceNS(component) | Verbatim(component) => new_path.push(component),
                Disk(c) | VerbatimDisk(c) => new_path.push(format!("{}:", c as char)),
                UNC(server, share) | VerbatimUNC(server, share) => {
                    new_path.push(r"\\");
                    new_path.push(server);
                    new_path.push(share);
                }
            },
            _ => new_path.push(component.as_os_str()),
        }
    }

    Ok(new_path)
}

pub fn realpaths<S: AsRef<OsStr>>(paths: S) -> io::Result<OsString> {
    let paths = paths.as_ref();
    let mut new_paths: Vec<PathBuf> = Vec::new();

    for path in env::split_paths(paths) {
        if path.as_os_str().is_empty() {
            continue;
        }
        let path = match realpath(&path) {
            Err(ref error) if error.kind() == io::ErrorKind::NotFound => path,
            Err(error) => return Err(error),
            Ok(path) => path,
        };

        if !new_paths.contains(&path) {
            new_paths.push(path);
        }
    }

    match env::join_paths(new_paths) {
        Err(error) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "error parsing path list '{}': {}",
                paths.to_string_lossy(),
                error
            ),
        )),
        Ok(paths) => Ok(paths),
    }
}
