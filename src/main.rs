use std::{collections::HashMap, fmt::Display, io::stdin};

enum Input {
    Number(u8),
    Text(String),
}

enum Course {
    Economics,
    ComputerScience,
    Architecture,
}

struct Pupil {
    name: String,
    age: u8,
    course: Course,
    subjects: Vec<String>,
    grades: HashMap<String, Vec<Option<u8>>>,
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Course::Economics => write!(f, "Экономика"),
            Course::ComputerScience => write!(f, "ИСиП"),
            Course::Architecture => write!(f, "Строительство"),
        }
    }
}

impl Pupil {
    fn new() -> Self {
        let subjects = vec![
            "Операционные системы".to_string(),
            "Базы данных".to_string(),
            "Алгоритмы и структуры данных".to_string(),
        ];
        let mut grades = HashMap::new();
        for subject in subjects.iter() {
            grades.insert(subject.to_owned(), vec![None]);
        }

        Self {
            name: "Олег".to_string(),
            age: 17,
            course: Course::ComputerScience,
            subjects,
            grades,
        }
    }

    fn update_info(&mut self) -> Result<(), String> {
        println!("Введите новое имя:");
        let name = read_text();
        if name.trim().is_empty() {
            return Err("Имя не может быть пустым".to_string());
        }

        println!("Введите возраст:");
        let age_input = read_text();
        let age: u8 = match age_input.trim().parse() {
            Ok(v) if (10..=100).contains(&v) => v,
            _ => return Err("Возраст должен быть от 10 до 100".to_string()),
        };

        println!("Введите специальность (Экономика / ИСиП / Строительство):");
        let course = read_text();

        self.name = name;
        self.age = age;
        self.set_course(&course)?;
        self.set_subjects_by_course();
        println!(" Информация обновлена успешно!");
        Ok(())
    }
    fn set_course(&mut self, course: &str) -> Result<(), String> {
        match course.to_lowercase().as_str() {
            "экономика" => {
                self.course = Course::Economics;
                Ok(())
            }
            "исип" => {
                self.course = Course::ComputerScience;
                Ok(())
            }
            "строительство" => {
                self.course = Course::Architecture;
                Ok(())
            }
            _ => Err("Сюда не поступают".to_string()),
        }
    }
    fn set_subjects_by_course(&mut self) {
        self.subjects = match self.course {
            Course::Economics => vec![
                "Микроэкономика".to_string(),
                "Макроэкономика".to_string(),
                "Бухгалтерский учет".to_string(),
            ],
            Course::ComputerScience => vec![
                "Программирование".to_string(),
                "Базы данных".to_string(),
                "Операционные системы".to_string(),
            ],
            Course::Architecture => vec![
                "Сопромат".to_string(),
                "Архитектурное проектирование".to_string(),
                "Строительные материалы".to_string(),
            ],
        };

        self.grades.clear();
        for subject in &self.subjects {
            self.grades.insert(subject.clone(), vec![None]);
        }
    }

    fn input_grade(&mut self) -> Result<(), String> {
        if self.subjects.is_empty() {
            return Err("Нет предметов для ввода оценок".to_string());
        }

        println!("Выберите предмет для ввода оценки:");
        for (i, subject) in self.subjects.iter().enumerate() {
            println!("{}. {}", i + 1, subject);
        }

        let choice_input = read_text();
        let choice: usize = choice_input.trim().parse().unwrap_or(0);

        if choice == 0 || choice > self.subjects.len() {
            return Err("Неверный номер предмета".to_string());
        }

        let subject = &self.subjects[choice - 1];
        println!("Введите оценку (0–5) по предмету '{}':", subject);

        match input() {
            Input::Number(v) if v <= 5 => {
                self.grades.get_mut(subject).unwrap().push(Some(v));
                println!(" Оценка {} добавлена по '{}'", v, subject);
                Ok(())
            }
            _ => Err("Неверный формат оценки".to_string()),
        }
    }

    fn average_performance(&self) {
        let mut total_sum = 0;
        let mut total_count = 0;

        for grades in self.grades.values() {
            for grade in grades {
                if let Some(v) = grade {
                    total_sum += *v as u32;
                    total_count += 1;
                }
            }
        }

        if total_count == 0 {
            println!("Нет оценок для подсчета общей успеваемости.");
        } else {
            let avg = total_sum as f32 / total_count as f32;
            println!(" Общая успеваемость {}: {:.2}", self.name, avg);
        }
    }

    fn subject_performance(&self) -> Result<(), String> {
        if self.subjects.is_empty() {
            return Err("Нет предметов для анализа".to_string());
        }

        println!("Выберите предмет для просмотра успеваемости:");
        for (i, subject) in self.subjects.iter().enumerate() {
            println!("{}. {}", i + 1, subject);
        }

        let choice_input = read_text();
        let choice: usize = choice_input.trim().parse().unwrap_or(0);

        if choice == 0 || choice > self.subjects.len() {
            return Err("Неверный номер предмета".to_string());
        }

        let subject = &self.subjects[choice - 1];
        
            match self.grades.get(subject) {
        Some(grades) => {
            let mut total_sum = 0u32;
            let mut total_count = 0;

            for grade in grades {
                if let Some(v) = grade {
                    total_sum += *v as u32;
                    total_count += 1;
                }
            }

            if total_count == 0 {
                Err(format!("Оценок по предмету '{}' нет", subject))
            } else {
                let avg = total_sum as f32 / total_count as f32;
                println!("Средняя оценка по '{}': {:.2}", subject, avg);
                Ok(())
            }
        }
        None => Err(format!("Предмет '{}' не найден", subject)),
    }
    }

    fn list_subjects(&self) {
        let mut message = format!(
            "{} со специальности {} изучает следующие предметы:",
            self.name, self.course
        );
        for subject in self.subjects.iter() {
            message.push_str(&format!(" {},", subject));
        }
        message.remove(message.len() - 1);
        println!("{message}");
    }
}

fn main() {
    let mut person = Pupil::new();

    loop {
        println!("\n Выберите задание:");
        println!("1. Изменить информацию об Олеге");
        println!("2. Показать список предметов");
        println!("3. Ввести оценку");
        println!("4. Показать общую успеваемость");
        println!("5. Показать успеваемость по предмету");
        println!("0. Выйти");

        let choice = read_text();

        match choice.trim() {
            "1" => {
                if let Err(e) = person.update_info() {
                    println!("Ошибка: {}", e);
                }
            }
            "2" => person.list_subjects(),
            "3" => {
                if let Err(e) = person.input_grade() {
                    println!("Ошибка: {}", e);
                }
            }
            "4" => person.average_performance(),
            "5" => {
                if let Err(e) = person.subject_performance() {
                    println!("Ошибка: {}", e);
                }
            }
            "0" => {
                println!("Программа завершена.");
                break;
            }
            _ => println!("Неверный выбор, попробуйте снова."),
        }
    }
}

fn input() -> Input {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    match trimmed.parse::<u8>() {
        Ok(num) => Input::Number(num),
        Err(_) => Input::Text(trimmed.to_string()),
    }
}

fn read_text() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
