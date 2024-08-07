use std::io::{BufReader, Seek, Write};
use std::process::Command;
use std::time::Duration;

fn main() {
    let date = time::OffsetDateTime::now_local()
        .unwrap()
        .format(
            &time::format_description::parse("[day]-[month]-[year] [hour].[minute].[second]")
                .unwrap(),
        )
        .unwrap();
    let dmesg_loop = {
        let date = date.clone();
        std::thread::spawn(move || {
            let mut file = std::fs::File::create(format!("/Users/dmesg-{date}.txt")).unwrap();
            let mut length = BufReader::new(&file).buffer().len();

            loop {
                let output = Command::new("dmesg").output().unwrap();
                file.write_all(&output.stdout[length..]).unwrap();
                file.sync_all().unwrap();
                length += output.stdout.len() - length;
                std::thread::sleep(Duration::from_millis(10));
            }
        })
    };

    let ioreg_loop = std::thread::spawn(move || {
        let mut file = std::fs::File::create(format!("/Users/ioreg-{date}.txt")).unwrap();
        loop {
            let output = Command::new("ioreg")
                .args(["-w0", "-flx"])
                .output()
                .unwrap();
            let buf_reader = BufReader::new(&file);
            if buf_reader.buffer() == output.stdout {
                continue;
            }
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(&output.stdout).unwrap();
            file.sync_all().unwrap();
            std::thread::sleep(Duration::from_millis(10));
        }
    });

    dmesg_loop.join().unwrap();
    ioreg_loop.join().unwrap();
}
