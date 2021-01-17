/*! transfer table through databases using rust to call ubuntu shell command
 * export table from postgresql (remote server) to local file
 * import table into postgresql (remote server) from local file
 * export table from greenplum (remote server under docker) to local file
 * import table into greenplum (remote server under docker) from local file
 */
// use url_encode
use crate::file_status::FileStatus;
/** struct with database information */
pub struct DBinfo {
    host: String,
    port: String,
    database: String,
    username: String,
    password: String,
}
impl DBinfo{
    /** initialization */
    #[allow(dead_code)]
    pub fn init(host: String,
                port: String,
                database: String,
                username: String,
                password: String,
    ) -> Result<Self, &'static str> {
        Ok(
            DBinfo {
                host: host,
                port: port,
                database: database,
                username: username,
                password: password,
            }
        )
    }
    /** check local file: delete if exist */
    #[allow(dead_code)]
    pub fn local_clean(&mut self, pathstr: &str) -> Result<(), &'static str>{
        let path=std::path::PathBuf::from(pathstr);
        let md = match std::fs::metadata(&path){
            Ok(res) => res,
            Err(err) => {
                debug!("temporary file {} does not exist: {}", pathstr, err);
                return Ok(())
            }
        };
        if md.is_file(){
            match FileStatus::delete_file(pathstr){
                Ok(_) => return Ok(()),
                Err(err) => {
                    error!("delete {} fail: {}!", pathstr, err);
                    return Err("log file is not one file!");
                }
            };
        }
        Ok(())
    }
    /** export table from database to local file
     *  Greenplum on remote server in docker
     * Postgresql in remote server
     */
    #[allow(dead_code)]
    pub fn export(&mut self,
                  skm: &str,
                  tbl: &str,
                  pathstr: &str,
    ) -> Result<(), &'static str>{
        match self.local_clean(pathstr){
            Ok(_) => (),
            Err(err) => {
                error!("Failed to clean {}: {}", pathstr, err);
                return Err("Export failed!");
            }
        };
        info!("Start exporting {}.{} to {}...", skm, tbl, pathstr);
        // let pw_phase=format!("PGPASSWORD={}", self.password);
        // let qry=r##;
        let qry=format!("\\copy {}.{} to '{}' delimiter ',' csv header;", skm, tbl, pathstr);
        // info!("DEBUG: qry is {}: {}", &self.password, qry);
        let output = std::process::Command::new("psql")
                         .env("PGPASSWORD", &self.password)
                         .arg("-h")
                         .arg(&self.host)
                         .arg("-p")
                         .arg(&self.port)
                         .arg("-d")
                         .arg(&self.database)
                         .arg("-U")
                         .arg(&self.username)
                         .arg("-c")
                         .arg(&qry)
                         .output()
                         .expect("failed to execute process");
        // info!("DEBUG stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed export: {}", String::from_utf8_lossy(&output.stderr));
            return Err("export failed.");
        }
        info!("Finish exporting {}.{} to {}.", skm, tbl, pathstr);
        Ok(())
    }
    /** import table from local file
     *  Greenplum on remote server in docker
     * Postgresql in remote server
     */
    #[allow(dead_code)]
    pub fn import(&mut self,
                  skm: &str,
                  tbl: &str,
                  pathstr: &str,
    ) -> Result<(), &'static str>{
        let path=std::path::PathBuf::from(pathstr);
        match std::fs::metadata(&path){
            Ok(res) => res,
            Err(err) => {
                error!("temporary file {} does not exist: {}", pathstr, err);
                return Err("Local data file DOES NOT exist!!!")
            }
        };
        info!("Start importing {}.{} from {}...", skm, tbl, pathstr);
        // let pw_phase=format!("PGPASSWORD={}", self.password);
        // let qry=r##;
        let qry=format!("\\copy {}.{} from '{}' delimiter ',' csv header;", skm, tbl, pathstr);
        // info!("DEBUG: qry is {}: {}", &self.password, qry);
        let output = std::process::Command::new("psql")
                         .env("PGPASSWORD", &self.password)
                         .arg("-h")
                         .arg(&self.host)
                         .arg("-p")
                         .arg(&self.port)
                         .arg("-d")
                         .arg(&self.database)
                         .arg("-U")
                         .arg(&self.username)
                         .arg("-c")
                         .arg(&qry)
                         .output()
                         .expect("failed to execute process");
        // info!("DEBUG stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed export: {}", String::from_utf8_lossy(&output.stderr));
            return Err("export failed.");
        }
        info!("Finish importing {}.{} from {}.", skm, tbl, pathstr);
        Ok(())
    }
    /** check table exsitence using psql
     * Greenplum on remote server in docker
     * Postgresql in remote server
     */
    #[allow(dead_code)]
    pub fn psql_chk_tbl_exist(&mut self,
                  skm: &str,
                  tbl: &str,
    ) -> Result<bool, &'static str>{
        let qry=format!("SELECT count(*) FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_TYPE='BASE TABLE' AND TABLE_SCHEMA='{}' AND TABLE_NAME='{}'",
            skm, tbl,
        );
        // info!("DEBUG: qry is {}: {}", &self.password, qry);
        let output = std::process::Command::new("psql")
                         .env("PGPASSWORD", &self.password)
                         .arg("-h")
                         .arg(&self.host)
                         .arg("-p")
                         .arg(&self.port)
                         .arg("-d")
                         .arg(&self.database)
                         .arg("-U")
                         .arg(&self.username)
                         .arg("-c")
                         .arg(&qry)
                         .output()
                         .expect("failed to execute process");
        // info!("DEBUG stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed export: {}", String::from_utf8_lossy(&output.stderr));
            return Err("export failed.");
        }
        let out=String::from_utf8_lossy(&output.stdout);
        let lines = out.lines();
        let vec: Vec<&str> = lines.collect();
        let res = vec[2].trim();
        // info!("Result: '{}'", res);
        if res == "0" {return Ok(false)}
        Ok(true)
    }
    /** create table ddl using psql
     * Greenplum on remote server in docker
     * Postgresql in remote server
     */
    #[allow(dead_code)]
    pub fn psql_tbl_ddl_gen(&mut self,
                  port: &str,
                  skm: &str,
                  tbl: &str,
                  dllfile: &str,
    ) -> Result<(), &'static str>{
        let cmd=format!("pg_dump -p {} --schema-only -t {}.{} -x dbhuge | grep -v '^SET' > {}", port, skm, tbl, dllfile);
        let output = std::process::Command::new("ssh")
                         .arg("-i")
                         .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                         .arg("sinnud@192.168.1.213")
                         .arg(&cmd)
                         .output()
                         .expect("failed to execute process");
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed ssh: {}", String::from_utf8_lossy(&output.stderr));
            return Err("ssh failed.");
        }
        Ok(())
    }
    /** scp table ddl from remote to local
     */
    #[allow(dead_code)]
    pub fn tbl_ddl_scp(&mut self,
                  ddlfile: &str,
    ) -> Result<(), &'static str>{
        //scp -i .ssh/ubuntu_user.pem user@192.168.1.213:/tmp/data243.sql /tmp
        let remote_ddl_file=format!("sinnud@192.168.1.213:{}", ddlfile);
        let output = std::process::Command::new("scp")
                         .arg("-i")
                         .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                         .arg(&remote_ddl_file)
                         .arg(ddlfile)
                         .output()
                         .expect("failed to execute process");
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed scp: {}", String::from_utf8_lossy(&output.stderr));
            return Err("scp failed.");
        }
        Ok(())
    }
    /** create table using local ddl
     */
    #[allow(dead_code)]
    pub fn create_tbl_use_ddl(&mut self,
                  ddlfile: &str,
    ) -> Result<(), &'static str> {
        let output = std::process::Command::new("psql")
                         .env("PGPASSWORD", &self.password)
                         .arg("-h")
                         .arg(&self.host)
                         .arg("-p")
                         .arg(&self.port)
                         .arg("-d")
                         .arg(&self.database)
                         .arg("-U")
                         .arg(&self.username)
                         .arg("-f")
                         .arg(ddlfile)
                         .output()
                         .expect("failed to execute process");
        // info!("DEBUG stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed create table using ddl: {}", String::from_utf8_lossy(&output.stderr));
            // return Err("create table failed.");
            return Ok(()) // allow DISTRIBUTED BY error.
        }
        Ok(())
    }
    /** truncate table using psql
     * Greenplum on remote server in docker
     * Postgresql in remote server
     */
    #[allow(dead_code)]
    pub fn psql_truncate_tbl(&mut self,
                  skm: &str,
                  tbl: &str,
    ) -> Result<(), &'static str>{
        let qry=format!("truncate TABLE {}.{}",
            skm, tbl,
        );
        let output = std::process::Command::new("psql")
                         .env("PGPASSWORD", &self.password)
                         .arg("-h")
                         .arg(&self.host)
                         .arg("-p")
                         .arg(&self.port)
                         .arg("-d")
                         .arg(&self.database)
                         .arg("-U")
                         .arg(&self.username)
                         .arg("-c")
                         .arg(&qry)
                         .output()
                         .expect("failed to execute process");
        // info!("DEBUG stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed truncate table using psql: {}", String::from_utf8_lossy(&output.stderr));
            return Err("truncate table failed.");
        }
        Ok(())
    }
    /** table ddl clean from both remote and local
     */
    #[allow(dead_code)]
    pub fn tbl_ddl_clean(&mut self,
                  ddlfile: &str,
    ) -> Result<(), &'static str>{
        let path=std::path::PathBuf::from(ddlfile);
        let md = match std::fs::metadata(&path){
            Ok(res) => res,
            Err(err) => {
                info!("temporary file {} does not exist: {}", ddlfile, err);
                return Ok(())
            }
        };
        if md.is_file(){
            match FileStatus::delete_file(ddlfile){
                Ok(_) => (),
                Err(err) => {
                    error!("delete {} fail: {}!", ddlfile, err);
                    return Err("failed to delete local ddl file!");
                }
            };
        }
        let output = std::process::Command::new("ssh")
                         .arg("-i")
                         .arg("/home/user/.ssh/ubuntu_sinnud.pem")
                         .arg("sinnud@192.168.1.213")
                         .arg("sudo")
                         .arg("rm")
                         .arg("-f")
                         .arg(ddlfile)
                         .output()
                         .expect("failed to execute process");
        if String::from_utf8_lossy(&output.stderr).len() > 0 {
            error!("Failed delete remote ddl file: {}", String::from_utf8_lossy(&output.stderr));
            return Err("delete remove ddl file failed.");
        }
        info!("DEBUG stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }
}