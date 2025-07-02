#![windows_subsystem = "windows"]

use clap::Parser;

mod gui;
mod core;

#[derive(Parser)]
#[command(about = "File receiver with CLI support")]
struct Args {
    #[arg(long)]
    port: Option<String>,
    #[arg(long)]
    baud: Option<u32>,
    #[arg(long)]
    rs: Option<u8>,
    #[arg(long)]
    dir: Option<String>,
    #[arg(long)]
    start: bool,
    #[arg(long)]
    nogui: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.nogui {
        if args.start {
            let port = args.port.unwrap_or_else(|| core::find_com_port().unwrap_or("COM16".to_string()));
            let baud = args.baud.unwrap_or(921600);
            let rs_bytes = args.rs.unwrap_or(10);
            let output_dir = args.dir.unwrap_or_else(|| "received_files/".to_string());

            loop {
                match core::receive_file(&port, baud, rs_bytes, &output_dir) {
                    Ok(filename) => println!("Файл получен: {}", filename),
                    Err(e) if e.contains("Таймаут") => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    Err(e) => eprintln!("Ошибка приёма: {}", e),
                }
            }
        } else {
            eprintln!("В режиме --nogui необходимо указать --start");
        }
    } else {
        gui::ReceiverApp::run_gui(args);
    }
}