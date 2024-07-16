use std::{fs, path::PathBuf};
use crop::Rope;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum CopyPosition {
    Prepend,
    Append,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyPatch {
    pub position: CopyPosition,
    pub target: String,
    pub sources: Vec<PathBuf>,
}
impl CopyPatch {
    /// Apply a copy patch onto the provided buffer and name.
    /// If the name is *not* a valid target of this patch, return false and do not
    /// modify the buffer.
    /// If the name *is* a valid target of this patch, prepend or append the source file(s)'s contents
    /// and return true.
    pub fn apply(&self, target: &str, rope: &mut Rope) -> bool {
        if self.target != target {
            return false;
        }

        // Merge the provided payloads into a single buffer. Each source path should
        // be made absolute by the patch loader.
        for source in self.sources.iter() {
            let contents = fs::read_to_string(source)
                .unwrap_or_else(|e| panic!("Failed to read patch file at {source:?}: {e:?}"));

            // Append or prepend the patch's lines onto the provided buffer.
            match self.position {
                CopyPosition::Prepend => { 
                    rope.insert(0, "\n");
                    rope.insert(0, &contents);
                },
                CopyPosition::Append => {
                    rope.insert(rope.byte_len(), "\n");
                    rope.insert(rope.byte_len(), &contents);
                }
                CopyPosition::At => {
                    log::warn!("Did not overwrite buffer with a Copy patch in the At position");
                }
            }
        }

        true
    }
}
