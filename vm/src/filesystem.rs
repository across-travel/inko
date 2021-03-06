//! Helpers for working with the filesystem.

use crate::date_time::DateTime;
use crate::object_pointer::ObjectPointer;
use crate::object_value;
use crate::process::RcProcess;
use crate::runtime_error::RuntimeError;
use crate::vm::state::RcState;
use std::fs;

const TIME_CREATED: i64 = 0;
const TIME_MODIFIED: i64 = 1;
const TIME_ACCESSED: i64 = 2;

const TYPE_INVALID: i64 = 0;
const TYPE_FILE: i64 = 1;
const TYPE_DIRECTORY: i64 = 2;

/// Returns a DateTime for the given path.
///
/// The `kind` argument specifies whether the creation, modification or access
/// time should be retrieved.
pub fn date_time_for_path(
    path: &str,
    kind: i64,
) -> Result<DateTime, RuntimeError> {
    let meta = fs::metadata(path)?;

    let system_time = match kind {
        TIME_CREATED => meta.created()?,
        TIME_MODIFIED => meta.modified()?,
        TIME_ACCESSED => meta.accessed()?,
        _ => {
            return Err(RuntimeError::Panic(format!(
                "{} is not a valid type of timestamp",
                kind
            )));
        }
    };

    Ok(DateTime::from_system_time(system_time))
}

/// Returns the type of the given path.
pub fn type_of_path(path: &str) -> i64 {
    if let Ok(meta) = fs::metadata(path) {
        if meta.is_dir() {
            TYPE_DIRECTORY
        } else {
            TYPE_FILE
        }
    } else {
        TYPE_INVALID
    }
}

/// Returns an Array containing the contents of a directory.
///
/// The entries are allocated right away so no additional mapping of vectors is
/// necessary.
pub fn list_directory_as_pointers(
    state: &RcState,
    process: &RcProcess,
    path: &str,
) -> Result<ObjectPointer, RuntimeError> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path().to_string_lossy().to_string();
        let pointer = process
            .allocate(object_value::string(path), state.string_prototype);

        paths.push(pointer);
    }

    let paths_ptr =
        process.allocate(object_value::array(paths), state.array_prototype);

    Ok(paths_ptr)
}
