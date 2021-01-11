/** # main function executable program start from.
 
- Use return Result<(), &'static str> such that ? operator can be used if error is &str.
- Assume log4rs.yaml file is stored under folder config
under the same folder the executable program located `config/log4rs.yaml`.
- Assume log folder is located the same folder the executable program located `log/wdinfo.log`.
  - When the production (released) program is executed from other folder, log folder 
    defined in log4rs.yaml need to manually created with full path
  - log file will be archived using timestamp at the beginning of run.
- Support command line arguments. See [envargs](../lib_wd/envargs/index.html).
 */
#[macro_use]
extern crate log;
extern crate log4rs;
use wdinfo::sqltrait::{SQL, SQLret};
/*
fn main()-> Result<(), &'static str>{
    let root=wdinfo::file_status::log_config_path()?;
    
    // log file is log/wdinfo.log. see config/log4ts.yaml
    let logfile=format!("{}/log/wdinfo.log", root);
    let log4rs_yaml=format!("{}/config/log4rs.yaml", root);
    
    wdinfo::file_status::rename_log_with_timestamp(&logfile)?;
    log4rs::init_file(&log4rs_yaml, Default::default()).unwrap();

    let opt=envargs()?;
    println!("Work on {}...", opt);
    
    let mut wd=wdinfo::wdinfo::WDInfo::default();

    if opt.contains("music"){
        wd.wdrefresh("/mnt/music", "wdinfo", "music243")?;
        wd.wdrefresh("/mnt/public/music", "wdinfo", "music241")?;
        wd.wdsync("wdinfo", "music243", "music241")?;
    }

    if opt.contains("data"){
        wd.wdrefresh("/mnt/data", "wdinfo", "data243")?;
        wd.wdrefresh("/mnt/public/data", "wdinfo", "data241")?;
        wd.wdsync("wdinfo", "data243", "data241")?;
    }

    if opt.contains("photos"){
        wd.wdrefresh("/mnt/photos", "wdinfo", "photos243")?;
        wd.wdrefresh("/mnt/public/photos", "wdinfo", "photos241")?;
        wd.wdsync("wdinfo", "photos243", "photos241")?;
    }

    if opt.contains("movie"){
        wd.wdrefresh("/mnt/movie", "wdinfo", "movie243")?;

        wd.wdrefresh("/mnt/public/newmovies", "wdinfo", "movie241")?;
    }
    
    Ok(())
}
/** # check command line argument
 - no argument is default, do all works: data, music, photos, movie
 - can be arbitrary argument(s) from the above four
 - If duplicated, choose one
 - if other than above four, error out
 - return clean string
 */
pub fn envargs() ->Result<String, &'static str> {
    let mut args: Vec<String> = std::env::args().collect();
    let default_opt="data music photos movie".to_owned();
    if args.len() == 1{
        return Ok(default_opt)
    }
    let mut res="".to_owned();
    args.drain(0..1); // remove first one: execute program name
    for e in args{
        let good = e.to_lowercase();
        if default_opt.contains(&good){
            if res.len()==0{res.push_str(&good);}
            else if ! res.contains(&good){
                res.push_str(" ");
                res.push_str(&good);
            }
        }
    }
    Ok(res)
}
*/
fn main()-> Result<(), &'static str>{
    let root=wdinfo::file_status::log_config_path()?;
    
    // log file is log/wdinfo.log. see config/log4ts.yaml
    let logfile=format!("{}/log/wdinfo.log", root);
    let log4rs_yaml=format!("{}/config/log4rs.yaml", root);
    
    wdinfo::file_status::rename_log_with_timestamp(&logfile)?;
    log4rs::init_file(&log4rs_yaml, Default::default()).unwrap();
    
    // info!("Connect to MySql using default...");
    // let mut ms=wdinfo::libmysql::LibMySQL::default();
    info!("Connect to MySql using new method...");
    let mut ms=wdinfo::libmysql::LibMySQL::new("192.168.1.213".to_owned(),
        "sinnud".to_owned(),
        "Jeffery45!@".to_owned(),
        "wdinfo".to_owned()
    )?;
    info!("Submit one query without return...");
    ms.execute_queries_no_return("drop table if exists wdinfo.sinnud; create table wdinfo.sinnud (like wdinfo.data)")?;
    info!("Submit one query without return...");
    let rows: Vec<mysql::Row> = ms.execute_query_with_return("select * from wdinfo.sinnud")?;
    info!("Return {} rows.", rows.len());
    info!("check table exists...");
    if ms.check_table_exists("wdinfo", "sinnud")?{
        info!("Table exists!");
    } else {
        info!("Table DOES NOT exist!");
    }
    info!("create_truncate_table...");
    ms.create_truncate_table("wdinfo", "_file_st", "filename text, folder text, type text, fullpath text, filesize bigint, filecreate_dt timestamp")?;
    
    info!("Connect to PostgreSql using default...");
    let mut pg=wdinfo::postgresql::PostgreSQL::default();
    info!("Submit one query without return...");
    pg.execute_queries_no_return("drop table if exists wdinfo.sinnud; create table wdinfo.sinnud (like wdinfo.data243)")?;
    info!("Submit one query without return...");
    let rows: Vec<postgres::Row> = pg.execute_query_with_return("select * from wdinfo.sinnud")?;
    info!("Return {} rows.", rows.len());
    info!("check table exists...");
    if pg.check_table_exists("wdinfo", "sinnud")?{
        info!("Table exists!");
    } else {
        info!("Table DOES NOT exist!");
    }
    info!("create_truncate_table...");
    pg.create_truncate_table("wdinfo", "_file_st", "filename text, folder text, type text, fullpath text, filesize bigint, filecreate_dt timestamp")?;
    
    // let mut file = match std::fs::File::open("/home/user/bulb"){
    //     Ok(res) => res,
    //     Err(err) => {
    //         error!("Error open file: {}", err);
    //         return Err("Failed open file!");
    //     }
    // };
    // let mut contents = String::new();
    // use std::io::prelude::*;
    // match file.read_to_string(&mut contents){
    //     Ok(_) => (),
    //     Err(err) => {
    //         error!("Error read file: {}", err);
    //         return Err("Failed read file!");
    //     }
    // };
    // ms.import_data("wdinfo", "sinnud", contents)?;
    
    /*
    info!("clean server file...");
    let output = std::process::Command::new("ssh")
                     .arg("-i")
                     .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                     .arg("sinnud@192.168.1.213")
                     .arg("sudo")
                     .arg("rm")
                     .arg("-f")
                     .arg("/tmp/bulb")
                     .output()
                     .expect("failed to execute process");
    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    info!("export data to server file...");
    ms.execute("select * from movie limit 10 into outfile '/tmp/bulb'")?;
    info!("load data from server file...");
    ms.execute("load data infile '/tmp/bulb' into table sinnud")?;
    
    info!("Submit one query with return...");
    let rows = ms.query("select * from movie limit 2")?;
    // info!("result length: {}", rows.len());
    // let mut v: Vec<String> = Vec::with_capacity(rows.len());
    // for row in rows{
    //     let elm: String=row.get(0);
    //     v.push(elm);
    // }
    // debug!("result value:\n{}", v.join("\n"));
    info!("Result: {:?}", rows);
    info!("Check table exists...");
    if ms.table_exist("wdinfo", "data")? {
        info!("Table wdinfo.data exists.");
    }
    info!("drop table...");
    ms.drop_table("wdinfo", "sinnud")? ;
    info!("truncate table: create...");
    // ms.execute("drop table if exists sinnud; create table sinnud (like data)")?;
    let tblstr="`index` bigint, filename text, folder text, file_type text, fullpath text, filesize bigint, createtime datetime";
    ms.create_table("wdinfo", "sinnud", tblstr)? ;
    info!("truncate table: insert...");
    ms.execute("insert into sinnud select * from data limit 10")?;
    info!("truncate table...");
    ms.truncate_table("wdinfo", "sinnud")? ;
    info!("create_truncate_table...");
    ms.create_truncate_table("wdinfo", "sinnud", tblstr)? ;
    
    let x = md5::compute("qwertyuiopasdfghjklzxcvbnm");
    let s = format!("{:x}", x);
    let s8 = &s[0..8];
    info!("MD5 of qwertyuiopasdfghjklzxcvbnm is {} and short {}", s, s8);
    info!("Finished");
    */
    Ok(())
}