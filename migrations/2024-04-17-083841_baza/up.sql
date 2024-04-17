CREATE TABLE baza (
    id integer primary key AUTOINCREMENT,
    name varchar not null,
    email varchar not null,
    created_at timestamp not null default current_timestamp
)