// #[macro_use]
extern crate log;
extern crate log4rs;
#[allow(unused_imports)]
use wdinfo::sqltrait::SQL;
/** # sync WDInfo tables from PostgreSQL into Greenplum in docker
 * Greenplum in docker start (maybe merge then into docker and rust code later):
 - ssh -i .ssh/ubuntu_sinnud.pem sinnud@192.168.1.213
   - cd /var/local/docker/greenplum-oss-docker/gpdb
   - sudo ./run_ossdocker.sh
     - startGPDB.sh
     - su - gpadmin
       - psql
         - create user sinnud WITH PASSWORD 'password';
         - create database mydb;
         - GRANT ALL PRIVILEGES ON  DATABASE mydb TO sinnud;
         - ALTER USER sinnud WITH ENCRYPTED PASSWORD 'password';
         - \q
       - psql -U sinnud -h localhost -d mydb
         - create schema wdinfo;
         - \q
 * After investigating docker, we can reduce the above command as: 
  - ssh -i .ssh/ubuntu_sinnud.pem sinnud@192.168.1.213
    - cd /var/local/docker/greenplum-oss-docker/gpdb
    - make sure we have scripts folder with two files gpinit.bash and gpinit2.bash (see contents below)
    - sudo ./run_ossdocker.sh &
    - sudo docker exec -it gpdb5oss startGPDB.sh
    - sudo docker exec -it gpdb5oss su - gpadmin /code/scripts/gpinit.bash
    - sudo docker exec -it gpdb5oss su - gpadmin /code/scripts/gpinit2.bash
    - content of gpinit.bash and gpinit2.bash
```
script file: gpinit.bash
psql -c "create user sinnud WITH PASSWORD 'password';"
psql -c "create database mydb;"
psql -c "GRANT ALL PRIVILEGES ON  DATABASE mydb TO sinnud;"
psql -c "ALTER USER sinnud WITH ENCRYPTED PASSWORD 'password';"
```
```
script file: gpinit2.bash
PGPASSWORD=password psql -U sinnud -h localhost -d mydb -c "create schema wdinfo;"
```
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
    let filename="/mnt/public/data/other/pem/config_pg_sinnud";
    let pem = wdinfo::pem::db_pem(filename).unwrap();
    let ip = &pem[0];
    let port = &pem[1];
    let database = &pem[2];
    let username = &pem[3];
    let password = &pem[4];
    let mut iepg = wdinfo::ubuntu::DBinfo::init(
        ip.to_string(), port.to_string(), database.to_string(), username.to_string(), password.to_string()
    )?;
    let filename="/mnt/public/data/other/pem/config_pg_sinnud";
    let pem = wdinfo::pem::db_pem(filename).unwrap();
    let ip = &pem[0];
    let port = &pem[1];
    let database = &pem[2];
    let username = &pem[3];
    let password = &pem[4];
    let mut iegp = wdinfo::ubuntu::DBinfo::init(
        ip.to_string(), port.to_string(), database.to_string(), username.to_string(), password.to_string()
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