// Импортируем стандартную библиотеку ввода-вывода
use std::io::{self, Write};

// Функция для чтения ввода пользователя
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка при чтении строки");
    input.trim().to_string()
}

// Функция для отображения главного меню и получения выбора пользователя
fn main_menu() -> u32 {
    println!("To-Do List");
    println!("1. Добавить задачу");
    println!("2. Показать задачи");
    println!("3. Удалить задачу");
    println!("4. Выйти");
    print!("Введите ваш выбор: ");
    io::stdout().flush().expect("Ошибка при выводе");

    // Считываем ввод пользователя и возвращаем его как целое число
    read_input().trim().parse().unwrap_or(0)
}

// Главная функция
fn main() {
    // Инициализируем пустой вектор для хранения задач
    let mut tasks: Vec<String> = Vec::new();

    // Бесконечный цикл, тоже самое, что и while true
    loop {
        // Оператор выбора
        // Вызываем функцию для отображения главного меню и получения выбора пользователя
        match main_menu() {
            1 => {
                // Добавление новой задачи
                println!("Введите новую задачу:");
                let task = read_input();
                tasks.push(task);
            }
            2 => {
                // Показ всех задач
                println!("Список задач:");
                for (i, task) in tasks.iter().enumerate()
                    println!("{}. {}", i + 1, task);
            }
            3 => {
                // Удаление задачи
                println!("Введите номер задачи для удаления:");
                let num: usize = read_input().trim().parse().expect("Некорректный номер");
                if num > 0 && num <= tasks.len()
                    tasks.remove(num - 1);
                else
                    println!("Некорректный номер задачи");
            }
            4 => {
                // Выход из программы
                println!("Выход из программы...");
                break;
            }
            _ => {
                // Другое значение
                println!("Некорректный вариант, попробуйте снова.");
            }
        }
    }
}
