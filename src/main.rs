use chrono::Utc;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePool, types::chrono, Row, Sqlite};
use std::{env, str::FromStr};

struct Todo {
    id: i64,
    content: String,
    is_done: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

enum Command {
    Create,
    ReadAll,
    Read,
    Update,
    Delete,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Command::Create),
            "readall" => Ok(Command::ReadAll),
            "read" => Ok(Command::Read),
            "update" => Ok(Command::Update),
            "delete" => Ok(Command::Delete),
            _ => Err(()),
        }
    }
}

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await.unwrap();
    create_todo_table(&pool).await?;
    match args.len() {
        1 => {
            println!("this is todo cli in rust")
        }
        2 => {
            let args_1 = Command::from_str(&args[1]);
            match args_1 {
                Ok(Command::Create) => {
                    println!("Create");
                    let content = String::from("test todo");
                    create(&pool, &content).await?;
                }
                Ok(Command::ReadAll) => {
                    read_all(&pool).await?;
                }
                Ok(Command::Read) => {
                    println!("Read")
                }
                Ok(Command::Update) => {
                    println!("Update")
                }
                Ok(Command::Delete) => {
                    println!("Delete")
                }
                Err(()) => {
                    println!("Error, command not found!")
                }
            }
        }
        _ => {
            println!("show help here")
        }
    }
    Ok(())
}

async fn create_todo_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let _result = sqlx::query(
        "
create table if not exists todo(
  id integer primary key autoincrement not null,
  content text not null,
  isDone boolean not null,
  createdAt datetime not null default current_time,
  updatedAt datetime not null default current_time,
  doneAt datetime
);
",
    )
    .execute(pool)
    .await
    .unwrap();
    Ok(())
}

async fn read_all(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        "
select Id, Content, IsDone from todo;
",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("content"));
    }
    Ok(())
}

async fn create(pool: &SqlitePool, content: &String) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        "
insert into todo (content, isDone, createdAt, updatedAt) values (?, ?, ?, ?)
",
    )
    .bind(content)
    .bind(false)
    .bind(Utc::now())
    .bind(Utc::now())
    .execute(pool)
    .await
    .unwrap();
    println!("{:?}", result);
    Ok(())
}
