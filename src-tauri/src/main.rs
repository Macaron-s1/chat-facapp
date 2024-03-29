#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
 )]

 #[macro_use] extern crate rocket;

use async_trait::async_trait;
use rocket::http::Method;
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request, Route};
use tauri::{Manager, generate_context};
use rocket::{State, Shutdown};
use rocket::fs::{relative, FileServer};
use rocket::form::Form;
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;

#[derive(Clone)]
struct WindowHandler {
    window: tauri::Window,
}

impl WindowHandler {
    fn new(window: tauri::Window) -> Self {
        Self { window }
    }
}

#[async_trait]
impl Handler for WindowHandler {
    async fn handle<'r>(&self, request: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        self.window
            .emit("from-rust", format!("message"))
            .expect("failed to emit");
            todo!()    
}
}
impl From<WindowHandler> for Vec<Route> {
    fn from(value: WindowHandler) -> Self {
        vec![Route::new(Method::Get, "/", value)]
    }
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

/// returneaza events, si fiecare este un mesaj
/// primite prin broadcast queue trimise de `post` handler.
#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

///Primire mesaj de la cineva care a trimis si trimitere catre primitori.
#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // "fail" daca nu exista niciun subscriber
    let _res = queue.send(form.into_inner());
}

#[rocket::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            let index = WindowHandler::new(window);
            // mount the rocket instance
            tauri::async_runtime::spawn(async move {
                let _rocket = rocket::build()
                .manage(channel::<Message>(1024).0)
                .mount("/", routes![post, events])
                .mount("/", FileServer::from(relative!("../dist")))
                .launch().await;
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}