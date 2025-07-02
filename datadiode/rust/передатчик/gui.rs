use iced::{Application, Settings, Element, Command, Subscription};
use rfd::FileDialog;
use super::Args;
use std::time::Instant;

#[derive(Default)]
pub struct SenderApp {
    port: String,
    baud: u32,
    rs: u8,
    file_path: Option<String>,
    status: String,
    port_input: String,
    baud_input: String,
    rs_input: String,
    status_opacity: f32,
    status_set_time: Option<Instant>,
}

impl SenderApp {
    pub fn run_gui(args: Args) {
        let initial_port = args.port.unwrap_or_else(|| super::core::find_com_port().unwrap_or("COM14".to_string()));
        let initial_baud = args.baud.unwrap_or(921600);
        let initial_rs = args.rs.unwrap_or(10);
        let initial_file = args.file;

        let settings = Settings {
            window: iced::window::Settings {
                size: (650, 300),
                resizable: true,
                ..iced::window::Settings::default()
            },
            flags: SenderApp {
                port: initial_port,
                baud: initial_baud,
                rs: initial_rs,
                file_path: initial_file,
                status: String::new(),
                port_input: String::new(),
                baud_input: String::new(),
                rs_input: String::new(),
                status_opacity: 1.0,
                status_set_time: None,
            },
            ..Settings::default()
        };
        SenderApp::run(settings).unwrap();
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
    SelectFile,
    SendFile,
    FileSent(Result<(), String>),
    Tick,
}

impl Application for SenderApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = SenderApp;

fn new(flags: Self::Flags) -> (Self, Command<Message>) {
    let app = flags;
    if app.file_path.is_some() {
        let port = app.port.clone();
        let baud = app.baud;
        let rs = app.rs;
        let file_path = app.file_path.clone().unwrap();
        return (
            app,
            Command::perform(
                async move { super::core::send_file(&port, baud, rs, &file_path) },
                Message::FileSent,
            ),
        );
    }
    (app, Command::none())
}

    fn title(&self) -> String {
        "Передатчик".to_string()
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
            Message::SelectFile => {
                if let Some(file) = FileDialog::new().pick_file() {
                    self.file_path = Some(file.to_string_lossy().to_string());
                    self.status = format!(
                        "Выбрано: {}",
                        file.file_name().unwrap_or_default().to_string_lossy()
                    );
                    self.status_opacity = 1.0;
                    self.status_set_time = Some(Instant::now());
                }
            }
            Message::SendFile => {
    if let Some(file_path) = &self.file_path {
        println!("Начинаю отправку файла: {}", file_path);
        let port = self.port.clone();
        let baud = self.baud;
        let rs = self.rs;
        let file_path = file_path.clone();
        return Command::perform(
            async move {
                println!("Асинхронная задача запущена");
                let result = super::core::send_file(&port, baud, rs, &file_path);
                println!("Асинхронная задача завершена с результатом: {:?}", result);
                result
            },
            Message::FileSent,
        );
    } else {
        self.status = "Ошибка: выберите файл".to_string();
        self.status_opacity = 1.0;
        self.status_set_time = Some(Instant::now());
        println!("Ошибка: файл не выбран");
    }
}
            Message::FileSent(result) => {
    match &result {
        Ok(()) => println!("Файл успешно отправлен"),
        Err(e) => println!("Ошибка отправки: {}", e),
    }
    self.status = match result {
        Ok(()) => "Файл успешно отправлен".to_string(),
        Err(e) => format!("Ошибка отправки: {}", e),
    };
    self.status_opacity = 1.0;
    self.status_set_time = Some(Instant::now());
}
            Message::Tick => {
                if let Some(set_time) = self.status_set_time {
                    let elapsed = set_time.elapsed().as_secs_f32();
                    if elapsed >= 1.5 {
                        self.status_opacity -= 0.1;
                        if self.status_opacity < 0.01 {
                            self.status = String::new();
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
            text(
                self.file_path
                    .as_ref()
                    .map_or("Файл не выбран".to_string(), |f| {
                        format!("Выбрано: {}", std::path::Path::new(f).file_name().unwrap_or_default().to_string_lossy())
                    })
            ),
            container(
                button("Выберите файл").on_press(Message::SelectFile)
            )
            .width(iced::Length::Fill)
            .center_x(),
            container(
                button("Отправить файл").on_press(Message::SendFile)
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