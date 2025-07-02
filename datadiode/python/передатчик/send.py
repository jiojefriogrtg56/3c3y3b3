import tkinter as tk  # Импорт библиотеки для GUI
from tkinter import filedialog, messagebox  # Импорт диалогов и сообщений
import serial  # Импорт для работы с последовательным портом
from reedsolo import RSCodec  # Импорт для коррекции ошибок Reed-Solomon
import os  # Импорт для работы с файлами и путями
import serial.tools.list_ports  # Импорт для поиска COM-портов
import argparse  # Импорт для обработки аргументов командной строки

# Определение аргументов командной строки
parser = argparse.ArgumentParser(description="File sender with CLI support")
parser.add_argument("--port", type=str, help="COM port (e.g., COM14)")
parser.add_argument("--baud", type=int, help="Baud rate (e.g., 921600)")
parser.add_argument("--rs", type=int, help="Reed-Solomon bytes (e.g., 10)")
parser.add_argument("--file", type=str, help="Path to file to send (e.g., C:/file.txt)")
parser.add_argument("--send", action="store_true", help="Automatically send the file")
parser.add_argument("--nogui", action="store_true", help="Run in background without GUI")
args = parser.parse_args()

# Инициализация основного окна
root = tk.Tk()
root.title("Передатчик")  # Установка заголовка окна
root.geometry("450x230")  # Установка размера окна
if args.nogui:
    root.withdraw()  # Скрытие окна в фоновом режиме

# Переменные для настроек
baud_rate = tk.IntVar(value=args.baud if args.baud else 921600)  # Скорость передачи
rs_value = tk.IntVar(value=args.rs if args.rs else 10)  # Количество байтов Reed-Solomon
rs = RSCodec(rs_value.get())  # Инициализация Reed-Solomon

# Поиск доступных COM-портов
def find_com_port():
    ports = serial.tools.list_ports.comports()
    for port in ports:
        if "Silicon Labs CP210x USB to UART Bridge" in port.description:
            return port.device
    return None

port = tk.StringVar(value=args.port if args.port else (find_com_port() or "COM14"))  # Выбор порта

# Установка порта
def set_port():
    port.set(port_entry.get())
    messagebox.showinfo("Успешно", f"Порт установлен на {port.get()}")

# Установка скорости с ограничением
def set_baud_rate():
    try:
        new_baud = int(baud_entry.get())
        baud_rate.set(new_baud)
        messagebox.showinfo("Успешно", f"Бод установлен на {baud_rate.get()}")
    except ValueError:
        messagebox.showerror("Ошибка", "Введите правильное значение бод")

# Установка Reed-Solomon
def set_rs_value():
    try:
        new_rs = int(rs_entry.get())
        rs_value.set(new_rs)
        global rs
        rs = RSCodec(new_rs)
        messagebox.showinfo("Успешно", f"Reed-Solomon установлен на {rs_value.get()} байт")
    except ValueError:
        messagebox.showerror("Ошибка", "Введите правильное значение байт")

# Создание интерфейса
port_frame = tk.Frame(root)
port_frame.pack(pady=5)  # Фрейм для поля порта
tk.Label(port_frame, text="Порт:").pack(side=tk.LEFT)  # Метка порта
port_entry = tk.Entry(port_frame, textvariable=port)  # Поле ввода порта
port_entry.pack(side=tk.LEFT, padx=5)
tk.Button(port_frame, text="OK", command=set_port).pack(side=tk.LEFT)  # Кнопка подтверждения

baud_frame = tk.Frame(root)
baud_frame.pack(pady=5)  # Фрейм для скорости
tk.Label(baud_frame, text="Скорость (бод):").pack(side=tk.LEFT)  # Метка скорости
baud_entry = tk.Entry(baud_frame, textvariable=tk.StringVar(value=str(baud_rate.get())))  # Поле ввода скорости
baud_entry.pack(side=tk.LEFT, padx=5)
tk.Button(baud_frame, text="OK", command=set_baud_rate).pack(side=tk.LEFT)  # Кнопка подтверждения
tk.Label(baud_frame, text="(максимум 3 млн)").pack(side=tk.LEFT, padx=5)  # Подсказка

rs_frame = tk.Frame(root)
rs_frame.pack(pady=5)  # Фрейм для Reed-Solomon
tk.Label(rs_frame, text="Reed-Solomon (байты):").pack(side=tk.LEFT)  # Метка Reed-Solomon
rs_entry = tk.Entry(rs_frame, textvariable=tk.StringVar(value=str(rs_value.get())))  # Поле ввода Reed-Solomon
rs_entry.pack(side=tk.LEFT, padx=5)
tk.Button(rs_frame, text="OK", command=set_rs_value).pack(side=tk.LEFT)  # Кнопка подтверждения
tk.Label(rs_frame, text="(максимум 254)").pack(side=tk.LEFT, padx=5)  # Подсказка

# Выбор файла
def select_file():
    file_path = filedialog.askopenfilename(filetypes=[("All files", "*.*")])
    if file_path:
        file_label.config(text=f"Выбрано: {file_path.split('/')[-1]}")
        file_label.path = file_path

# Отправка файла
def send_file():
    file_path = file_label.path
    if not file_path:
        messagebox.showerror("Ошибка", "Выберите файл")
        return
    try:
        filename = os.path.basename(file_path)  # Извлечение имени файла
        with open(file_path, 'rb') as f:
            data = f.read()  # Чтение файла
            encoded_data = rs.encode(data)  # Кодирование с Reed-Solomon
            ser = serial.Serial(port.get(), baud_rate.get(), timeout=1)  # Подключение к порту
            ser.write(len(filename.encode()).to_bytes(2, 'big'))  # Отправка длины имени
            ser.write(filename.encode())  # Отправка имени
            ser.write(encoded_data)  # Отправка данных
            ser.close()  # Закрытие порта
            messagebox.showinfo("Успешно", "Файл отправлен")  # Уведомление
            if args.nogui:  # Закрытие только в фоновом режиме
                root.after(2000, root.quit)
    except Exception as e:
        messagebox.showerror("Ошибка", f"Ошибка отправки: {e}")

file_label = tk.Label(root, text="Файл не выбран")  # Метка для выбранного файла
file_label.pack(pady=5)
file_label.path = args.file if args.file else None  # Установка пути из CLI
if args.file:
    file_label.config(text=f"Выбрано: {args.file.split('/')[-1]}")

tk.Button(root, text="Выберите файл", command=select_file).pack(pady=5)  # Кнопка выбора файла
tk.Button(root, text="Отправить файл", command=send_file).pack(pady=10)  # Кнопка отправки

# Автоматическая отправка при запуске с --send
if args.send and args.file:
    root.after(100, send_file)

root.mainloop()  # Запуск главного цикла
