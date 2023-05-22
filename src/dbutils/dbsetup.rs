use sqlite;
use sqlite::State;


#[path = "tabledef_struct.rs"]
mod tabledef_struct;

pub struct DBSetup;

impl DBSetup {

    pub fn create_tables(filepath: &str, tabledefpath: &str){
        println!("[!] create_table setup: Not implemented");
        println!(" [i] DB path: {}", filepath);
        println!(" [i] config path: {}", tabledefpath);

        let conn = sqlite::open(filepath).unwrap();
        let check_query = "SELECT sum(1) as rowcount FROM sqlite_master WHERE type = 'table' AND name = ?";

        let tabledefs_str = match std::fs::read_to_string(tabledefpath) { 
            Ok(x) => x.to_string(),
            Err(e) => panic!("[!] cannot read config file: {}",e)
        };

        let tabledef : tabledef_struct::Root = serde_json::from_str(&tabledefs_str).expect("    [!] error loading config data");
    
        for table in &tabledef.tables {
            // TODO: decide what to do with existing table - maybe just drop them? 
            let mut statement = conn.prepare(check_query).unwrap();

            let tn = table.name.as_str();
            println!("Table name: {}", tn);
            statement.bind((1, tn)).unwrap();
            while let Ok(State::Row) = statement.next() {
                let exists = statement.read::<i64, _>("rowcount").unwrap();
                println!("{}", exists);
            }
        }

    }
}
