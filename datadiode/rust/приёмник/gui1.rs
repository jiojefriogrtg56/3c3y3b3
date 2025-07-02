use iced::{Application, Settings, Element, Command, Subscription};
use rfd::FileDialog;
use super::Args;
use super::core::receive_file;
use std::time::Instant;

#[derive(Default)]
pub struct ReceiverApp {
    port: String,
    baud: u32,
    rs: u8,
    output_dir: String,
    status: String,
    port_input: String,
    baud_input: String,
    rs_input: String,
    status_opacity: f32,
    status_set_time: Option<Instant>,
    receiving: bool,
}

impl ReceiverApp {
    pub fn run_gui(args: Args) {
        let initial_port = args.port.unwrap_or_else(|| super::core::find_com_port().unwrap_or("COM16".to_string()));
        let initial_baud = args.baud.unwrap_or(921600);
        let initial_rs = args.rs.unwrap_or(10);
        let initial_dir = args.dir.unwrap_or_else(|| "received_files/".to_string());

        let settings = Settings {
            window: iced::window::Settings {
                size: (650, 300),
                resizable: true,
                ..iced::window::Settings::default()
            },
            flags: ReceiverApp {
                port: initial_port,
                baud: initial_baud,
                rs: initial_rs,
                output_dir: initial_dir,
                status: String::new(),
                port_input: String::new(),
                baud_input: String::new(),
                rs_input: String::new(),
                status_opacity: 1.0,
                status_set_time: None,
                receiving: false,
            },
            ..Settings::default()
        };
        ReceiverApp::run(settings).unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPort,
    SetBaud,
    SetRs,
    PortChanged(String),
    BaudChanged(String),
    RsChanged(String),
    SelectDir,
    StartReceiving,
    StopReceiving,
    FileReceived(Result<String, String>),
    Tick,
}

impl Application for ReceiverApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ReceiverApp;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (flags, Command::none())
    }

    fn title(&self) -> String {
        "Приёмник".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SetPort => {
                self.port = self.port_input.clone();
                self.status = format!("Порт установлен на {}", self.port);
                self.status_opacity = 1.0;
                self.status_set_time = Some(Instant::now());
            }
            Message::SetBaud => {
                if let Ok(baud) = self.baud_input.parse::<u32>() {
                    self.baud = baud;
                    self.status = format!("Скорость установлена на {}", self.baud);
                } else {
                    self.status = "Ошибка: некорректная скорость".to_string();
                }
                self.status_opacity = 1.0;
                self.status_set_time = Some(Instant::now());
            }
            Message::SetRs => {
                if let Ok(rs) = self.rs_input.parse::<u8>() {
                    if rs <= 254 {
                        self.rs = rs;
                        self.status = format!("Reed-Solomon установлен на {} байт", self.rs);
                    } else {
                        self.status = "Ошибка: RS должен быть <= 254".to_string();
                    }
                } else {
                    self.status = "Ошибка: некорректное значение RS".to_string();
                }
                self.status_opacity = 1.0;
                self.status_set_time = Some(Instant::now());
            }
            Message::PortChanged(value) => self.port_input = value,
            Message::BaudChanged(value) => self.baud_input = value,
            Message::RsChanged(value) => self.rs_input = value,
            Message::SelectDir => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.output_dir = dir.to_string_lossy().into_owned() + "/";
                    self.status = format!("Директория установлена: {}", self.output_dir);
                    self.status_opacity = 1.0;
                    self.status_set_time = Some(Instant::now());
                }
            }
            Message::StartReceiving => {
                self.receiving = true;
                self.status = "Слушаю порт...".to_string();
                let port = self.port.clone();
                let baud = self.baud;
                let rs = self.rs;
                let output_dir = self.output_dir.clone();
                return Command::perform(
                    async move { receive_file(&port, baud, rs, &output_dir) },
                    Message::FileReceived,
                );
            }
            Message::StopReceiving => {
                self.receiving = false;
                self.status = "Приём остановлен".to_string();
            }
            Message::FileReceived(result) => {
                self.status = match result {
                    Ok(filename) => format!("Файл получен: {}", filename),
                    Err(e) if e.contains("Таймаут") => "Слушаю порт...".to_string(),
                    Err(e) => format!("Ошибка приёма: {}", e),
                };
                self.status_opacity = 1.0;
                self.status_set_time = Some(Instant::now());
                if self.receiving {
                    let port = self.port.clone();
                    let baud = self.baud;
                    let rs = self.rs;
                    let output_dir = self.output_dir.clone();
                    return Command::perform(
                        async move { receive_file(&port, baud, rs, &output_dir) },
                        Message::FileReceived,
                    );
                }
            }
            Message::Tick => {
                if let Some(set_time) = self.status_set_time {
                    let elapsed = set_time.elapsed().as_secs_f32();
                    if elapsed >= 1.5 {
                        self.status_opacity -= 0.1;
                        if self.status_opacity < 0.01 {
                            self.status = if self.receiving { "Слушаю порт...".to_string() } else { String::new() };
                            self.status_set_time = None;
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.status_set_time.is_some() {
            iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        use iced::widget::{button, column, row, text, text_input, container};

        column![
            row![
                text("Порт:"),
                text_input("", &self.port_input).on_input(Message::PortChanged),
                button("OK").on_press(Message::SetPort),
            ].spacing(5),
            row![
                text("Скорость (бод):"),
                text_input("", &self.baud_input).on_input(Message::BaudChanged),
                button("OK").on_press(Message::SetBaud),
                text("(максимум 3 млн)")
            ].spacing(5),
            row![
                text("Reed-Solomon (байты):"),
                text_input("", &self.rs_input).on_input(Message::RsChanged),
                button("OK").on_press(Message::SetRs),
                text("(максимум 254)")
            ].spacing(5),
            row![
                text("Путь сохранения:"),
                text(&self.output_dir),
                button("Выбрать").on_press(Message::SelectDir),
            ].spacing(5),
            container(
                if self.receiving {
                    button("Прекратить прослушивание").on_press(Message::StopReceiving)
                } else {
                    button("Начать приём").on_press(Message::StartReceiving)
                }
            )
            .width(iced::Length::Fill)
            .center_x(),
            if !self.status.is_empty() {
                text(&self.status).style(iced::theme::Text::Color(iced::Color {
                    a: self.status_opacity,
                    ..iced::Color::BLACK
                }))
            } else {
                text("")
            },
        ]
        .spacing(10)
        .padding(20)
        .align_items(iced::Alignment::Center)
        .into()
    }
}