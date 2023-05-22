mod dbutils;

//use std::env;
use dbutils::DBSetup;



fn main() {
    println!("[*] Starting...");


    let dbpath : &str = "C:\\progs\\dev\\rust_sqlitetest\\rsqlite\\app.db";
    let tabledefpath : &str = "c:\\progs\\dev\\rust_sqlitetest\\rsqlite\\configs\\schema_defs.json";

    dbutils::test1();
    DBSetup::create_tables(&dbpath, &tabledefpath);
    println!("[*] main: done.");
}
