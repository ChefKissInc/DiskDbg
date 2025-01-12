use std::io::{BufReader, Seek, Write};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let date = Command::new("date").arg("-Iseconds").output().unwrap();
    assert!(date.status.success());
    let date = String::from_utf8(date.stdout).unwrap();
    let header_template = Arc::new(format!(
        "Command: !CMDLINE\nDate: {}\n#######################################################\n\n",
        date.trim()
    ));

    let timestamp = Command::new("date").arg("+%s").output().unwrap();
    assert!(timestamp.status.success());
    let timestamp = Arc::new(
        String::from_utf8(timestamp.stdout)
            .unwrap()
            .trim()
            .to_owned(),
    );

    let dmesg_loop = {
        let timestamp = timestamp.clone();
        let header_template = header_template.clone();
        std::thread::spawn(move || {
            let mut file =
                std::fs::File::create(format!("/Library/Logs/dmesg-{timestamp}.txt")).unwrap();
            file.write_all(&header_template.replace("!CMDLINE", "dmesg").into_bytes())
                .unwrap();
            file.sync_all().unwrap();
            let mut length = BufReader::new(&file).buffer().len();

            loop {
                let Ok(output) = Command::new("dmesg").output() else {
                    eprintln!("warning: failed to execute `dmesg` command");
                    continue;
                };
                file.write_all(&output.stdout[length..]).unwrap();
                file.sync_all().unwrap();
                length += output.stdout.len() - length;
                std::thread::sleep(Duration::from_millis(10));
            }
        })
    };

    let ioreg_loop = {
        let timestamp = timestamp.clone();
        let header_template = header_template.clone();

        std::thread::spawn(move || {
            let mut file =
                std::fs::File::create(format!("/Library/Logs/ioreg-{timestamp}.txt")).unwrap();

            loop {
                let Ok(output) = Command::new("ioreg").args(["-w0", "-flx"]).output() else {
                    eprintln!("warning: failed to execute `ioreg -w0 -flx` command");
                    continue;
                };
                let buf_reader = BufReader::new(&file);
                if buf_reader.buffer() == output.stdout {
                    continue;
                }
                file.seek(std::io::SeekFrom::Start(0)).unwrap();
                file.write_all(
                    &header_template
                        .replace("!CMDLINE", "ioreg -w0 -flx")
                        .into_bytes(),
                )
                .unwrap();
                file.write_all(&output.stdout).unwrap();
                file.sync_all().unwrap();
                std::thread::sleep(Duration::from_millis(10));
            }
        })
    };

    let agdcdiagnose_loop = std::thread::spawn(move || {
        let mut file =
            std::fs::File::create(format!("/Library/Logs/AGDCDiagnose-{timestamp}.txt")).unwrap();

        loop {
            let Ok(output) = Command::new(
                "/System/Library/Extensions/AppleGraphicsControl.kext/Contents/MacOS/AGDCDiagnose",
            )
            .output() else {
                eprintln!("warning: failed to execute `/System/Library/Extensions/AppleGraphicsControl.kext/Contents/MacOS/AGDCDiagnose` command");
                continue;
            };
            let buf_reader = BufReader::new(&file);
            if buf_reader.buffer() == output.stdout {
                continue;
            }
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(
                &header_template
                    .replace("!CMDLINE", "/System/Library/Extensions/AppleGraphicsControl.kext/Contents/MacOS/AGDCDiagnose")
                    .into_bytes(),
            )
            .unwrap();
            file.write_all(&output.stdout).unwrap();
            file.sync_all().unwrap();
            std::thread::sleep(Duration::from_millis(10));
        }
    });

    dmesg_loop.join().unwrap();
    ioreg_loop.join().unwrap();
    agdcdiagnose_loop.join().unwrap();
}
