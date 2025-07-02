use std::fs::File;
use std::io::Read;
#[allow(unused_imports)]
use serialport::SerialPort;
use reed_solomon::Encoder;

pub fn find_com_port() -> Option<String> {
    serialport::available_ports()
        .ok()?
        .into_iter()
        .find(|p| {
            if let serialport::SerialPortType::UsbPort(info) = &p.port_type {
                info.vid == 0x10C4 && info.pid == 0xEA60
            } else {
                false
            }
        })
        .map(|p| p.port_name)
}

pub fn send_file(port: &str, baud: u32, rs_bytes: u8, file_path: &str) -> Result<(), String> {
    println!("Открываем файл: {}", file_path);
    let mut file = File::open(file_path).map_err(|e| format!("Не удалось открыть файл: {}", e))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).map_err(|e| format!("Ошибка чтения файла: {}", e))?;
    println!("Файл прочитан, размер: {} байт", data.len());

    let filename = std::path::Path::new(file_path)
        .file_name()
        .ok_or("Неверный путь к файлу")?
        .to_str()
        .ok_or("Некорректное имя файла")?;
    let filename_bytes = filename.as_bytes();
    let filename_len = filename_bytes.len() as u16;
    println!("Имя файла: {}, длина: {}", filename, filename_len);

    let encoder = Encoder::new(rs_bytes as usize);
    let encoded_data = encoder.encode(&data);
    println!("Данные закодированы, размер: {} байт", encoded_data.len());

    println!("Открываем порт: {} с baud {}", port, baud);
    let mut ser = serialport::new(port, baud)
        .timeout(std::time::Duration::from_secs(2))
        .open()
        .map_err(|e| format!("Ошибка открытия порта: {}", e))?;
    println!("Порт успешно открыт: {}", port);

    println!("Отправка длины имени файла: {} байт", filename_len);
    ser.write_all(&filename_len.to_be_bytes()).map_err(|e| format!("Ошибка отправки длины имени: {}", e))?;
    println!("Длина имени файла отправлена");

    println!("Отправка имени файла: {}", filename);
    ser.write_all(filename_bytes).map_err(|e| format!("Ошибка отправки имени файла: {}", e))?;
    println!("Имя файла отправлено");

    println!("Отправка длины данных: {} байт", encoded_data.len());
    ser.write_all(&(encoded_data.len() as u32).to_be_bytes()).map_err(|e| format!("Ошибка отправки длины данных: {}", e))?;
    println!("Длина данных отправлена");

    println!("Отправка данных...");
    ser.write_all(&encoded_data).map_err(|e| format!("Ошибка отправки данных: {}", e))?;
    println!("Данные успешно отправлены");

    Ok(())
}