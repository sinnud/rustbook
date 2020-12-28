
use postgresql::{PostgreSQL, FileStatus};
fn main()-> Result<(), &'static str>{
    // let mut wd = PostgreSQL::default();
    let mut wd = match PostgreSQL::new("192.168.1.213".to_owned(),
        "sinnud".to_owned(),
        "Jeffery45!@".to_owned(),
        "dbhuge".to_owned(),
    ){
        Ok(pg) => pg,
        Err(err) => {
            println!("Error message in main(): {:?}", err);
            return Err("Failed to connect postgresql database!");
        }
    };
    
    let mut qry: String="select table_name from information_schema.tables ".to_owned();
    let qry2="where table_catalog='dbhuge' ";
    qry.push_str(qry2);
    let qry2="and table_schema='wdinfo'";
    qry.push_str(qry2);
    println!("Query is \n{}", &qry);
    let qry=&qry[0..];
    for row in match &wd.conn.query(qry, &[]){
        Ok(res) => res,
        Err(err) => {
            println!("Error message in main(): {:?}", err);
            return Err("Failed to run above query!");
        }
    } {
        let tblname: String=row.get(0);
        println!("Found table {:?}", tblname);
    }
    
    let qry="truncate table wdinfo._file_st";
    println!("Query is \n{}", qry);
    match &wd.conn.execute(qry, &[]){
        Ok(_) => {}
        Err(err) => {
            println!("Error message in main(): {:?}", err);
            return Err("Failed to execute above query!");
        }
    };

    let fs=FileStatus::get_file_status_under_folder("src","|").unwrap();
    println!("Result:\n{}", fs);
    let qry="COPY wdinfo._file_st FROM STDIN DELIMITER '|'";
    match wd.import_data(qry, fs){
        Ok(_) => {},
        Err(err) => {
            println!("Error message in main(): {:?}", err);
            println!("Query: {}", qry);
            return Err("Failed to execute the above query with import_data()!");
        }
    };
    Ok(())
}
