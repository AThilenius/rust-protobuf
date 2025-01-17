use crate::customize::Customize;
use crate::gen::rust::rel_path::RustRelativePath;

pub struct FileAndMod {
    pub file: String,
    pub relative_mod: RustRelativePath,
    pub customize: Customize,
}
