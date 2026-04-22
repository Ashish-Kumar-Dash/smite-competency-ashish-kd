use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use nix::errno::Errno;
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;
use signal_hook::consts::signal::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

fn main() -> io::Result<()> {
    let mut child=spawn_child()?;

    println!(
        "spawned child pid={} in dedicated process group; send SIGINT/SIGTERM to this process",
        child.id()
    );

    let signal_rx=install_signal_listener()?;

    loop {
        if let Some(status)=child.try_wait()? {
            println!("child exited before signal: {status}");
            return Ok(());
        }
        match signal_rx.try_recv() {
            Ok(sig) => {
                let signal=Signal::try_from(sig).unwrap_or(Signal::SIGTERM);
                println!("received signal {signal:?}, shutting down child process group");
                let status=shutdown_process_group(&mut child, Duration::from_secs(5))?;
                println!("shutdown complete: {status}");
                return Ok(());
            }
            Err(mpsc::TryRecvError::Empty) => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                eprintln!("signal listener disconnected unexpectedly");
                return shutdown_with_fallback(&mut child);
            }
        }
    }
}

fn spawn_child() -> io::Result<Child> {
    let mut args=std::env::args().skip(1);
    let mut cmd=if let Some(program) = args.next() {
        let mut cmd=Command::new(program);
        cmd.args(args);
        cmd
    } else {
        // Default command intentionally creates a grandchild process.
        let mut cmd=Command::new("sh");
        cmd.arg("-c")
            .arg("sleep 600 & echo grandchild=$!; wait")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::null());
        cmd
    };
    // Create a new process group so we can terminate child + grandchildren.
    cmd.process_group(0);
    cmd.spawn()
}
fn install_signal_listener() -> io::Result<mpsc::Receiver<i32>> {
    let mut signals=Signals::new([SIGINT, SIGTERM])?;
    let (tx,rx)=mpsc::channel();
    thread::spawn(move|| {
        if let Some(sig)=signals.forever().next() {
            let _=tx.send(sig);
        }
    });
    Ok(rx)
}

fn shutdown_with_fallback(child: &mut Child) -> io::Result<()> {
    let _=shutdown_process_group(child, Duration::from_secs(5))?;
    Ok(())
}

fn shutdown_process_group(child: &mut Child, timeout: Duration) -> io::Result<ExitStatus> {
    let pid=i32::try_from(child.id()).map_err(|_| io::Error::other("pid exceeds i32::MAX"))?;
    let pgid=Pid::from_raw(pid);

    if let Some(status)=child.try_wait()? {
        return Ok(status);
    }

    send_group_signal(pgid,Signal::SIGTERM)?;

    let deadline=Instant::now()+timeout;
    while Instant::now()<deadline {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        thread::sleep(Duration::from_millis(20));
    }

    send_group_signal(pgid, Signal::SIGKILL)?;
    child.wait()
}

fn send_group_signal(pgid: Pid, signal: Signal) -> io::Result<()> {
    match killpg(pgid, signal) {
        Ok(()) => Ok(()),
        Err(Errno::ESRCH) => Ok(()),
        Err(e) => Err(io::Error::other(format!(
            "failed to send {signal:?} to process group {pgid}: {e}"
        ))),
    }
}
