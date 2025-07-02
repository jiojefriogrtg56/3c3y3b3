

use clap::Parser;

mod gui;
mod core;

#[derive(Parser)]
#[command(about = "File sender with CLI support")]
struct Args {
    #[arg(long)]
    port: Option<String>,
    #[arg(long)]
    baud: Option<u32>,
    #[arg(long)]
    rs: Option<u8>,
    #[arg(long)]
    file: Option<String>,
    #[arg(long)]
    send: bool,
    #[arg(long)]
    nogui: bool,
}

fn main() {
    let args = Args::parse();

    if args.nogui {
        if args.send && args.file.is_some() {
            let port = args.port.unwrap_or_else(|| core::find_com_port().unwrap_or("COM14".to_string()));
            let baud = args.baud.unwrap_or(921600);
            let rs_bytes = args.rs.unwrap_or(10);
            let file_path = args.file.unwrap();

            match core::send_file(&port, baud, rs_bytes, &file_path) {
                Ok(_) => println!("Файл успешно отправлен"),
                Err(e) => eprintln!("Ошибка отправки файла: {}", e),
            }
        } else {
            eprintln!("В режиме --nogui необходимо указать --send и --file");
        }
    } else {
        gui::SenderApp::run_gui(args);
    }
}