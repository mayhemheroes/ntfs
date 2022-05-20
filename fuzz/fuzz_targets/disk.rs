#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut disk = std::io::Cursor::new(data);
    if let Ok(fs) = ntfs::Ntfs::new(&mut disk) {
        if let Ok(root_dir) = fs.root_directory(&mut disk) {
            if let Ok(index) = root_dir.directory_index(&mut disk) {
                let mut iter = index.entries();

                while let Some(entry) = iter.next(&mut disk) {
                    if let Ok(entry) = entry {
                        let _ = entry.key();
                    }
                }
            }
        }
    }
});
