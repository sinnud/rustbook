/*! WD sync tool
 * Use PostgreSQL and FileStatus
 */
#[allow(unused_imports)]
use crate::postgresql::PostgreSQL;
#[allow(unused_imports)]
use crate::file_status::FileStatus;
/** # WDInfo struct
 * pg as PostgreSQL connection (public)
 * temporary schema, default `wdinfo`
 * temporary table for PostgreSQL::import_data, default `_file_st`
 * table structure for temporary table (string)
 * inserted timestamp (not used?)
 * key variable for tables, default `fullpath`
 * prefix string for 192.168.1.241, default `//192.168.1.241/public/`
 * prefix string for 192.168.1.243, default `//192.168.1.243/`
 */
pub struct WDInfo{
    pub pg: PostgreSQL,
    tmp_skm: String,
    tmp_tbl: String,
    tbl_str: String,
    insdt: String,
    keyvar: String,
    pre241: String,
    pre243: String,
}
impl Default for WDInfo {
    /** # default function for WDInfo */
    #[allow(dead_code)]
    fn default() -> Self {
        WDInfo{
            pg: PostgreSQL::default(),
            tmp_skm: "wdinfo".to_owned(),
            tmp_tbl: "_file_st".to_owned(),
            tbl_str: "filename text, folder text, type text, fullpath text, filesize bigint, filecreate_dt timestamp".to_owned(),
            insdt: "inserted_dt".to_owned(),
            keyvar: "fullpath".to_owned(),
            pre241: "//192.168.1.241/public/".to_owned(),
            pre243: "//192.168.1.243/".to_owned(),
        }
    }
}
impl WDInfo {
    /** # WD refresh: refresh PostgreSQL table based on files on WD net drive
     * Like one interface of this library
     * Calls self.fs_import_pg and self.wdinfo_refresh
     */
    #[allow(dead_code)]
    pub fn wdrefresh(&mut self,
        path: &str,
        skm: &str,
        tbl: &str,
    ) -> Result<(), &'static str> {
        match self.fs_import_pg(path){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::wdfresh(): fs_import_pg {:?}", err);
                return Err("Failed to run WDInfo::wdfresh!");
            }
        };
        match self.wdinfo_refresh(skm, tbl){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::wdfresh(): wdinfo_refresh {:?}", err);
                return Err("Failed to run WDInfo::wdfresh!");
            }
        };    
        Ok(())
    }
    /** # WD sync: sync WD net drive and PostgreSQL table based on records in tables
     * Like one interface of this library
     * Calls self.wdinfo_compare and self.wdinfo_sync
     */
    #[allow(dead_code)]
    pub fn wdsync(&mut self,
        skm: &str,
        newtbl: &str,
        oldtbl: &str,
    ) -> Result<(), &'static str> {
        let pathlist = match self.wdinfo_compare(skm, newtbl, oldtbl){
            Ok(res) => res,
            Err(err) => {
                error!("in WDInfo::wdsync(): wdinfo_compare {:?}", err);
                return Err("Failed to run WDInfo::wdsync!");
            }
        };
        if pathlist.len()>0{
            match self.wdinfo_sync(skm, newtbl, oldtbl, pathlist){
                Ok(_) => (),
                Err(err) => {
                    error!("in WDInfo::wdsync(): wdinfo_sync {:?}", err);
                    return Err("Failed to run WDInfo::wdsync!");
                }
            }                    
        }
        Ok(())
    }
    /** # WDInfo initialization, other than default
     * use customer defined PostgreSQL connection arguments
     * Other private elements are still using default
     */
    #[allow(dead_code)]
    pub fn new(host: String,
        username: String,
        password: String,
        database: String,
    ) -> Result<Self, &'static str> {
        Ok(WDInfo {
            pg: PostgreSQL::new(host, username, password, database)?,
            ..Self::default()
        })
    }
    /** # WDInfo initialization, other than default
     * use customer defined PostgreSQL connection arguments
     * And other private elements
     */
    #[allow(dead_code)]
    pub fn new_special(host: String,
        username: String,
        password: String,
        database: String,
        tmp_skm: String,
        tmp_tbl: String,
        tbl_str: String,
        insdt: String,
        keyvar: String,
        pre241: String,
        pre243: String,
    ) -> Result<Self, &'static str> {
        Ok(WDInfo {
            pg: PostgreSQL::new(host, username, password, database)?,
            tmp_skm: tmp_skm,
            tmp_tbl: tmp_tbl,
            tbl_str: tbl_str,
            insdt: insdt,
            keyvar: keyvar,
            pre241: pre241,
            pre243: pre243,
        })
    }
    /** # Check last time refresh of PostgreSQL table based on WD net drive
     * (not used?)
     */
    #[allow(dead_code)]
    pub fn last_insert_dt(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<String, &'static str> {
        let qry=format!("select max({})::text from {}.{}", self.insdt, skm, tbl);
        info!("in WDInfo::last_insert_dt() query is\n{}", qry);
        let qry=&qry[..];
        let row = match self.pg.conn.query_one(qry, &[]){
            Ok(row) => row,
            Err(err) => {
                error!("in WDInfo::last_insert_dt(): query_one {:?}", err);
                return Err("Failed to run WDInfo::last_insert_dt!");
            }
        };
        let dt: String = row.get(0);
        Ok(dt)
    }
    /** # Check WD net drive, import FileStatus into PostgreSQL temporary table
     * The time consuming function: log start and finish status
     * call FileStatus::get_file_status_under_folder with dilimeter "|", then
     * call self.pg.import_data with query using dilimeter "|"
     */
    #[allow(dead_code)]
    pub fn fs_import_pg(&mut self,
        path: &str,
    ) -> Result<(), &'static str> {
        info!("in WDInfo::fs_import_pg({}) start...", path);
        // let mntpoint=if path contains "/public/" then "//192.168.1.241/" else "//192.168.1.243/";
        let mntpoint=if path.contains("/public/"){"//192.168.1.241/"} else {"//192.168.1.243/"};
        let fs=match FileStatus::get_file_status_under_folder(path, "|", &mntpoint){
            Ok(res) => res,
            Err(err) => {
                error!("in WDInfo::fs_import_pg(): get_file_status_under_folder ({}) {:?}", path, err);
                return Err("Failed to run WDInfo::fs_import_pg!");
            }
        };
        // Counting Newlines Really Fast code from https://llogiq.github.io/2016/09/24/newline.html
        info!("in WDInfo::fs_import_pg() file number is {}", fs.as_bytes().iter().filter(|&&c| c == b'\n').count());
        
        match self.pg.create_truncate_table(&self.tmp_skm, &self.tmp_tbl, &self.tbl_str){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::fs_import_pg(): create_truncate_table {:?}", err);
                return Err("Failed to run WDInfo::fs_import_pg!");
            }
        };
        let qry=format!("COPY {}.{} FROM STDIN DELIMITER '|'", self.tmp_skm, self.tmp_tbl);
        debug!("in WDInfo::fs_import_pg() query is\n{}", qry);
        let qry=&qry[..];
        match self.pg.import_data(qry, fs){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::fs_import_pg(): import_data {:?}", err);
                return Err("Failed to run WDInfo::fs_import_pg!");
            }
        };
        info!("in WDInfo::fs_import_pg() finished!");
        Ok(())
    }
    /** # update table using temporary table
     * Not used?
     */
    #[allow(dead_code)]
    pub fn wdinfo_update(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<(), &'static str> {
        // select a.* from wdinfo.movie241 a left join wdinfo._file_st B
        // on a.filename=b.filename
        // where b.folder is null;
        // delete records in table but already deleted from WD net drive
        let qry=format!("delete from {}.{} where {} in (select a.{} from {}.{} a left join {}.{} b on a.{}=b.{} where b.{} is null)",
                        skm, tbl, self.keyvar, self.keyvar, skm, tbl, self.tmp_skm, self.tmp_tbl, self.keyvar, self.keyvar, self.keyvar,
                        );
        info!("in WDInfo::wdinfo_update query 1: \n{}", qry);
        let qry=&qry[..];
        match self.pg.execute(qry, &[]){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::wdinfo_update(): execute {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_update!");
            }
        };
        // select b.* from wdinfo.movie241 a right join wdinfo._file_st B
        // on a.filename=b.filename
        // where a.folder is null;
        // insert records into table which is new from WD net drive
        let qry=format!("insert into {}.{} select b.*, now()::timestamp from {}.{} a right join {}.{} b on a.{}=b.{} where a.{} is null",
                        skm, tbl, skm, tbl, self.tmp_skm, self.tmp_tbl, self.keyvar, self.keyvar, self.keyvar,
                       );
        info!("in WDInfo::wdinfo_update query 2: \n{}", qry);
        let qry=&qry[..];
        match self.pg.execute(qry, &[]){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::wdinfo_update(): execute {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_update!");
            }
        };
        Ok(())
    }
    /** # refresh table using temporary table
     * truncate table
     * insert into table using now()::timestamp
     */
    #[allow(dead_code)]
    pub fn wdinfo_refresh(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<(), &'static str> {
        match self.pg.truncate_table(&skm, &tbl){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::wdinfo_refresh(): truncate_table {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_refresh!");
            }
        };
        let qry=format!("insert into {}.{} select *, now()::timestamp from {}.{}",
                        skm, tbl, self.tmp_skm, self.tmp_tbl,
                       );
        debug!("in WDInfo::wdinfo_refresh query: \n{}", qry);
        let qry=&qry[..];
        match self.pg.execute(qry, &[]){
            Ok(_) => (),
            Err(err) => {
                error!("in WDInfo::wdinfo_refresh(): execute {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_refresh!");
            }
        };
        Ok(())
    }
    /** # compare tables for records represent different WD net drive for sync later
     * call self.pg.query with return Vec<Row>
     * convert return to Vec<String>
     */
    // select a.* from music243 a left join music241 b on substr(a.fullpath,16)=substr(b.fullpath,23) 
    // where b.fullpath is null;
    #[allow(dead_code)]
    pub fn wdinfo_compare(&mut self,
        skm: &str,
        newtbl: &str,
        oldtbl: &str,
    ) -> Result<Vec<String>, &'static str> {
        let mut qry=format!("select a.{} from {}.{} a left join {}.{} b ",
                        self.keyvar, skm, newtbl, skm, oldtbl,
                       );
        let qry2=format!("on substr(a.{}, {})=substr(b.{}, {}) where b.{} is null",
                        self.keyvar, self.pre243.len().to_string(), 
                        self.keyvar, self.pre241.len().to_string(), self.keyvar,
                       );
        qry.push_str(&qry2);
        debug!("In WDInfo::wdinfo_compare query: \n{}", qry);
        let qry=&qry[..];
        let rows = match self.pg.query(qry, &[]){
            Ok(res) => res,
            Err(err) => {
                error!("In WDInfo::wdinfo_compare(): query {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_compare!");
            }
        };
        info!("In wdinfo_compare, new file number: {}", rows.len());
        let mut v: Vec<String> = Vec::with_capacity(rows.len());
        for row in rows{
            let elm: String=row.get(0);
            v.push(elm);
        }
        debug!("New file list:\n{}", v.join("\n"));
        Ok(v)
    }
    /** # sync WD net drive for one file
     * call FileStatus::copy_file
     * update table for this one record (append)
     * Consider file name with single quotation (PostgreSQL use two single quotation represent one single quotation)
     * path in tables start with //IP_address; convert to /mnt for copying file
     */
    #[allow(dead_code)]
    pub fn wdinfo_sync_one(&mut self,
        skm: &str,
        newtbl: &str,
        oldtbl: &str,
        fullpath: &str,
    ) -> Result<(), &'static str> {
        let path243=fullpath.replace(&self.pre243, "/mnt/");
        let path241=fullpath.replace(&self.pre243, "/mnt/public/");
        debug!("fullpath is {}, path243 is {}, path241 is {}.", fullpath, path243, path241);
        match FileStatus::copy_file(&path243, &path241){
            Ok(_) => (),
            Err(err) => {
                error!("In wdinfo_sync_one, copy_file errored:\n{}", err);
                return Err("Failed to wdinfo_sync_one.");
            }
        };
        let path=std::path::PathBuf::from(fullpath);
        let base=path.file_name().unwrap().to_str().unwrap();
        let dir=path.parent().unwrap().to_str().unwrap();
        let dir241=dir.replace(&self.pre243, &self.pre241).replace("'", "''");
        let full241=format!("{}/{}", &dir241, &base).replace("'", "''");
        let fullstr=fullpath.to_owned().replace("'", "''");
        // info!("in wdinfo_sync_one, folder: {} and fullpath: {}", &dir241, &full241);
        // filename | folder | type | fullpath | filesize | filecreate_dt | inserted_dt
        let qry=format!("insert into {}.{} select filename, '{}', type, '{}', filesize, filecreate_dt, now()::timestamp from {}.{} where {}='{}'",
            skm, oldtbl, &dir241, &full241, skm, newtbl, self.keyvar, &fullstr);
        // info!("In WDInfo::wdinfo_sync_one query: \n{}", qry);
        let qry=&qry[..];
        match self.pg.execute(qry, &[]){
            Ok(_) => (),
            Err(err) => {
                error!("In wdinfo_sync_one, execute errored:\n{}", err);
                error!("Query is: {}.", qry);
                return Err("Failed to wdinfo_sync_one.");
                }
        }
        Ok(())
    }
    
    /** # sync WD net drive for giving file list
     * call self.wdinfo_sync_one
     * time consuming process, with log of start and finished
     */
    #[allow(dead_code)]
    pub fn wdinfo_sync(&mut self,
        skm: &str,
        newtbl: &str,
        oldtbl: &str,
        pathlist: Vec<String>,
    ) -> Result<(), &'static str> {
        info!("Sync {} to {} start...", newtbl, oldtbl);
        for fullpath in pathlist{
            match self.wdinfo_sync_one(&skm, newtbl, oldtbl, &fullpath){
                Ok(_) => (),
                Err(err) => {
                    error!("In wdinfo_sync, wdinfo_sync_one({}) errored:\n{}", &fullpath, err);
                    return Err("Failed to wdinfo_sync.");
                }
            }
        }
        info!("Sync {} to {} Finished!", newtbl, oldtbl);
        Ok(())
    }
}