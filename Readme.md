
//curl 127.0.0.1:8000/baza -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
//curl -X POST -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' http://127.0.0.1:8000/baza
//curl 127.0.0.1:8000/baza -X POST -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
//> curl 127.0.0.1:8000/baza/1 -X PUT -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
//[{"email":"sadagatasgarov@gmil.com","id":1,"name":"Sada2 Asga2"}]%  


// Installed package `diesel_cli v2.1.1` (executable `diesel`)
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> sudo apt install libsqlite3-dev           
// Reading package lists... Done
// Building dependency tree... Done
// Reading state information... Done
// libsqlite3-dev is already the newest version (3.37.2-2ubuntu0.3).
// 0 upgraded, 0 newly installed, 0 to remove and 47 not upgraded.
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel
// Usage: diesel [OPTIONS] <COMMAND>

// Commands:
//   migration     A group of commands for generating, running, and reverting migrations.
//   setup         Creates the migrations directory, creates the database specified in your DATABASE_URL, and runs existing migrations.
//   database      A group of commands for setting up and resetting your database.
//   completions   Generate shell completion scripts for the diesel command.
//   print-schema  Print table definitions for database schema.
//   help          Print this message or the help of the given subcommand(s)

// Options:
//       --database-url <DATABASE_URL>  Specifies the database URL to connect to. Falls back to the DATABASE_URL environment variable if unspecified.
//       --config-file <CONFIG_FILE>    The location of the configuration file to use. Falls back to the `DIESEL_CONFIG_FILE` environment variable if unspecified. Defaults to `diesel.toml` in your project root. See diesel.rs/guides/configuring-diesel-cli for documentation on this file.
//       --locked-schema                Require that the schema file is up to date.
//   -h, --help                         Print help (see more with '--help')
//   -V, --version                      Print version

// You can also run `diesel SUBCOMMAND -h` to get more information about that subcommand.
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel setup
// Creating migrations directory at: /home/sada/RUST/rust-crud-api1/migrations
// The --database-url argument must be passed, or the DATABASE_URL environment variable must be set.
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel setup --database-url database.sqlite
// Creating database: database.sqlite
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel setup --database-url ./database.sqlite
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel migration                             
// A group of commands for generating, running, and reverting migrations.

// Usage: diesel migration [OPTIONS] <COMMAND>

// Commands:
//   run       Runs all pending migrations.
//   revert    Reverts the specified migrations.
//   redo      Reverts and re-runs the latest migration. Useful for testing that a migration can in fact be reverted.
//   list      Lists all available migrations, marking those that have been applied.
//   pending   Returns true if there are any pending migrations.
//   generate  Generate a new migration with the given name, and the current timestamp as the version.
//   help      Print this message or the help of the given subcommand(s)

// Options:
//       --database-url <DATABASE_URL>
//           Specifies the database URL to connect to. Falls back to the DATABASE_URL environment variable if unspecified.
//       --migration-dir <MIGRATION_DIRECTORY>
//           The location of your migration directory. By default this will look for a directory called `migrations` in the current directory and its parents.
//       --config-file <CONFIG_FILE>
//           The location of the configuration file to use. Falls back to the `DIESEL_CONFIG_FILE` environment variable if unspecified. Defaults to `diesel.toml` in your project root. See diesel.rs/guides/configuring-diesel-cli for documentation on this file.
//       --locked-schema
//           Require that the schema file is up to date.
//   -h, --help
//           Print help (see more with '--help')
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel migration generate 
// error: the following required arguments were not provided:
//   <MIGRATION_NAME>

// Usage: diesel migration generate <MIGRATION_NAME> [table-name]...

// For more information, try '--help'.
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel migration generate baza
// Creating migrations/2024-04-16-192907_baza/up.sql
// Creating migrations/2024-04-16-192907_baza/down.sql
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel migration list --database-url=database.sql
// Migrations:
//   [ ] 2024-04-16-192907_baza
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel migration list --database-url=database.sqlite
// Migrations:
//   [ ] 2024-04-16-192907_baza
// ┌[sada☮SA]-(~/RUST/rust-crud-api1)-[git://main ✗]-
// └> diesel migration run --database-url=database.sqlite
// Running migration 2024-04-16-192907_baza





curl 127.0.0.1:8000/baza -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -X POST -H 'Content-type: application/json' -d '{"name":"John Doeee", "email":"j_d@example.com"}'

 curl 127.0.0.1:8000/baza -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -H 'Content-type: application/json'
 
[{"created_at":"2024-04-17 09:59:08","email":"j_d@example.com","id":1,"name":"John Doeee"},{"created_at":"2024-04-17 09:59:53","email":"j_d@example.com","id":2,"name":"John Doeee"},{"created_at":"2024-04-17 09:59:55","email":"j_d@example.com","id":3,"name":"John Doeee"},{"created_at":"2024-04-17 10:01:27","email":"j_d@example.com","id":4,"name":"qqqqqJohn Doeee"}]% 