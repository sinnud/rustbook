// postgresql
use postgres::{Client, NoTls};
// import data into postgresql database
use std::io::Write;
// URL encoding for connecting to postgresql
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
// walk dir
extern crate walkdir;
use walkdir::WalkDir;
use std::path::PathBuf;

// in case when your password have symbol out of URL set, like !@ (see below)
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
    pub fn import_data(mut self: PostgreSQL,
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

#[derive(Debug)]
pub struct FileStatus {
    path: String,
    name: String,
    isdir: bool,
    filetype: String,
    size: u64,
    dt_modified: String,
    dt_accessed: String,
    dt_created: String,
}
impl FileStatus {
    pub fn get_status(path: &PathBuf) -> Result<FileStatus, &'static str> {
        // println!("DEBUG in get_status: extension() is {:?}", path.extension());
        let ext=if path.extension().is_none(){""}else{path.extension().unwrap().to_str().unwrap()};
        // println!("DEBUG in get_status: ext is '{}' with length {}", ext, ext.len());
        let base=path.file_stem().unwrap().to_str().unwrap();
        let dir=path.parent().unwrap().to_str().unwrap();
        let md = std::fs::metadata(path).unwrap();
        let res=FileStatus {
            path: dir.to_string()
            , name: if ext.len()==0 {base.to_string()} else {format!("{}.{}",base,ext)}
            , isdir: md.is_dir()
            , filetype: ext.to_string()
            , size: md.len()
            ,dt_modified: if let Ok(time) = md.modified() {
                let datetime: chrono::DateTime<chrono::offset::Local> = time.into();
                // datetime.format("%Y%m%d_%H%M%S").to_string()
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }else{return Err("failed to get modified time!");}
            ,dt_accessed: if let Ok(time) = md.accessed() {
                let datetime: chrono::DateTime<chrono::offset::Local> = time.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }else{return Err("failed to get accessed time!");}
            ,dt_created: if let Ok(time) = md.created() {
                let datetime: chrono::DateTime<chrono::offset::Local> = time.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }else{return Err("failed to get created time!");}
        };
        Ok(res)
    }
    pub fn get_file_status_under_folder(path: &str, 
        delimiter: &str     // for data base import
    ) -> Result<String, &'static str> {
        let pathori=PathBuf::from(path);
        let pathcrt=if pathori.is_relative(){
            std::env::current_dir().unwrap().join(path)
        } else {pathori};
        // println!("DEBUG path is: {}", pathcrt.to_str().unwrap());
        let dir=WalkDir::new(pathcrt.to_str().unwrap());
        let mut res="".to_string();
        for e in dir.into_iter().filter_map(|e| e.ok()){
            if e.metadata().unwrap().is_file(){
                // println!("DEBUG e is: {:?}", e.path());
                let buf = PathBuf::from(e.path());
                let fs=FileStatus::get_status(&buf).unwrap();
                // println!("DEBUG fs is: {:?}", fs);
                res.push_str(&fs.name);res.push_str(delimiter);
                res.push_str(&fs.path);res.push_str(delimiter);
                res.push_str(&fs.filetype);res.push_str(delimiter);
                res.push_str(&fs.path);res.push_str("/");
                res.push_str(&fs.name);res.push_str(delimiter);
                res.push_str(&fs.size.to_string());res.push_str(delimiter);
                res.push_str(&fs.dt_created);res.push_str("\n");
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_default() {
        let mut pg=PostgreSQL::conn_default().unwrap_or_else(|err| {
                eprintln!("(E)Error: {}", err);
                std::process::exit(1);
            });
        let qrystring="select 1".to_string();
        let qry=&qrystring[0..];
        let rst = pg.query(qry, &[]).unwrap();
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
        let rst = pg.query("select 1", &[]).unwrap();
        println!("{:#?}", rst);
    }
    #[test]
    fn check_file_status() {
        let filename="README.md";
        let pathori=PathBuf::from(filename);
        let path=if pathori.is_relative(){
            std::env::current_dir().unwrap().join(filename)
            } else {pathori};
        let fs=FileStatus::get_status(&path).unwrap();
        assert_eq!(fs.name,"README.md".to_string());
    }
}

