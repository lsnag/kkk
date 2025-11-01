
#[derive(Debug)]
struct Task {
    title: String,
    description: String,
    completed: bool,
}
impl Task {
    fn new(title: &str, description: &str) -> Self {
        Task {
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}
#[derive(Debug)]
struct Employee {
    name: String,
    position: String,
    tasks: Vec<Task>,
}

impl Employee {
    fn new(name: &str, position: &str) -> Self {
        Employee {
            name: name.to_string(),
            position: position.to_string(),
            tasks: Vec::new(),
        }
    }

    fn assign_task(&mut self, new_task: Task) {
        for task in &mut self.tasks {
            if task.title == new_task.title {
                println!("Задача {} уже добавлена", new_task.title);
                return;
            }
        }
        println!("{} назначил задачу: {}", self.name, new_task.title);
        self.tasks.push(new_task);
        
    }

    fn complete_task(&mut self, title: &str) {
        for task in &mut self.tasks {
            if task.title == title {
                if !task.completed {
                    task.complete();
                    println!("{} выполнил задачу: {}", self.name, title);
                } else {
                    println!("Задача '{}' уже выполнена.", title);
                }
                return;
            }
        }
        println!("Задача '{}' не найдена.", title);
    }

    fn show_all_tasks(&self) {
        println!("Задачи, выданные {}:", self.name);
        for task in &self.tasks {
            println!(
                "- {} ({})",
                task.title,
                if task.completed { "выполнена" } else { "не выполнена" }
            );
        }
    }

    fn show_completed_tasks(&self) {
        println!("Выполненные задачи от {}:", self.name);
        for task in &self.tasks {
            if task.completed {
                println!("- {}", task.title);
            }
        }
    }

    fn show_pending_tasks(&self) {
        println!("Невыполненные задачи от {}:", self.name);
        for task in &self.tasks {
            if !task.completed {
                println!("- {}", task.title);
            }
        }
    }
}

fn main() {
    let mut oleg = Employee::new("Олег", "Менеджер");

    let task1 = Task::new("Проверить оборудование", "Осмотреть технику в цехе №3");
    let task2 = Task::new("Подготовить отчёт", "Сделать отчёт о проделанной работе за неделю");
    let task3 = Task::new("Настроить сервер", "Установить обновления безопасности");
    let task4 = Task::new("Подготовить отчёт", "Сделать отчёт о проделанной работе за неделю"); 

    oleg.assign_task(task1);
    oleg.assign_task(task2);
    oleg.assign_task(task3);
    oleg.assign_task(task4);

    oleg.show_all_tasks();

    oleg.complete_task("Проверить оборудование");
    oleg.complete_task("Настроить сервер");
    oleg.complete_task("Несуществующая задача");

    oleg.show_completed_tasks();
    oleg.show_pending_tasks();
    oleg.show_all_tasks();
}