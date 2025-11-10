use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Mutex;
use chrono::{Utc, DateTime};

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    title: String,
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

struct AppState {
    tasks: Mutex<Vec<Task>>,
}
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();
    HttpResponse::Ok().json(tasks.clone())
}
async fn create_task(data: web::Data<AppState>, new_task: web::Json<Task>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let task = Task {
        id: Uuid::new_v4(),
        title: new_task.title.clone(),
        description: new_task.description.clone(),
        completed: false,
        created_at: Utc::now(),
    };
    tasks.push(task.clone());
    HttpResponse::Created().json(task)
}
async fn update_task(data: web::Data<AppState>, task_id: web::Path<Uuid>, updated_task: web::Json<Task>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == *task_id) {
        task.title = updated_task.title.clone();
        task.description = updated_task.description.clone();
        task.completed = updated_task.completed;
        return HttpResponse::Ok().json(task.clone());
    }
    HttpResponse::NotFound().body("Task not found")
}
async fn delete_task(data: web::Data<AppState>, task_id: web::Path<Uuid>) -> impl Responder {
    let mut tasks = data.tasks.lock().unwrap();
    let len_before = tasks.len();
    tasks.retain(|t| t.id != *task_id);
    if tasks.len() < len_before {
        return HttpResponse::Ok().body("Task deleted");
    }
    HttpResponse::NotFound().body("Task not found")
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tasks: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_origin().allow_any_method().allow_any_header();
        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::put().to(update_task))
            .route("/tasks/{id}", web::delete().to(delete_task))
    }) .bind("127.0.0.1:8080") ?
    .run().await.inspect_err(|e| eprintln!("Server error: {}", e))
}