use std::io::{BufReader, Seek, Write};
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

fn cmd_loop(program: &str, args: &[&str], date: &str, timestamp: &str) -> ! {
    let program_name = program.split("/").last().unwrap_or(program);

    let mut file =
        std::fs::File::create(format!("/Library/Logs/{program_name}-{timestamp}.txt")).unwrap();

    let cmdline = program_name.to_owned() + " " + &args.join(" ");
    let cmdline = cmdline.trim();
    let header = format!(
        "Command: {cmdline}\nDate: {date}\n#######################################################\n\n"
    );

    loop {
        let Ok(output) = Command::new(program).args(args).output() else {
            eprintln!("warning: failed to execute `{cmdline}` command");
            continue;
        };
        if !output.status.success() {
            eprintln!("warning: failed to execute `{cmdline}` command");
            continue;
        }
        let buf_reader = BufReader::new(&file);
        if buf_reader.buffer().len() > header.len()
            && buf_reader.buffer()[header.len()..] == output.stdout
        {
            continue;
        }
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        file.write_all(header.as_bytes()).unwrap();
        file.write_all(&output.stdout).unwrap();
        file.sync_all().unwrap();
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let date = Command::new("date")
        .arg("+\"%Y-%m-%dT%H:%M:%S%z\"")
        .output()
        .unwrap();
    assert!(date.status.success());
    let date = Arc::new(String::from_utf8(date.stdout).unwrap());

    let timestamp = Command::new("date").arg("+%s").output().unwrap();
    assert!(timestamp.status.success());
    let timestamp = Arc::new(
        String::from_utf8(timestamp.stdout)
            .unwrap()
            .trim()
            .to_owned(),
    );

    let dmesg_loop = {
        let date = Arc::clone(&date);
        let timestamp = Arc::clone(&timestamp);
        std::thread::spawn(move || {
            cmd_loop("dmesg", &[], &date, &timestamp);
        })
    };

    let ioreg_loop = {
        let date = Arc::clone(&date);
        let timestamp = Arc::clone(&timestamp);
        std::thread::spawn(move || {
            cmd_loop("ioreg", &["-w0", "-flx"], &date, &timestamp);
        })
    };

    let agdcdiagnose_loop = {
        let date = Arc::clone(&date);
        let timestamp = Arc::clone(&timestamp);
        std::thread::spawn(move || {
            cmd_loop(
                "/System/Library/Extensions/AppleGraphicsControl.kext/Contents/MacOS/AGDCDiagnose",
                &[],
                &date,
                &timestamp,
            );
        })
    };

    dmesg_loop.join().unwrap();
    ioreg_loop.join().unwrap();
    agdcdiagnose_loop.join().unwrap();
}
