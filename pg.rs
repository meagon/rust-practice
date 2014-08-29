extern crate postgres;
extern crate time;

use time::Timespec;

use postgres::{PostgresConnection, NoSsl};

struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = PostgresConnection::connect("postgres://pg@10.255.32.250/postgres",
                                           &NoSsl).unwrap();

    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    time_created    TIMESTAMP NOT NULL,
                    data            BYTEA
                  )", []).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None
    };
    conn.execute("INSERT INTO person (name, time_created, data)
                    VALUES ($1, $2, $3)",
                 [&me.name, &me.time_created, &me.data]).unwrap();

    let stmt = conn.prepare("SELECT id, name, time_created, data FROM person")
            .unwrap();
    for row in stmt.query([]).unwrap() {
        let person = Person {
            id: row.get(0u),
            name: row.get(1u),
            time_created: row.get(2u),
            data: row.get(3u)
        };
        println!("Found person {}", person.name);
    }
}

