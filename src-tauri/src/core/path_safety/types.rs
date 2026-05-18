use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PathErrorCode {
    InvalidIdLength,
    InvalidIdFormat,
    PathResolveRootFailed,
    PathResolvePathFailed,
    PathResolveParentFailed,
    PathEscapesRoot,
}
