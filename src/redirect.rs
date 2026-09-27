use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Stdio;
use std::thread;

pub(crate) fn handle_output(output: &str, redirect: &Option<String>, redirect_mode: bool) {
    if let Some(file_path) = redirect {
        let mut hfile: File;
        if redirect_mode {
            hfile = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .unwrap();
        } else {
            hfile = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(file_path)
                .unwrap();
        }
        hfile.write_all(output.as_bytes()).unwrap();
        hfile.flush().unwrap();
    } else {
        print!("{}", output);
        io::stdout().flush().unwrap();
    }
}

pub(crate) fn stream_pipe<R: Read + Send + 'static>(
    mut pipe: R,
    redirect: Option<String>,
    append: bool,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut file = redirect.map(|path| {
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .truncate(!append)
                .open(path)
                .unwrap()
        });

        let mut buf = [0u8; 4096];
        loop {
            let n = pipe.read(&mut buf).unwrap();
            if n == 0 {
                break;
            }
            if let Some(f) = file.as_mut() {
                f.write_all(&buf[..n]).unwrap();
                f.flush().unwrap();
            } else {
                io::stdout().write_all(&buf[..n]).unwrap();
                io::stdout().flush().unwrap();
            }
        }
    })
}

pub(crate) fn output_pipe(text: &str) -> Stdio {
    let (reader, mut writer) = std::io::pipe().unwrap();
    writer.write_all(text.as_bytes()).unwrap();
    drop(writer);
    Stdio::from(reader)
}
