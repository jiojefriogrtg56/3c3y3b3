import tkinter as tk  # Импорт библиотеки для GUI
from tkinter import messagebox, filedialog  # Импорт диалогов и сообщений
import serial  # Импорт для работы с последовательным портом
from reedsolo import RSCodec  # Импорт для коррекции ошибок Reed-Solomon
import os  # Импорт для работы с файлами и путями
import time  # Импорт для временных меток
import serial.tools.list_ports  # Импорт для поиска COM-портов
import threading  # Импорт для работы в потоках
import argparse  # Импорт для обработки аргументов командной строки

# Определение аргументов командной строки
parser = argparse.ArgumentParser(description="File receiver with CLI support")
parser.add_argument("--port", type=str, help="COM port (e.g., COM16)")
parser.add_argument("--baud", type=int, help="Baud rate (e.g., 921600)")
parser.add_argument("--rs", type=int, help="Reed-Solomon bytes (e.g., 10)")
parser.add_argument("--dir", type=str, help="Directory to save files (e.g., D:/infodiode)")
parser.add_argument("--start", action="store_true", help="Start listening automatically")
parser.add_argument("--nogui", action="store_true", help="Run in background without GUI")
args = parser.parse_args()

# Инициализация основного окна
root = tk.Tk()
root.title("Приёмник")  # Установка заголовка окна
root.geometry("450x250")  # Установка размера окна
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

port = tk.StringVar(value=args.port if args.port else (find_com_port() or "COM16"))  # Выбор порта
output_dir = tk.StringVar(value=args.dir if args.dir else "D:/infodiode/")  # Путь сохранения

# Установка порта
def set_port():
    port.set(port_entry.get())
    messagebox.showinfo("Успешно", f"Порт установлен на {port.get()}")

# Установка скорости
def set_baud_rate():
    try:
        new_baud = int(baud_entry.get())
        baud_rate.set(new_baud)
        messagebox.showinfo("Успешно", f"Скорость установлена на {baud_rate.get()}")
    except ValueError:
        messagebox.showerror("Ошибка", "Введите корректную скорость!")

# Установка Reed-Solomon
def set_rs_value():
    try:
        new_rs = int(rs_entry.get())
        rs_value.set(new_rs)
        global rs
        rs = RSCodec(new_rs)
        messagebox.showinfo("Успешно", f"Reed-Solomon установлен на {rs_value.get()} байт")
    except ValueError:
        messagebox.showerror("Ошибка", "Введите корректное число!")

# Выбор папки для сохранения
def select_folder():
    folder = filedialog.askdirectory(initialdir=output_dir.get())
    if folder:
        output_dir.set(folder)

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
tk.Label(rs_frame, text="Reed-Solomon (байт):").pack(side=tk.LEFT)  # Метка Reed-Solomon
rs_entry = tk.Entry(rs_frame, textvariable=tk.StringVar(value=str(rs_value.get())))  # Поле ввода Reed-Solomon
rs_entry.pack(side=tk.LEFT, padx=5)
tk.Button(rs_frame, text="OK", command=set_rs_value).pack(side=tk.LEFT)  # Кнопка подтверждения
tk.Label(rs_frame, text="(максимум 254)").pack(side=tk.LEFT, padx=5)  # Подсказка

path_frame = tk.Frame(root)
path_frame.pack(pady=5)  # Фрейм для пути сохранения
tk.Label(path_frame, text="Путь сохранения файлов:").pack(side=tk.LEFT)  # Метка пути
tk.Entry(path_frame, textvariable=output_dir, state='readonly').pack(side=tk.LEFT, padx=5)  # Поле пути
tk.Button(root, text="Выбрать папку", command=select_folder).pack(pady=5)  # Кнопка выбора папки

# Приём файла
def receive_file():
    ser = None
    file_received = False
    try:
        ser = serial.Serial(port.get(), baud_rate.get(), timeout=1)  # Подключение к порту
        ser.flushInput()  # Очистка буфера
        while running.get() and not root.quit_triggered and not file_received:
            data = ser.read(2)  # Чтение длины имени
            if data and int.from_bytes(data, 'big') > 0:
                name_len = int.from_bytes(data, 'big')  # Извлечение длины имени
                filename = ser.read(name_len).decode()  # Чтение имени файла
                if len(filename) > 0:
                    timestamp = time.strftime("%Y%m%d_%H%M%S")  # Временная метка
                    temp_file = f"{output_dir.get()}received_{timestamp}.bin"  # Временный файл
                    with open(temp_file, "wb") as f:
                        while True:
                            chunk = ser.read(1024)  # Чтение данных порциями
                            if not chunk:
                                break
                            f.write(chunk)
                    with open(temp_file, "rb") as f:
                        raw_data = f.read()  # Чтение данных для декодирования
                        decoded_data = rs.decode(raw_data)  # Декодирование Reed-Solomon
                    with open(f"{output_dir.get()}decoded_{filename}", "wb") as f:
                        f.write(decoded_data[0])  # Сохранение декодированного файла
                    os.remove(temp_file)  # Удаление временного файла
                    file_received = True  # Флаг успешного приёма
                    root.after(1000, lambda fn=filename, t=timestamp: messagebox.showinfo("Успешно", f"Файл получен: decoded_{t}_{fn}"))  # Уведомление
                    if args.nogui:  # Закрытие только в фоновом режиме
                        root.after(10000, root.quit)  # Закрытие через 10 секунд
    except Exception as e:
        if not root.quit_triggered:
            root.after(1000, lambda err=str(e): messagebox.showerror("Ошибка", f"Failed: {err}"))  # Обработка ошибок
    finally:
        if ser and ser.is_open:
            ser.close()  # Закрытие порта

running = tk.BooleanVar(value=False)  # Переменная для управления циклом
root.quit_triggered = False  # Флаг для принудительного завершения

# Обработка закрытия окна
def on_closing():
    root.quit_triggered = True
    running.set(False)
    root.destroy()

# Переключение прослушивания
def toggle_listening():
    if not running.get():
        running.set(True)
        btn_toggle.config(text="Прекратить прослушивание")
        threading.Thread(target=receive_file, daemon=True).start()  # Запуск потока приёма
    else:
        running.set(False)
        btn_toggle.config(text="Запуск прослушивания")

# Автоматический запуск, если указан --start
if args.start:
    root.after(100, toggle_listening)

btn_toggle = tk.Button(root, text="Запуск", command=toggle_listening)  # Кнопка переключения
btn_toggle.pack(pady=10)
root.protocol("WM_DELETE_WINDOW", on_closing)  # Обработчик закрытия окна

root.mainloop()  # Запуск главного цикла
