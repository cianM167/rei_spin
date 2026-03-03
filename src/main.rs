use std::fs;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};
use include_dir::{include_dir, Dir};

static ASCII_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/ascii_frames");

fn main() -> io::Result<()> {
    let mut entries: Vec<_> = ASCII_DIR.files().collect();
    entries.sort_by_key(|f| f.path());

    let frames: Vec<&str> = entries
        .iter()
        .filter_map(|f| f.contents_utf8())
        .collect();

    if frames.is_empty() {
        println!("directory is empty");
        return Ok(());
    }

    let mut stdout = io::stdout();

    loop {
        for frame in &frames {
            print!("\x1B[H{}", frame);
            stdout.flush()?;

            thread::sleep(Duration::from_millis(170));
        }
    }

}
