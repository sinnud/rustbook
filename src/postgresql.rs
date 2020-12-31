// PostgreSQL related libraries
// postgresql
use postgres::{Client, NoTls, Row};
use postgres::types::ToSql;
// import data into postgresql database
use std::io::Write;
// URL encoding for connecting to postgresql
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
// walk dir

// in case when your password have symbol out of URL set, like !@ (see below)
#[allow(dead_code)]
pub fn url_encode(ori: &str) -> String {
    /// https://url.spec.whatwg.org/#fragment-percent-encode-set
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
        .add(b'<').add(b'>').add(b'`')
        .add(b'!').add(b'@')
        ;
    let ori = utf8_percent_encode(ori, FRAGMENT).to_string();
    ori
}

pub struct PostgreSQL{
    pub conn: Client,
}
impl Default for PostgreSQL {
    #[allow(dead_code)]
    fn default() -> Self {
        let pw_url=url_encode("Jeffery45!@");
        let constr=format!("postgresql://sinnud:{}@192.168.1.213/dbhuge", pw_url);
        PostgreSQL{
            conn: match Client::connect(&constr, NoTls){
                Ok(pg) => pg,
                Err(err) =>{
                    println!("Error message in default() for PostgreSQL: {:?}", err);
                    println!("Failed to connect database using default() function!");
                    std::process::exit(1);
                }
            },
        }
    }
}
impl PostgreSQL {
    #[allow(dead_code)]
    pub fn new(host: String,
               username: String,
               password: String,
               database: String
    ) -> Result<Self, &'static str> {
        let constr=format!("postgresql://{}:{}@{}/{}", username, url_encode(&password), host, database);
        Ok( PostgreSQL{
            conn: match Client::connect(&constr, NoTls){
                Ok(pg) => pg,
                Err(err) => {
                    println!("Error message in PostgreSQL::new(): {:?}", err);
                    println!("Failed to connect database using new({}:{}@{}/{}) function!",
                        username, &password, host, database
                        );
                    return Err("Failed to connect database using new() function!");
                }
            },
        }
        )
    }
    #[allow(dead_code)]
    pub fn execute(&mut self,
        qry: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<(), &'static str> {
        let row_updated = match self.conn.execute(qry, params){
            Ok(res) => res,
            Err(err) => {
                println!("Error message in PostgreSQL::execute(): {:?}", err);
                return Err("Failed to run execute!");
            }
        };
        info!("number of rows updated: {}", row_updated);
        Ok(())
    }
    #[allow(dead_code)]
    pub fn query(&mut self,
        qry: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<Vec<Row>, &'static str> {
        let vr = match self.conn.query(qry, params){
            Ok(res) => res,
            Err(err) => {
                println!("Error message in PostgreSQL::query(): {:?}", err);
                return Err("Failed to run query!");
            }
        };
        // info!("length of result: {}", vr.len());
        Ok(vr)
    }
    #[allow(dead_code)]
    pub fn table_exist(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<bool, &'static str> {
        let qry="SELECT count(*)::int FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE='BASE TABLE' AND TABLE_SCHEMA=$1 AND TABLE_NAME=$2";
        let row = match self.conn.query_one(qry, &[&skm, &tbl]){
            Ok(res) => res,
            Err(err) => {
                println!("Error message in PostgreSQL::table_exist(): {:?}", err);
                return Err("Failed to run PostgreSQL::table_exist!");
            }
        };
        let res: i32 = row.get(0);
        // info!("{}", res);
        Ok(res>0)
    }
    #[allow(dead_code)]
    pub fn drop_table(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<(), &'static str> {
        let qry=format!("SET SEARCH_PATH='{}'; DROP TABLE {}", skm, tbl);
        info!("in PostgreSQL::drop_table() query is\n{}", qry);
        let qry=&qry[..];
        match self.conn.batch_execute(qry){
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Error message in PostgreSQL::drop_table: {:?}", err);
                return Err("Failed to run PostgreSQL::drop_table!");
            }
        }
    }
    #[allow(dead_code)]
    pub fn truncate_table(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<(), &'static str> {
        let qry=format!("SET SEARCH_PATH='{}'; TRUNCATE TABLE {}", skm, tbl);
        info!("in PostgreSQL::truncate_table() query is\n{}", qry);
        let qry=&qry[..];
        match self.conn.batch_execute(qry){
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Error message in PostgreSQL::truncate_table: {:?}", err);
                return Err("Failed to run PostgreSQL::truncate_table!");
            }
        }
        // let mut transaction = match self.conn.transaction(){
        //     Ok(res) => res,
        //     Err(err) => {
        //         println!("Error message in PostgreSQL::truncate_table.transaction(): {:?}", err);
        //         return Err("Failed to run truncate_table!");
        //     }
        // };
        // match transaction.execute("SET SEARCH_PATH=$1", &[&skm]){
        //     Ok(_) => (),
        //     Err(err) => {
        //         println!("Error message in PostgreSQL::truncate_table.transaction.execute() 1: {:?}", err);
        //         return Err("Failed to run truncate_table!");
        //     }
        // };
        // let row_updated = match transaction.execute("TRUNCATE TABLE $1", &[&tbl]){
        //     Ok(res) => res,
        //     Err(err) => {
        //         println!("Error message in PostgreSQL::truncate_table.transaction.execute() 2: {:?}", err);
        //         return Err("Failed to run execute!");
        //     }
        // };
        // info!("number of rows updated: {}", row_updated);
        // match transaction.commit(){
        //     Ok(_) => (),
        //     Err(err) => {
        //         println!("Error message in PostgreSQL::truncate_table.transaction.commit(): {:?}", err);
        //         return Err("Failed to run truncate_table!");
        //     }
        // };
        // Ok(())
    }
    #[allow(dead_code)]
    pub fn create_table(&mut self,
        skm: &str,
        tbl: &str,
        tbl_str: &str,
    ) -> Result<(), &'static str> {
        let mut qry=format!("SET SEARCH_PATH='{}'; CREATE TABLE {} (", skm, tbl);
        qry.push_str(tbl_str);
        qry.push_str(")");
        info!("in PostgreSQL::create_table() query is\n{}", qry);
        let qry=&qry[..];
        match self.conn.batch_execute(qry){
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Error message in PostgreSQL::create_table: {:?}", err);
                return Err("Failed to run PostgreSQL::create_table!");
            }
        }
    }
    #[allow(dead_code)]
    pub fn create_truncate_table(&mut self,
        skm: &str,
        tbl: &str,
        tbl_str: &str,
    ) -> Result<(), &'static str> {
        if match self.table_exist(skm, tbl){
            Ok(res) => res,
            Err(err) => {
                println!("Error message in PostgreSQL::create_truncate_table: {:?}", err);
                return Err("Failed to run create_truncate_table!");
            }
        }{
            match self.truncate_table(skm, tbl){
                Ok(_) => (),
                Err(err) => {
                    println!("Error message in PostgreSQL::create_truncate_table: {:?}", err);
                    return Err("Failed to run create_truncate_table!");
                }
            };
        } else {
            match self.create_table(skm, tbl, tbl_str){
                Ok(_) => (),
                Err(err) => {
                    println!("Error message in PostgreSQL::create_truncate_table: {:?}", err);
                    return Err("Failed to run PostgreSQL::create_truncate_table!");
                }
            };
        }
        Ok(())
    }
    #[allow(dead_code)]
    pub fn import_data(&mut self,
        qry: &str,
        datastring: String,
    ) -> Result<(), &'static str> {
        let mut writer = match self.conn.copy_in(qry){
            Ok(w) => w,
            Err(err) => {
                println!("Error message in PostgreSQL::import_data().copy_in: {:?}", err);
                return Err("Failed to create writer in import_data() function!");
            }
        };
        match writer.write_all(datastring.as_bytes()){
            Ok(_) => {},
            Err(err) => {
                println!("Error message in PostgreSQL::import_data().write_all: {:?}", err);
                return Err("Failed to write_all in import_data() function!");
            }
        };
        match writer.finish(){
            Ok(_) => {},
            Err(err) => {
                println!("Error message in PostgreSQL::import_data().finish: {:?}", err);
                return Err("Failed to close writer in import_data() function!");
            }
        };
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_default() {
        let mut pg=PostgreSQL::default();
        let qrystring="select 1".to_string();
        let qry=&qrystring[0..];
        let rst = pg.conn.query(qry, &[]).unwrap();
        println!("{:#?}", rst);
    }
    #[test]
    fn connect_to_dbhuge() {
        let mut pg=PostgreSQL::new(
                "192.168.1.213".to_string(), // host
                "sinnud".to_string(),        // username
                "Jeffery45!@".to_string(),   // password
                "dbhuge".to_string(),        // database
            ).unwrap_or_else(|err| {
                eprintln!("(E)Error: {}", err);
                std::process::exit(1);
            });
        let rst = pg.conn.query("select 1", &[]).unwrap();
        println!("{:#?}", rst);
    }
}