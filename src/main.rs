use anyhow::{bail, Result};
use errno::errno;
use libc::{close, dup, dup2, open, O_RDWR, STDIN_FILENO, STDOUT_FILENO};
use std::env;
use std::ffi::{CStr, OsString};
use std::fs::File;
use std::io::copy;
use std::process::Command;
use tempfile::NamedTempFile;

macro_rules! check {
    ($e:expr) => {{
        let rc = $e;
        if rc < 0 {
            bail!("{} failed: {}", stringify!($e), errno());
        }
        rc
    }};
}

fn main() -> Result<()> {
    let vi = OsString::from("vi");
    let editor = env::var_os("EDITOR").unwrap_or(vi);
    let mut args: Vec<OsString> = env::args_os().collect();
    let mut tmp = NamedTempFile::new()?;
    let dev_tty = CStr::from_bytes_with_nul(b"/dev/tty\0")?.as_ptr();

    // Special case for setting vim filetype
    if let Some(e) = editor.to_str() {
        if e.contains("vi") && args.len() == 2 {
            if let Some(arg) = args[1].to_str() {
                if !arg.starts_with("-") && !arg.starts_with("+") {
                    let prefix = "+set ft=";
                    let ft = arg;
                    let mut arg1 = OsString::with_capacity(prefix.len() + ft.len());
                    arg1.push(prefix);
                    arg1.push(ft);
                    args[1] = arg1;
                }
            }
        }
    }

    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    copy(&mut handle, &mut tmp)?;

    let (tty, stdout_fd) = unsafe {
        let fd = check!(dup(STDOUT_FILENO));
        let tty = check!(open(dev_tty, O_RDWR));
        check!(dup2(tty, STDIN_FILENO));
        check!(dup2(tty, STDOUT_FILENO));
        (tty, fd)
    };

    let (_, path) = tmp.into_parts();
    let status = Command::new(editor).args(&args[1..]).arg(&path).status()?;
    if !status.success() {
        let vp = &args[0].to_str().unwrap_or("vp");
        bail!("{} aborted", vp);
    }

    unsafe {
        check!(close(tty));
        check!(close(STDIN_FILENO));
        check!(dup2(stdout_fd, STDOUT_FILENO));
    }

    let mut f = File::open(path)?;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    copy(&mut f, &mut handle)?;
    Ok(())
}
