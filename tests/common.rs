use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::oneshot::{Receiver, Sender};

use log::{error, info};
use std::process::Stdio;

pub async fn start_backend(exit_status_sender: Sender<()>, kill_receiver: Receiver<()>) {
    let executable = env!("CARGO_BIN_EXE_encurtador");
    start_process(
        executable,
        &[],
        exit_status_sender,
        kill_receiver,
        "Gotham listening on",
    )
    .await;
}

pub async fn drop_database() {
    info!("Dropping database");
    let mut cmd = Command::new("diesel");
    cmd.args(vec!["database", "reset"]);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.spawn()
        .expect("Command failed to start")
        .wait()
        .await
        .expect("Command failed to run");
}

async fn start_process(
    executable: &str,
    args: &[&str],
    exit_status_sender: Sender<()>,
    kill_receiver: Receiver<()>,
    output_ready: &str,
) {
    info!("Starting: {:?}", executable);

    let mut cmd = Command::new(executable);
    cmd.env("RUST_LOG", "info");
    cmd.args(args);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.kill_on_drop(true);

    let mut child = cmd.spawn().unwrap();
    let output = child.stdout.take().unwrap();
    let mut reader = BufReader::new(output).lines();
    tokio::spawn(async move {
        match kill_receiver.await {
            Ok(_) => {
                info!("Killing process with ID: {:?}", child.id());
                child.kill().await.unwrap();
            }
            Err(_) => error!("Kill sender dropped"),
        }
        let status = child.wait().await.unwrap();
        info!("Process exit status: {}", status);
        exit_status_sender.send(()).unwrap();
    });

    while let Some(line) = reader.next_line().await.unwrap() {
        if line.contains(output_ready) {
            info!(
                "Got '{}' from process stdout, process is running",
                output_ready
            );
            break;
        }
    }
}
