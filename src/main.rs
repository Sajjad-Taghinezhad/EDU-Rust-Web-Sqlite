use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rusqlite::params;
use rusqlite::Connection;
use std::convert::Infallible;
#[derive(Debug)]
struct Users {
    username: String,
    password: String,
    email: String,
}
#[derive(Debug)]
struct Data {
    username: String,
    email: String,
}

async fn hello(_: Request<Body>, text: String) -> Result<Response<Body>, Infallible> {
    let y = &text.to_string();
    let b = String::from(y);
    Ok(Response::new(Body::from(b)))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id                INTEGER PRIMARY KEY AUTOINCREMENT,
                  username          TEXT NOT NULL,
                  password          TEXT NOT NULL,
                  email             TEXT NOT NULL
                  )",
        params![],
    )?;
    let me = Users {
        username: "Sajjad".to_string(),
        password: "Sajj@dpassword".to_string(),
        email: "sajjad@gmail.com".to_string(),
    };
    conn.execute(
        "INSERT INTO users (username ,password ,email )
                  VALUES (?1, ?2, ?3)",
        params![me.username, me.password, me.email],
    )?;

    let mut stmt = conn.prepare("SELECT username,email FROM users")?;
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Data {
            username: row.get(0)?,
            email: row.get(1)?,
        })
    })?;
    let mut names = Vec::new();
    for name_result in person_iter {
        names.push(name_result?);
    }

    let email: String = names[0].email.to_string();


    let make_svc = make_service_fn(move |_| {
        let email = email.clone();

        async move {
            let _email = email.clone();
            Ok::<_, Infallible>(service_fn(move |x| hello(x, email.clone())))
        }
    });
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
