use std::collections::HashMap;
use std::path::{Path, PathBuf};

use carmen_db::documents::Document as OldDocument;
use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use uuid::Uuid;

use crate::document::Document as NewDocument;

struct OldDocumentInfo {
    id: Uuid,
    checksum: [u8; 32],
}

pub struct AddedDocument {
    pub canonical_path: String,
    pub checksum: [u8; 32],
    pub raw_path: PathBuf,
    pub exported_path: PathBuf,
}

pub struct UpdatedDocument {
    pub id: Uuid,
    pub checksum: [u8; 32],
    pub raw_path: PathBuf,
    pub exported_path: PathBuf,
}

#[derive(Default)]
pub struct DocumentDiff {
    pub added: Vec<AddedDocument>,
    pub updated: Vec<UpdatedDocument>,
    pub removed: Vec<Uuid>,
}

impl DocumentDiff {
    pub async fn compute(old: Vec<OldDocument>, new: Vec<NewDocument>) -> anyhow::Result<Self> {
        let mut diff = Self::default();
        let mut old_documents = HashMap::with_capacity(old.len());

        for OldDocument {
            id,
            canonical_path,
            checksum,
            ..
        } in old
        {
            old_documents.insert(canonical_path, OldDocumentInfo { id, checksum });
        }

        for new_document in new {
            let new_checksum = file_checksum(&new_document.raw_path).await?;

            let old_document = match old_documents.remove(&new_document.canonical_path) {
                Some(doc) => doc,
                None => {
                    diff.added.push(AddedDocument {
                        canonical_path: new_document.canonical_path,
                        checksum: new_checksum,
                        raw_path: new_document.raw_path,
                        exported_path: new_document.exported_path,
                    });
                    continue;
                }
            };

            if new_checksum != old_document.checksum {
                diff.updated.push(UpdatedDocument {
                    id: old_document.id,
                    checksum: new_checksum,
                    raw_path: new_document.raw_path,
                    exported_path: new_document.exported_path,
                });
            }
        }

        for (_, unmatched_document) in old_documents {
            diff.removed.push(unmatched_document.id);
        }

        Ok(diff)
    }
}

async fn file_checksum(path: &Path) -> anyhow::Result<[u8; 32]> {
    let file = File::open(path).await?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = reader.read(&mut buffer).await?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().into())
}
