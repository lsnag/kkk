#[derive(Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    description: String,
    is_completed: bool,
}

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    tasks: Vec<Task>, 
}

impl Employee {
    fn assign_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    
    fn complete_task(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.is_completed = true;
        }
    }
    
    fn get_all_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
    
    fn get_completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| task.is_completed)
            .collect()
    }
    
    fn get_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|task| !task.is_completed)
            .collect()
    }
    
}

fn main() {
    let mut oleg = Employee {
        id: 1,
        name: "Олег Менеджер".to_string(),
        tasks: Vec::new(), 
    };
    
    let task1 = Task {
        id: 1,
        title: "Подготовить отчет".to_string(),
        description: "Подготовить квартальный отчет".to_string(),
        is_completed: false,
    };
    
    let task2 = Task {
        id: 2,
        title: "Провести встречу".to_string(),
        description: "Провести планерку".to_string(),
        is_completed: false,
    };
    
    let task3 = Task {
        id: 3,
        title: "Анализ рынка".to_string(),
        description: "Проанализировать рынкок".to_string(),
        is_completed: false,
    };
    
    oleg.assign_task(task1);
    oleg.assign_task(task2);
    oleg.assign_task(task3);
    
    oleg.complete_task(1);
    
    println!("Все задачи Олега");
    for task in oleg.get_all_tasks() {
        println!("{}: {} - {}", 
            task.id, 
            task.title, 
            if task.is_completed { "Выполнено" } else { "Не выполнено" }
        );
    }
}