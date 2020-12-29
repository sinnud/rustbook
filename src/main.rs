mod lib_pg;
use crate::lib_pg::PostgreSQL;
mod lib_fs;
#[allow(unused_imports)]
use crate::lib_fs::FileStatus;
fn main()-> Result<(), &'static str>{
    let mut wd = PostgreSQL::default();
    // let mut wd = PostgreSQL::new("192.168.1.213".to_owned(),
    // "sinnud".to_owned(),
    // "Jeffery45!@".to_owned(),
    // "dbhuge".to_owned(),
    // )?;
    
    // let qry="select table_name from information_schema.tables where table_catalog='dbhuge' and table_schema='wdinfo'";
    // let dbname="dbhuge";
    // let skmname="wdinfo";
    // let qry="select table_name from information_schema.tables where table_catalog=$1 and table_schema=$2";
    // println!("Query is: {}", qry);
    // for row in wd.query(qry, &[&dbname, &skmname])? {
    //     let tblname: String=row.get(0);
    //     println!("Found table {:?}", tblname);
    // }
    
    let skmname="wdinfo";
    let tblname="_file_st";
    let tblstr="filename texterr, folder text, type text, fullpath text, filesize bigint, filecreate_dt timestamp";
    wd.create_truncate_table(skmname, tblname, tblstr)?;

    let fs=FileStatus::get_file_status_under_folder("src","|").unwrap();
    println!("Result:\n{}", fs);
    let qry="COPY wdinfo._file_st FROM STDIN DELIMITER '|'";
    wd.import_data(qry, fs)?;
    Ok(())
}
