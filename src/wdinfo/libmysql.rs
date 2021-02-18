/*! MySQL related functions and metods */
// query related
use mysql::prelude::Queryable;
// time related for import_data
#[allow(unused_imports)]
use chrono::offset::{Utc, Local};
use chrono::DateTime;
use std::time::SystemTime;
// file related for import_data
use std::io::Write;
// use url_encode
use crate::sqltrait::url_encode;
// use trait
use crate::sqltrait::SQL;

use crate::pem;
/** # LibMySQL
 * Connect to MySQL
 * Execute queries to MySQL
 * import data into MySQL
 */
pub struct LibMySQL{
    pub conn: mysql::PooledConn,
}
/** # Default initialization of LibMySQL
 * use as defalt
 */
impl Default for LibMySQL {
    /** # default method
     * Use mysql::Pool method
     * MySQL database installed in 192.168.1.213
     */
    #[allow(dead_code)]
    fn default() -> Self {
        let filename="/mnt/public/data/other/pem/config_ms_sinnud";
        let pem = pem::db_pem(filename).unwrap();
        let ip = &pem[0];
        // let port = &pem[1];
        let database = &pem[2];
        let username = &pem[3];
        let password = &pem[4];
        let pw_url=url_encode(password);
        let constr=format!("mysql://{}:{}@{}/{}", username, pw_url, ip, database);
        let pool = match mysql::Pool::new(&constr){
            Ok(res) => res,
            Err(err) => {
                error!("in default() for LibMySQL: {:?}", err);
                println!("Pool::new failed: {:?}", err);
                println!("Failed to connect database using default() function!");
                std::process::exit(1);
            }
        };
        LibMySQL{
            conn: match pool.get_conn(){
                Ok(res) => res,
                Err(err) =>{
                    error!("in default() for LibMyQL: {:?}", err);
                    println!("get_conn failed: {:?}", err);
                    println!("Failed to connect database using default() function!");
                    std::process::exit(1);
                }
            },
        }
    }
}
impl LibMySQL{
    /** # customer connection
     * You can provide:
       * host as string
       * username as string
       * password as string
       * database as string
     */
    #[allow(dead_code)]
    pub fn new(host: String, username: String, password: String, database: String) -> Result<Self, &'static str> {
        let constr=format!("mysql://{}:{}@{}/{}", username, url_encode(&password), host, database);
        let pool = match mysql::Pool::new(&constr){
            Ok(res) => res,
            Err(err) => {
                error!("in new() for LibMySQL: {:?}", err);
                return Err("Failed to connect database using new() function!");
            }
        };
        Ok( LibMySQL{
            conn: match pool.get_conn(){
                Ok(res) => res,
                Err(err) => {
                    error!("in LibMySQL::new(): {:?}", err);
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
impl SQL for LibMySQL {
    // type Output = LibMySQL;
    /** # submit query without return values
     */
    fn execute_queries_no_return(&mut self, qry: &str) -> Result<(), &'static str>{
        match self.conn.query_drop(qry){
            Ok(res) => res,
            Err(err) => {
                error!("in LibMySQL::execute(): {:?}", err);
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
        let qry=format!("SELECT cast(count(*) as nchar) as cnt FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA='{}' AND TABLE_NAME='{}'",
                skm, tbl);
        // info!("DEBUG: query: {}", qry);
        let rows = match self.execute_query_with_return(&qry){
            Ok(res) => res,
            Err(err) => {
                error!("in LibMySQL::table_exist(): {:?}", err);
                return Err("Failed to run LibMySQL::table_exist!");
            }
        };
        // info!("DEBUG: {:?}", rows);
        for row in &rows {
            // info!("DEBUG: {} row='{}' with length {}", tbl, row, row.len());
            if row.len() == 0 {return Ok(false)}
            let i: i64 = row.split("\t").nth(0).unwrap().parse::<i64>().unwrap();
            if i>0 {return Ok(true)}
        }
        Ok(false)
    }
    /** # drop table
     * need schema and table name as argument
     */
    fn drop_table(&mut self, skm: &str, tbl: &str) -> Result<(), &'static str>{
        let qry=format!("DROP TABLE {}.{}", skm, tbl);
        info!("in LibMySQL::drop_table() query is\n{}", qry);
        match self.execute_queries_no_return(&qry){
            Ok(_) => Ok(()),
            Err(err) => {
                error!("in LibMySQL::drop_table: {:?}", err);
                return Err("Failed to run LibMySQL::drop_table!");
            }
        }
    }
    /** # truncate table
     * need schema and table name as argument
     */
    fn truncate_table(&mut self, skm: &str, tbl: &str) -> Result<(), &'static str>{
        let qry=format!("TRUNCATE TABLE {}.{}", skm, tbl);
        debug!("in LibMySQL::truncate_table() query is\n{}", qry);
        match self.execute_queries_no_return(&qry){
            Ok(_) => Ok(()),
            Err(err) => {
                error!("in LibMySQL::truncate_table: {:?}", err);
                return Err("Failed to run LibMySQL::truncate_table!");
            }
        }
    }
    /** # create table
     * need schema, table name, and table structure (String) as argument
     */
    fn create_table(&mut self, skm: &str, tbl: &str, tbl_str: &str) -> Result<(), &'static str>{
        let mut qry=format!("CREATE TABLE {}.{} (", skm, tbl);
        qry.push_str(tbl_str);
        qry.push_str(")");
        // info!("in LibMySQL::create_table() query is\n{}", qry);
        match self.execute_queries_no_return(&qry){
            Ok(_) => Ok(()),
            Err(err) => {
                error!("in LibMySQL::create_table: {:?}", err);
                return Err("Failed to run LibMySQL::create_table!");
            }
        }
    }
    /** # import data into LibMySQL table
     * need query and data string as argument
     */
    fn import_data(&mut self, skm: &str, tbl: &str, datastring: String) -> Result<(), &'static str>{
        info!("Store datastring to /tmp/random_file...");
        let x = md5::compute(&datastring);
        let s = format!("{:x}", x);
        let s8 = &s[0..8];
        let system_time = SystemTime::now();
        let datetime: DateTime<Local> = system_time.into();
        let post_ts=datetime.format("%Y%m%d_%H%M%S");
        let tmp_file=format!("/tmp/{}_{}", s8, post_ts);
        let mut ofile = std::fs::File::create(&tmp_file)
                       .expect("unable to create file");
        ofile.write_all(datastring.as_bytes()).expect("unable to write");
        info!("ftp to mysql workstation...");
        //scp -i .ssh/ubuntu_user.pem dbeaver.desktop user@192.168.1.213:/tmp
        let output = std::process::Command::new("scp")
                         .arg("-i")
                         .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                         .arg(&tmp_file)
                         .arg("sinnud@192.168.1.213:/tmp")
                         .output()
                         .expect("failed to execute process");
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed scp: {}", String::from_utf8_lossy(&output.stderr));
            return Err("scp failed.");
        }
        info!("change owner to mysql.mysql...");
        let output = std::process::Command::new("ssh")
                         .arg("-i")
                         .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                         .arg("sinnud@192.168.1.213")
                         .arg("sudo")
                         .arg("chown")
                         .arg("mysql.mysql")
                         .arg(&tmp_file)
                         .output()
                         .expect("failed to execute process");
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed ssh: {}", String::from_utf8_lossy(&output.stderr));
            return Err("ssh failed.");
        }
        info!("load into MySQL table...");
        let qry=format!("load data infile '{}' into table {}.{}", &tmp_file, skm, tbl);
        match self.execute_queries_no_return(&qry){
            Ok(_) => (),
            Err(err) => {
                error!("Failed to load data: {}", err);
                return Err("load data infile failed");
            }
        };
        info!("Delete local data file...");
        match std::fs::remove_file(&tmp_file){
            Ok(_) => (),
            Err(err) => {
                error!("Failed to delete local data file {}: {}", tmp_file, err);
                return Err("Delete local data file failed!");
            }
        }
        info!("Delete mysql data file...");
        let output = std::process::Command::new("ssh")
                         .arg("-i")
                         .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                         .arg("sinnud@192.168.1.213")
                         .arg("sudo")
                         .arg("rm")
                         .arg(&tmp_file)
                         .output()
                         .expect("failed to execute process");
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed ssh: {}", String::from_utf8_lossy(&output.stderr));
            return Err("ssh failed.");
        }
        Ok(())
    }
    /** # submit query and catch the output
     * output is vector of String with [tab] as delimiter
     * Leave it to be handled by the following code
     */
    fn execute_query_with_return(&mut self, qry: &str) -> Result<Vec<String>, &'static str>{
        let vr: Vec<mysql::Row> = match self.conn.query(qry){
            Ok(res) => res,
            Err(err) => {
                error!("in MySQL::query(): {:?}", err);
                return Err("Failed to run query!");
            }
        };
        let mut vs: Vec<String> = Vec::with_capacity(vr.len());
        for row in &vr {
            let mut line = String::new();
            // println!("DEBUG Length of row: {}", row.len());
            let clms=row.len();
            for i in 0..clms {
                let cstr: String = row.get(i).unwrap();
                line.push_str(&cstr);
                if i+1 < clms {
                    line.push_str("\t");
                }
            }
            vs.push(line);
        }
        // println!("DEBUG: {:?}", vs);
        Ok(vs)
    }
}