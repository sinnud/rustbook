
#[allow(unused_imports)]
use crate::postgresql::PostgreSQL;
#[allow(unused_imports)]
use crate::file_status::FileStatus;

pub struct WDInfo{
    pub pg: PostgreSQL,
    tmp_skm: String,
    tmp_tbl: String,
    tbl_str: String,
    insdt: String,
    keyvar: String,
}
impl Default for WDInfo {
    #[allow(dead_code)]
    fn default() -> Self {
        WDInfo{
            pg: PostgreSQL::default(),
            tmp_skm: "wdinfo".to_owned(),
            tmp_tbl: "_file_st".to_owned(),
            tbl_str: "filename text, folder text, type text, fullpath text, filesize bigint, filecreate_dt timestamp".to_owned(),
            insdt: "inserted_dt".to_owned(),
            keyvar: "fullpath".to_owned(),
        }
    }
}
impl WDInfo {
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
    ) -> Result<Self, &'static str> {
        Ok(WDInfo {
            pg: PostgreSQL::new(host, username, password, database)?,
            tmp_skm: tmp_skm,
            tmp_tbl: tmp_tbl,
            tbl_str: tbl_str,
            insdt: insdt,
            keyvar: keyvar,
        })
    }
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
                println!("Error message in WDInfo::last_insert_dt(): query_one {:?}", err);
                return Err("Failed to run WDInfo::last_insert_dt!");
            }
        };
        let dt: String = row.get(0);
        Ok(dt)
    }
    #[allow(dead_code)]
    pub fn fs_import_pg(&mut self,
        path: &str,
        mntpoint: &str,
    ) -> Result<(), &'static str> {
        let fs=match FileStatus::get_file_status_under_folder(path, "|", mntpoint){
            Ok(res) => res,
            Err(err) => {
                println!("Error message in WDInfo::fs_import_pg(): get_file_status_under_folder ({}) {:?}", path, err);
                return Err("Failed to run WDInfo::fs_import_pg!");
            }
        };
        // Counting Newlines Really Fast code from https://llogiq.github.io/2016/09/24/newline.html
        info!("in WDInfo::fs_import_pg() file number is {}", fs.as_bytes().iter().filter(|&&c| c == b'\n').count());
        
        match self.pg.create_truncate_table(&self.tmp_skm, &self.tmp_tbl, &self.tbl_str){
            Ok(_) => (),
            Err(err) => {
                println!("Error message in WDInfo::fs_import_pg(): create_truncate_table {:?}", err);
                return Err("Failed to run WDInfo::fs_import_pg!");
            }
        };
        let qry=format!("COPY {}.{} FROM STDIN DELIMITER '|'", self.tmp_skm, self.tmp_tbl);
        info!("in WDInfo::fs_import_pg() query is\n{}", qry);
        let qry=&qry[..];
        match self.pg.import_data(qry, fs){
            Ok(_) => (),
            Err(err) => {
                println!("Error message in WDInfo::fs_import_pg(): import_data {:?}", err);
                return Err("Failed to run WDInfo::fs_import_pg!");
            }
        };
        info!("in WDInfo::fs_import_pg() finished!");
        Ok(())
    }
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
                println!("Error message in WDInfo::wdinfo_update(): execute {:?}", err);
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
                println!("Error message in WDInfo::wdinfo_update(): execute {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_update!");
            }
        };
        Ok(())
    }
    #[allow(dead_code)]
    pub fn wdinfo_refresh(&mut self,
        skm: &str,
        tbl: &str,
    ) -> Result<(), &'static str> {
        match self.pg.truncate_table(&skm, &tbl){
            Ok(_) => (),
            Err(err) => {
                println!("Error message in WDInfo::wdinfo_refresh(): truncate_table {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_refresh!");
            }
        };
        let qry=format!("insert into {}.{} select *, now()::timestamp from {}.{}",
                        skm, tbl, self.tmp_skm, self.tmp_tbl,
                       );
        info!("in WDInfo::wdinfo_refresh query: \n{}", qry);
        let qry=&qry[..];
        match self.pg.execute(qry, &[]){
            Ok(_) => (),
            Err(err) => {
                println!("Error message in WDInfo::wdinfo_refresh(): execute {:?}", err);
                return Err("Failed to run WDInfo::wdinfo_refresh!");
            }
        };
        Ok(())
    }
}