// log to file
#[macro_use]
extern crate log;
extern crate log4rs;

#[allow(unused_imports)]
use crate::postgresql::PostgreSQL;
#[allow(unused_imports)]
use crate::file_status::FileStatus;
#[allow(unused_imports)]
use crate::wdinfo::WDInfo;

mod postgresql;
mod file_status;
mod wdinfo;

#[allow(unused_imports)]
use crate::file_status::rename_log_with_timestamp;


fn main()-> Result<(), &'static str>{
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // see config/log4ts.yaml
    rename_log_with_timestamp("log/requests.log")?;
    
    // let mut wd = PostgreSQL::default();
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
/*    
    let skmname="wdinfo";
    let tblname="_file_st";
    let tblstr="filename text, folder text, type text, fullpath text, filesize bigint, filecreate_dt timestamp";
    wd.create_truncate_table(skmname, tblname, tblstr)?;

    let fs=FileStatus::get_file_status_under_folder("src","|").unwrap();
    println!("Result:\n{}", fs);
    let qry="COPY wdinfo._file_st FROM STDIN DELIMITER '|'";
    wd.import_data(qry, fs)?;
*/
    let mut wd=WDInfo::default();
    // let thisstr=wd.last_insert_dt("wdinfo", "movie241")?;
    // info!("Result is {}", thisstr);

    info!("Start collecting file information from WD net drive, importing to postgresql temporary table...");
    wd.fs_import_pg("/mnt/public/newmovies", "//192.168.1.241/")?;
    info!("Finished fs_import_pg.");

    info!("Start refresh table in postgresql...");
    wd.wdinfo_refresh("wdinfo", "movie241")?;
    info!("Finished refresh table.");
    Ok(())
}
