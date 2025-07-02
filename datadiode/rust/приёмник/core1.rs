use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use serialport::SerialPort;
use reed_solomon::Decoder;
use chrono::Utc;

pub fn find_com_port() -> Option<String> {
    serialport::available_ports()
        .ok()?
        .into_iter()
        .find(|p| {
            if let serialport::SerialPortType::UsbPort(info) = &p.port_type {
                info.vid == 0x10C4 && info.pid == 0xEA60 // CP2102
            } else {
                false
            }
        })
        .map(|p| p.port_name)
}

pub fn receive_file(port: &str, baud: u32, rs_bytes: u8, output_dir: &str) -> Result<String, String> {
    println!("Открываем порт: {} с baud {}", port, baud);
    let mut ser = serialport::new(port, baud)
        .timeout(std::time::Duration::from_secs(2))
        .open()
        .map_err(|e| format!("Ошибка открытия порта: {}", e))?;

    let mut len_buf = [0u8; 2];
    ser.read_exact(&mut len_buf).map_err(|e| format!("Ошибка чтения длины имени: {}", e))?;
    let name_len = u16::from_be_bytes(len_buf) as usize;
    println!("Длина имени файла: {} байт", name_len);

    let mut filename_buf = vec![0u8; name_len];
    ser.read_exact(&mut filename_buf).map_err(|e| format!("Ошибка чтения имени файла: {}", e))?;
    let filename = String::from_utf8_lossy(&filename_buf).into_owned();
    println!("Получено имя файла: {}", filename);

    let mut data_len_buf = [0u8; 4];
    ser.read_exact(&mut data_len_buf).map_err(|e| format!("Ошибка чтения длины данных: {}", e))?;
    let data_len = u32::from_be_bytes(data_len_buf) as usize;
    println!("Длина данных: {} байт", data_len);

    let mut raw_data = vec![0u8; data_len];
    ser.read_exact(&mut raw_data).map_err(|e| format!("Ошибка чтения данных: {}", e))?;
    println!("Данные получены");

    let decoder = Decoder::new(rs_bytes as usize);
    let decoded_data = decoder.correct(&raw_data, None)
        .map_err(|e| format!("Ошибка декодирования: {:?}", e))?;
    println!("Данные декодированы, размер: {} байт", decoded_data.len());

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let output_file = format!("{}/decoded_{}_{}", output_dir, timestamp, filename);
    println!("Сохраняем файл: {}", output_file);

    create_dir_all(output_dir).map_err(|e| format!("Ошибка создания директории: {}", e))?;
    File::create(&output_file)
        .map_err(|e| format!("Ошибка создания файла: {}", e))?
        .write_all(&decoded_data)
        .map_err(|e| format!("Ошибка записи файла: {}", e))?;

    println!("Файл успешно сохранён");
    Ok(output_file)
}