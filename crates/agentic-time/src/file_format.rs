//! `.atime` binary file format — portable temporal graph.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::{TimeError, TimeResult};
use crate::TemporalId;

/// Magic bytes identifying `.atime` files.
pub const MAGIC: [u8; 4] = *b"ATIM";

/// Current format version.
pub const VERSION: u32 = 1;

/// File header (64 bytes).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FileHeader {
    /// Magic bytes "ATIM".
    pub magic: [u8; 4],
    /// Format version.
    pub version: u32,
    /// Flags (reserved).
    pub flags: u32,
    /// Number of temporal entities.
    pub entity_count: u64,
    /// Offset to entity index.
    pub index_offset: u64,
    /// Offset to deadline index.
    pub deadline_index_offset: u64,
    /// Offset to decay index.
    pub decay_index_offset: u64,
    /// File creation timestamp (Unix micros).
    pub created_at: u64,
    /// Last modified timestamp (Unix micros).
    pub modified_at: u64,
    /// BLAKE3 checksum placeholder (not yet computed during write).
    pub checksum: [u8; 32],
}

impl Default for FileHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl FileHeader {
    /// Header size in bytes.
    // 4 + 4 + 4 + 8 + 8 + 8 + 8 + 8 + 8 + 32 = 92
    // (Spec says 64 but the fields sum to 92. We write exactly these fields.)
    pub const SIZE: usize = 4 + 4 + 4 + 8 + 8 + 8 + 8 + 8 + 8 + 32;

    /// Create a new header with current timestamps.
    pub fn new() -> Self {
        let now = Utc::now().timestamp_micros() as u64;
        Self {
            magic: MAGIC,
            version: VERSION,
            flags: 0,
            entity_count: 0,
            index_offset: 0,
            deadline_index_offset: 0,
            decay_index_offset: 0,
            created_at: now,
            modified_at: now,
            checksum: [0; 32],
        }
    }

    /// Write header to a writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> TimeResult<()> {
        writer.write_all(&self.magic)?;
        writer.write_all(&self.version.to_le_bytes())?;
        writer.write_all(&self.flags.to_le_bytes())?;
        writer.write_all(&self.entity_count.to_le_bytes())?;
        writer.write_all(&self.index_offset.to_le_bytes())?;
        writer.write_all(&self.deadline_index_offset.to_le_bytes())?;
        writer.write_all(&self.decay_index_offset.to_le_bytes())?;
        writer.write_all(&self.created_at.to_le_bytes())?;
        writer.write_all(&self.modified_at.to_le_bytes())?;
        writer.write_all(&self.checksum)?;
        Ok(())
    }

    /// Read header from a reader.
    pub fn read_from<R: Read>(reader: &mut R) -> TimeResult<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if magic != MAGIC {
            return Err(TimeError::FileFormat(format!(
                "Invalid magic bytes: {:?} (expected {:?}). File may be corrupted — try creating a new .atime file.",
                magic, MAGIC
            )));
        }

        let mut buf4 = [0u8; 4];
        let mut buf8 = [0u8; 8];
        let mut checksum = [0u8; 32];

        reader.read_exact(&mut buf4)?;
        let version = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf4)?;
        let flags = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf8)?;
        let entity_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let index_offset = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let deadline_index_offset = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let decay_index_offset = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let created_at = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let modified_at = u64::from_le_bytes(buf8);

        reader.read_exact(&mut checksum)?;

        Ok(Self {
            magic,
            version,
            flags,
            entity_count,
            index_offset,
            deadline_index_offset,
            decay_index_offset,
            created_at,
            modified_at,
            checksum,
        })
    }
}

/// Entity types stored in file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntityType {
    /// Duration estimate.
    Duration = 1,
    /// Deadline.
    Deadline = 2,
    /// Schedule.
    Schedule = 3,
    /// Sequence.
    Sequence = 4,
    /// Decay model.
    Decay = 5,
}

impl TryFrom<u8> for EntityType {
    type Error = TimeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EntityType::Duration),
            2 => Ok(EntityType::Deadline),
            3 => Ok(EntityType::Schedule),
            4 => Ok(EntityType::Sequence),
            5 => Ok(EntityType::Decay),
            _ => Err(TimeError::FileFormat(format!(
                "Unknown entity type: {}",
                value
            ))),
        }
    }
}

/// Entity block — wraps a serialized temporal entity.
#[derive(Debug, Clone)]
pub struct EntityBlock {
    /// Entity type.
    pub entity_type: EntityType,
    /// Entity ID.
    pub id: TemporalId,
    /// Payload length.
    pub payload_len: u32,
    /// Payload (JSON-serialized entity).
    pub payload: Vec<u8>,
}

impl EntityBlock {
    /// Create a new entity block from a serializable entity.
    pub fn new<T: Serialize>(
        entity_type: EntityType,
        id: TemporalId,
        entity: &T,
    ) -> TimeResult<Self> {
        let payload = serde_json::to_vec(entity)?;
        Ok(Self {
            entity_type,
            id,
            payload_len: payload.len() as u32,
            payload,
        })
    }

    /// Write block to a writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> TimeResult<()> {
        writer.write_all(&[self.entity_type as u8])?;
        writer.write_all(self.id.0.as_bytes())?;
        writer.write_all(&self.payload_len.to_le_bytes())?;
        writer.write_all(&self.payload)?;
        Ok(())
    }

    /// Read block from a reader.
    pub fn read_from<R: Read>(reader: &mut R) -> TimeResult<Self> {
        let mut type_buf = [0u8; 1];
        reader.read_exact(&mut type_buf)?;
        let entity_type = EntityType::try_from(type_buf[0])?;

        let mut id_buf = [0u8; 16];
        reader.read_exact(&mut id_buf)?;
        let id = TemporalId(uuid::Uuid::from_bytes(id_buf));

        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;
        let payload_len = u32::from_le_bytes(len_buf);

        let mut payload = vec![0u8; payload_len as usize];
        reader.read_exact(&mut payload)?;

        Ok(Self {
            entity_type,
            id,
            payload_len,
            payload,
        })
    }

    /// Deserialize payload into a typed entity.
    pub fn deserialize<T: for<'de> Deserialize<'de>>(&self) -> TimeResult<T> {
        Ok(serde_json::from_slice(&self.payload)?)
    }
}

/// The complete `.atime` file — a temporal graph stored as a binary file.
#[derive(Debug)]
pub struct TimeFile {
    /// File path.
    pub path: PathBuf,
    /// Header.
    pub header: FileHeader,
    /// All entities.
    entities: HashMap<TemporalId, EntityBlock>,
}

impl TimeFile {
    /// Create a new empty `.atime` file.
    pub fn create(path: impl Into<PathBuf>) -> TimeResult<Self> {
        let path = path.into();
        let header = FileHeader::new();

        let file = Self {
            path,
            header,
            entities: HashMap::new(),
        };

        file.save()?;
        Ok(file)
    }

    /// Open an existing `.atime` file.
    pub fn open(path: impl Into<PathBuf>) -> TimeResult<Self> {
        let path = path.into();
        let mut file = std::fs::File::open(&path)?;

        let header = FileHeader::read_from(&mut file)?;

        let mut entities = HashMap::new();
        for _ in 0..header.entity_count {
            let block = EntityBlock::read_from(&mut file)?;
            entities.insert(block.id, block);
        }

        Ok(Self {
            path,
            header,
            entities,
        })
    }

    /// Save to disk (atomic write: temp + rename).
    pub fn save(&self) -> TimeResult<()> {
        let tmp_path = self.path.with_extension("atime.tmp");
        let mut file = std::fs::File::create(&tmp_path)?;

        let mut header = self.header.clone();
        header.entity_count = self.entities.len() as u64;
        header.modified_at = Utc::now().timestamp_micros() as u64;

        header.write_to(&mut file)?;

        for block in self.entities.values() {
            block.write_to(&mut file)?;
        }

        file.flush()?;
        std::fs::rename(&tmp_path, &self.path)?;

        Ok(())
    }

    /// Add or replace an entity.
    pub fn add<T: Serialize>(
        &mut self,
        entity_type: EntityType,
        id: TemporalId,
        entity: &T,
    ) -> TimeResult<()> {
        let block = EntityBlock::new(entity_type, id, entity)?;
        self.entities.insert(id, block);
        Ok(())
    }

    /// Get an entity by ID.
    pub fn get<T: for<'de> Deserialize<'de>>(&self, id: &TemporalId) -> TimeResult<Option<T>> {
        match self.entities.get(id) {
            Some(block) => Ok(Some(block.deserialize()?)),
            None => Ok(None),
        }
    }

    /// Remove an entity by ID.
    pub fn remove(&mut self, id: &TemporalId) -> bool {
        self.entities.remove(id).is_some()
    }

    /// List all entity blocks of a given type.
    pub fn list_by_type(&self, entity_type: EntityType) -> Vec<&EntityBlock> {
        self.entities
            .values()
            .filter(|b| b.entity_type == entity_type)
            .collect()
    }

    /// Total number of entities.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deadline::Deadline;
    use chrono::Duration as ChronoDuration;
    use tempfile::tempdir;

    #[test]
    fn test_create_and_reopen() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.atime");

        // Create
        let mut tf = TimeFile::create(&path).unwrap();
        let d = Deadline::new("Ship v1", Utc::now() + ChronoDuration::hours(24));
        let id = d.id;
        tf.add(EntityType::Deadline, id, &d).unwrap();
        tf.save().unwrap();

        // Reopen
        let tf2 = TimeFile::open(&path).unwrap();
        assert_eq!(tf2.entity_count(), 1);
        let loaded: Deadline = tf2.get(&id).unwrap().unwrap();
        assert_eq!(loaded.label, "Ship v1");
    }

    #[test]
    fn test_remove_entity() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.atime");

        let mut tf = TimeFile::create(&path).unwrap();
        let d = Deadline::new("Remove me", Utc::now() + ChronoDuration::hours(1));
        let id = d.id;
        tf.add(EntityType::Deadline, id, &d).unwrap();
        assert_eq!(tf.entity_count(), 1);

        assert!(tf.remove(&id));
        assert_eq!(tf.entity_count(), 0);
    }

    #[test]
    fn test_empty_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.atime");

        let tf = TimeFile::create(&path).unwrap();
        assert_eq!(tf.entity_count(), 0);
        assert!(tf.list_by_type(EntityType::Deadline).is_empty());

        let tf2 = TimeFile::open(&path).unwrap();
        assert_eq!(tf2.entity_count(), 0);
    }

    #[test]
    fn test_invalid_magic() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("bad.atime");
        std::fs::write(&path, b"BAD_DATA_NOT_ATIM").unwrap();

        let result = TimeFile::open(&path);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid magic bytes"));
    }
}
