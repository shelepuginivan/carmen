use std::collections::HashMap;
use std::path::{Path, PathBuf};

use carmen_db::documents::Document;
use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::extractors::ExtractedDocument;

struct OldDocument {
    id: Uuid,
    checksum: [u8; 32],
}

pub struct AddedDocument {
    pub canonical_path: String,
    pub checksum: [u8; 32],
    pub file_path: PathBuf,
}

pub struct UpdatedDocument {
    pub id: Uuid,
    pub checksum: [u8; 32],
    pub file_path: PathBuf,
}

#[derive(Default)]
pub struct DocumentDiff {
    pub added: Vec<AddedDocument>,
    pub updated: Vec<UpdatedDocument>,
    pub removed: Vec<Uuid>,
}

impl DocumentDiff {
    pub async fn compute(old: Vec<Document>, new: Vec<ExtractedDocument>) -> anyhow::Result<Self> {
        let mut diff = Self::default();
        let mut old_documents = HashMap::with_capacity(old.len());

        for Document {
            id,
            canonical_path,
            checksum,
            ..
        } in old
        {
            old_documents.insert(canonical_path, OldDocument { id, checksum });
        }

        for new_document in new {
            let new_checksum = file_checksum(&new_document.file_path).await?;

            let old_document = match old_documents.remove(&new_document.canonical_path) {
                Some(doc) => doc,
                None => {
                    diff.added.push(AddedDocument {
                        canonical_path: new_document.canonical_path,
                        checksum: new_checksum,
                        file_path: new_document.file_path,
                    });
                    continue;
                }
            };

            if new_checksum != old_document.checksum {
                diff.updated.push(UpdatedDocument {
                    id: old_document.id,
                    checksum: new_checksum,
                    file_path: new_document.file_path,
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
    let mut file = File::open(path).await?;
    let mut buffer = Vec::new();
    file.read(&mut buffer).await?;
    Ok(Sha256::digest(buffer).into())
}
