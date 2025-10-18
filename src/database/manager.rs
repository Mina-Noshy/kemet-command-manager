use crate::errors::app_error::AppError;
use crate::models::command::CliCommand;
use bincode::{deserialize, serialize};
use sled::Db;

const DATABASE_NAME: &str = "kemet_db";

pub struct CommandManager {
    db: Db,
    next_id: u64,
}

impl CommandManager {
    pub fn new() -> Result<Self, AppError> {
        let db = sled::open(DATABASE_NAME)?;
        let next_id = Self::get_next_id(&db);
        Ok(CommandManager { db, next_id })
    }

    fn get_next_id(db: &Db) -> u64 {
        let mut max_id = 0u64;
        for result in db.iter() {
            if let Ok((key, _)) = result {
                // Convert key bytes directly to u64
                if key.len() == 8 {
                    // u64 takes 8 bytes
                    let mut bytes = [0u8; 8];
                    if key.len() == 8 {
                        bytes.copy_from_slice(&key);
                        let id = u64::from_be_bytes(bytes);
                        if id > max_id {
                            max_id = id;
                        }
                    }
                }
            }
        }
        max_id + 1
    }

    pub fn add_command(&mut self, name: String, description: String) -> Result<(), AppError> {
        let command = CliCommand {
            id: self.next_id,
            name,
            command: description,
            is_active: true,
        };

        let key = self.next_id.to_be_bytes(); // Use raw u64 bytes as key
        let serialized = serialize(&command)?;
        self.db.insert(key, serialized)?;
        self.db.flush()?; // Ensure data is written to disk
        self.next_id += 1;

        println!("Command added: {} - {}", command.name, command.command);
        Ok(())
    }

    pub fn get_command_by_id(&self, id: u64) -> Result<Option<CliCommand>, AppError> {
        let key = id.to_be_bytes();
        if let Some(value) = self.db.get(key)? {
            let command: CliCommand = deserialize(&value)?;
            Ok(Some(command))
        } else {
            Ok(None)
        }
    }

    pub fn get_command_by_name(&self, name: &str) -> Result<Option<CliCommand>, AppError> {
        for entry in self.db.iter() {
            let (_, value) = entry?;
            let command: CliCommand = deserialize(&value)?;
            if command.name == name {
                return Ok(Some(command));
            }
        }
        Ok(None)
    }

    pub fn get_all_commands(&self) -> Result<Vec<CliCommand>, AppError> {
        let mut commands = Vec::new();

        for entry in self.db.iter() {
            let (_, value) = entry?;
            let command: CliCommand = deserialize(&value)?;
            commands.push(command);
        }

        // Sort commands by ID for consistent ordering
        commands.sort_by_key(|command| command.id);
        Ok(commands)
    }

    pub fn mark_command_active(&self, id: u64) -> Result<bool, AppError> {
        if let Some(mut command) = self.get_command_by_id(id)? {
            command.is_active = true;
            let key = id.to_be_bytes();
            let serialized = serialize(&command)?;
            self.db.insert(key, serialized)?;
            self.db.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn mark_command_inactive(&self, id: u64) -> Result<bool, AppError> {
        if let Some(mut command) = self.get_command_by_id(id)? {
            command.is_active = false;
            let key = id.to_be_bytes();
            let serialized = serialize(&command)?;
            self.db.insert(key, serialized)?;
            self.db.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn remove_command(&self, id: u64) -> Result<bool, AppError> {
        let key = id.to_be_bytes();
        let result = self.db.remove(key)?;
        if result.is_some() {
            self.db.flush()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn clear_all_commands(&mut self) -> Result<(), AppError> {
        // Use sled's clear method to remove all entries
        self.db.clear()?;
        self.db.flush()?;
        self.next_id = 1; // Reset to start from 1

        println!("All commands cleared.");
        Ok(())
    }
}
