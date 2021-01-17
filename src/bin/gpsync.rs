// #[macro_use]
extern crate log;
extern crate log4rs;
#[allow(unused_imports)]
use wdinfo::sqltrait::SQL;
/** # main function executable program start from.
 
- Use return Result<(), &'static str> such that ? operator can be used if error is &str.
- Assume log4rs.yaml file is stored under folder config
under the same folder the executable program located `config/log4rs.yaml`.
- Assume log folder is located the same folder the executable program located `log/wdinfo.log`.
  - When the production (released) program is executed from other folder, log folder 
    defined in log4rs.yaml need to manually created with full path
  - log file will be archived using timestamp at the beginning of run.
- Support command line arguments. See [envargs](./fn.envargs.html).
 */
fn main()-> Result<(), &'static str>{
    let root=wdinfo::file_status::log_config_path()?;
    
    // log file is log/wdinfo.log. see config/log4ts.yaml
    let logfile=format!("{}/log/wdinfo.log", root);
    let log4rs_yaml=format!("{}/config/log4rs.yaml", root);
    
    wdinfo::file_status::rename_log_with_timestamp(&logfile)?;
    log4rs::init_file(&log4rs_yaml, Default::default()).unwrap();

    let opt="data music photos movie".to_owned();
    let vec: Vec<&str>=opt.split(" ").collect();
    let mut tbllist=Vec::new();
    for f in &vec {
        tbllist.push(format!("{}241", f));
        tbllist.push(format!("{}243", f));
    }
    
    let mut iepg = wdinfo::ubuntu::DBinfo::init(
        "192.168.1.213".to_owned(),
        "5432".to_owned(), // PostgreSQL
        "dbhuge".to_owned(),  // PostgreSQL
        "sinnud".to_owned(),
        "Jeffery45!@".to_owned(), // PostgreSQL
    )?;
    let mut iegp = wdinfo::ubuntu::DBinfo::init(
        "192.168.1.213".to_owned(),
        "4512".to_owned(),    // Greenplum in docker
        "mydb".to_owned(),    // Greenplum in docker
        "sinnud".to_owned(),
        "password".to_owned(),    // Greenplum in docker
    )?;

    let skm="wdinfo".to_owned();
    for tbl in &tbllist {
        // let tbl="music243".to_owned();
        let csvfile=format!("/tmp/{}_db.csv", tbl);
        let ddlfile=format!("/tmp/{}.sql", tbl);
        if iegp.psql_chk_tbl_exist(&skm, &tbl)?{
            println!("target table exists, truncate it!");
            iegp.psql_truncate_tbl(&skm, &tbl)?;
        } else {
            println!("target table does not exist!");
            let filepath=&ddlfile;
            iepg.psql_tbl_ddl_gen("5432", &skm, &tbl, filepath)?;
            iegp.tbl_ddl_scp(filepath)?;
            iegp.create_tbl_use_ddl(filepath)?;
            iegp.tbl_ddl_clean(filepath)?;
        };
        iepg.export(&skm, &tbl, &csvfile)?;
        iegp.import(&skm, &tbl, &csvfile)?;
        iegp.local_clean(&csvfile)?;
    }
    Ok(())
}