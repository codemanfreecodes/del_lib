// src/lib.rs

pub mod del {
    use std::fs::{self, File};
    use std::io::{self, Read, Write};
    use rand::Rng;
    use ring::digest;
    use filetime::{self, FileTime, set_file_times};
    use std::path::Path;

    const NUM_HASH_PASSES: usize = 35; // Number of hash passes for secure deletion
    const BUFFER_SIZE: usize = 1024; // Buffer size for each read/write operation

    fn remove_magic_number(data: &mut Vec<u8>) {
        // ... (your existing remove_magic_number function)
    // Replace the magic number (example: 0xDEADBEEF) with random data
    let mut rng = rand::thread_rng();
    let magic_number = 0xDEADBEEFu32;

    for chunk in data.chunks_mut(4) {
        if chunk.len() == 4 && u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) == magic_number {
            // Replace the magic number with random data
            let random_data: u32 = rng.gen();
            let bytes = random_data.to_le_bytes();
            chunk[0] = bytes[0];
            chunk[1] = bytes[1];
            chunk[2] = bytes[2];
            chunk[3] = bytes[3];
        }
    }

    }

    pub fn secure_delete_file(file_path: &str) -> io::Result<()> {
        // ... (your existing secure_delete_file function)
// Read the file's data
    let mut file = File::open(file_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Remove magic numbers from the data
    remove_magic_number(&mut data);

    // Reopen the file for writing
    let mut file = File::create(file_path)?;

    // Perform cryptographic shredding with multiple hash passes
    for _ in 0..NUM_HASH_PASSES {
        let mut hasher = digest::Context::new(&digest::SHA256);
        hasher.update(&data);
        let hash = hasher.finish();

        // Write the hash to the file, overwriting its contents
        file.write_all(hash.as_ref())?;

        // Update the data with the hash for the next pass
        data = hash.as_ref().to_vec();
    }

    // Sync changes to disk
    file.sync_all()?;

    // Get the file's metadata
    let metadata = fs::metadata(file_path)?;

    // Generate random values for file times
    let mut rng = rand::thread_rng();
    let atime = FileTime::from_unix_time(rng.gen(), 0);
    let mtime = FileTime::from_unix_time(rng.gen(), 0);

    // Set the file times
    set_file_times(Path::new(file_path), atime, mtime)?;

    // Securely delete the file itself by overwriting its name
    let mut deleted_path = file_path.to_owned();
    deleted_path.push_str(".deleted");

    fs::rename(file_path, &deleted_path)?;

    Ok(())

    }
}
