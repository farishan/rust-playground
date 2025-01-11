use rusqlite::{Connection, Result};
use std::io;

const DB_PATH: &str = "./test.db";

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

fn main() -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    println!("Using db: {:?}", DB_PATH);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;

    let mut data: Vec<u8> = Vec::new();
    data.push(1);

    let mut name: String = Default::default();

    println!("Your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let person = Person {
        id: 0,
        name: name.trim().to_string(),
    };

    println!("{:?}", person.id);

    conn.execute("INSERT INTO person (name) VALUES (?1)", (person.name,))?;

    let mut stmt = conn.prepare("SELECT id, name FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person);
    }

    Ok(())
}
