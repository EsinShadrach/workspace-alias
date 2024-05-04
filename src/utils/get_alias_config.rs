// use std::{
//     env::{self, VarError},
//     path::Path,
// };

// use crate::{LogErrorMsg, WorkspaceError};

// use super::log_err_msg::create_error_msg;

// fn get_alias_path() -> Result<&'static Path, WorkspaceError> {
//     let home_dir = match env::var("HOME") {
//         Ok(path) => path,

//         Err(err) => {
//             create_error_msg(LogErrorMsg {
//                 msg: "Failed to Write Config".to_owned(),
//                 err: err.to_string(),
//             });

//             return Err(WorkspaceError::DirectoryCheckFailed);
//         }
//     };

//     let alias_file = format!("{home_dir}/alias-thing/.workspace-alias");
//     let alias_file_path = Path::new(&alias_file);

//     if alias_file_path.exists() {
//         let to_send = &alias_file_path;
//         return Ok(&to_send);
//     } else {
//         Err(WorkspaceError::FileCheckFailed)
//     }
// }
