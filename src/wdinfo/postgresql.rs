/*! PostgreSQL related functions and metods */
// postgresql
// use postgres::{Client, NoTls};
// use postgres::types::ToSql;
// import data into postgresql database
#[allow(unused_imports)]
use std::io::Write;
// use trait
use crate::sqltrait::SQL;

/** # PostgreSQL
 * Connect to PostgreSQL
 * Execute queries to PostgreSQL
 * import data into PostgreSQL
 */
pub struct PostgreSQL{
    pub conn: postgres::Client,
}
/** # Default initialization of PostgreSQL
 * use as defalt
 */
impl Default for PostgreSQL {
    /** # default method
     * Use postgres::Client::connect method
     * PostgreSQL database installed in 192.168.1.213
     */
    #[allow(dead_code)]
    fn default() -> Self {
        let pw_url=crate::sqltrait::url_encode("Jeffery45!@");
        let constr=format!("postgresql://sinnud:{}@192.168.1.213/dbhuge", pw_url);
        PostgreSQL{
            conn: match postgres::Client::connect(&constr, postgres::NoTls){
                Ok(pg) => pg,
                Err(err) =>{
                    error!("in default() for PostgreSQL: {:?}", err);
                    println!("Failed to connect database using default() function!");
                    std::process::exit(1);
                }
            },
        }
    }
}
impl PostgreSQL {
    /** # customer connection
     * You can provide:
       * host as string
       * username as string
       * password as string
       * database as string
     */
    #[allow(dead_code)]
    pub fn new(host: String, username: String, password: String, database: String) -> Result<Self, &'static str> {
        let constr=format!("postgresql://{}:{}@{}/{}", username, crate::sqltrait::url_encode(&password), host, database);
        Ok( PostgreSQL{
            conn: match postgres::Client::connect(&constr, postgres::NoTls){
                Ok(pg) => pg,
                Err(err) => {
                    error!("in PostgreSQL::new(): {:?}", err);
                    info!("Failed to connect database using new({}:{}@{}/{}) function!",
                        username, &password, host, database
                        );
                    return Err("Failed to connect database using new() function!");
                }
            },
        }
        )
    }
}
impl SQL for PostgreSQL {
    /** # submit query without return values
     */
    fn execute_queries_no_return(&mut self, qry: &str) -> Result<(), &'static str>{
        match self.conn.batch_execute(qry){
            Ok(res) => res,
            Err(err) => {
                error!("in PostgreSQL::batch_execute(): {:?}", err);
                return Err("Failed to run execute_query_no_return!");
            }
        };
        Ok(())
    }
    /** # check if table exist
     * Return bool
     * need schema and table name as argument
     */
    fn check_table_exists(&mut self, skm: &str, tbl: &str) -> Result<bool, &'static str>{
        let qry=format!("SELECT count(*)::text FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE='BASE TABLE' AND TABLE_SCHEMA='{}' AND TABLE_NAME='{}'",
            skm, tbl,
        );
        let rows = match self.execute_query_with_return(&qry){
            Ok(res) => res,
            Err(err) => {
                error!("in PostgreSQL::check_table_exist(): {:?}", err);
                return Err("Failed to run PostgreSQL::check_table_exist!");
            }
        };
        for row in &rows {
            // println!("DEBUG: {}", row);
            let res: i32 = match row.split("\t").nth(0).unwrap().parse::<i32>(){
                Ok(res) => res,
                Err(err) => {
                    error!("in PostgreSQL::check_table_exist(): {:?}", err);
                    return Err("Failed to run PostgreSQL::check_table_exist!");
                }
            };
            return Ok(res>0)
        }
        Ok(false)
    }
    /** # drop table
     * need schema and table name as argument
     */
    fn drop_table(&mut self, skm: &str, tbl: &str) -> Result<(), &'static str>{
        let qry=format!("SET SEARCH_PATH='{}'; DROP TABLE {}", skm, tbl);
        debug!("in PostgreSQL::drop_table() query is\n{}", qry);
        match self.execute_queries_no_return(&qry){
            Ok(_) => Ok(()),
            Err(err) => {
                error!("in PostgreSQL::drop_table: {:?}", err);
                return Err("Failed to run PostgreSQL::drop_table!");
            }
        }
    }
    /** # truncate table
     * need schema and table name as argument
     */
    fn truncate_table(&mut self, skm: &str, tbl: &str) -> Result<(), &'static str>{
        let qry=format!("SET SEARCH_PATH='{}'; TRUNCATE TABLE {}", skm, tbl);
        debug!("in PostgreSQL::truncate_table() query is\n{}", qry);
        match self.execute_queries_no_return(&qry){
            Ok(_) => Ok(()),
            Err(err) => {
                error!("in PostgreSQL::truncate_table: {:?}", err);
                return Err("Failed to run PostgreSQL::truncate_table!");
            }
        }
    }
    /** # create table
     * need schema, table name, and table structure (String) as argument
     */
    fn create_table(&mut self, skm: &str, tbl: &str, tbl_str: &str) -> Result<(), &'static str>{
        let mut qry=format!("SET SEARCH_PATH='{}'; CREATE TABLE {} (", skm, tbl);
        qry.push_str(tbl_str);
        qry.push_str(")");
        info!("in PostgreSQL::create_table() query is\n{}", qry);
        match self.execute_queries_no_return(&qry){
            Ok(_) => Ok(()),
            Err(err) => {
                error!("in PostgreSQL::create_table: {:?}", err);
                return Err("Failed to run PostgreSQL::create_table!");
            }
        }
    }
    /** # import data into PostgreSQL table
     * need query and data string as argument
     */
    fn import_data(&mut self, skm: &str, tbl: &str, datastring: String) -> Result<(), &'static str>{
        // let qry=format!("COPY {}.{} FROM STDIN DELIMITER '|'", skm, tbl);
        let qry=format!("COPY {}.{} FROM STDIN", skm, tbl);
        let qry=&qry[..];
        let mut writer = match self.conn.copy_in(qry){
            Ok(w) => w,
            Err(err) => {
                error!("in PostgreSQL::import_data().copy_in: {:?}", err);
                return Err("Failed to create writer in import_data() function!");
            }
        };
        match writer.write_all(datastring.as_bytes()){
            Ok(_) => {},
            Err(err) => {
                error!("in PostgreSQL::import_data().write_all: {:?}", err);
                return Err("Failed to write_all in import_data() function!");
            }
        };
        match writer.finish(){
            Ok(_) => {},
            Err(err) => {
                error!("in PostgreSQL::import_data().finish: {:?}", err);
                std::fs::write("_debug.txt", datastring).expect("Unable to write file");
                return Err("Failed to close writer in import_data() function!");
            }
        };
        Ok(())
    }
    /** # submit query and catch the output
     * output is vector of String with [tab] as delimiter
     * Leave it to be handled by the following code
     */
    fn execute_query_with_return(&mut self, qry: &str) -> Result<Vec<String>, &'static str>{
        let vr = match self.conn.query(qry, &[]){
            Ok(res) => res,
            Err(err) => {
                error!("in PostgreSQL::query(): {:?}", err);
                return Err("Failed to run query!");
            }
        };
        let mut vs: Vec<String> = Vec::with_capacity(vr.len());
        for row in &vr {
            let mut line = String::new();
            let clms=row.len();
            for i in 0..clms {
                // let cstr: String = row.get(i).parse::<&str>().unwrap();
                let cstr: String = row.get(i);
                line.push_str(&cstr);
                if i+1 < clms {
                    line.push_str("\t");
                }
            }
            vs.push(line);
        }
        Ok(vs)
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