/*! file status related functions and methods */
// #[allow(unused_imports)]
// #[macro_use]
// extern crate log;
// extern crate log4rs;

// walk dir
extern crate walkdir;
use walkdir::WalkDir;
use std::path::PathBuf;

// date and time
extern crate chrono;
#[allow(unused_imports)]
use chrono::offset::{Utc, Local};
use chrono::DateTime;
use std::time::SystemTime;

/** # get root path for `config/log4rs.yaml`
 ## detail
 - when we run `cargo run` from root path of project, executable file is under target/debug
 - when we run `cargo build --release` from root path of project, executable file
  is under target/release
 - when we copy our target executable file to production, we assume `config/log4rs.yaml`
  is located at the same folder with target executable file.
*/
#[allow(dead_code)]
pub fn log_config_path() -> Result<String, &'static str> {
    /*! # get root path of development (debug or release) or production
     - the sub folder config and log is under root.
     - For debug development, executable file is under root/target/debug
     - For release development, executable file is under root/target/release
     - For production, executable file is under root directly
    */ 
    let pathexe = match std::env::current_exe(){
        Ok(res) => res,
        Err(err) => {
            error!("In log_config_path, current_exe(): {}", err);
            return Err("log_config_path, failed!");
        }
    };
    let root=pathexe.parent().unwrap(); // drop execute file
    let parent=root.file_name().unwrap().to_str().unwrap();  // get release or debug, or production parent folder
        
    let res = if parent == "debug" || parent == "release" {
        root.parent().unwrap() // debug or release
            .parent().unwrap() // target
            .to_str().unwrap()
    } else {
        root.to_str().unwrap()
    };
    Ok(res.to_owned())
}
/** # each run with separate log file by timestamp as suffix in file name 
 * log4rs will create file if not exist, but just append if exist
 * If log4rs just create it, it is empty, no backup needed
 * Create suffix using yyyymmdd_HHMMSS timestamp
 * Copy log file to backup file and delete it (like rename)
 * log4rs will still append (create if not exist)
*/
#[allow(dead_code)]
pub fn rename_log_with_timestamp(pathstr: &str) -> Result<(), &'static str> {
    let path=PathBuf::from(&pathstr);
    let md = match std::fs::metadata(&path){
        Ok(res) => res,
        Err(err) => {
            error!("in rename_log_with_timestamp, log file {} does not exist: {}", pathstr, err);
            return Err("Check config/log4rs.yaml and log4rs!")
        }
    };
    if !md.is_file(){
        error!("in rename_log_with_timestamp, log {} is NOT a file!", pathstr);
        return Err("log file is not one file!")
    } 
    if md.len()==0{
        info!("{} is empty!", pathstr);
        return Ok(())
    }
    let system_time = SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();
    let post_ts=datetime.format("%Y%m%d_%H%M%S");
    let ext=if path.extension().is_none(){""}else{path.extension().unwrap().to_str().unwrap()};
    let base=path.file_stem().unwrap().to_str().unwrap();
    let dir=path.parent().unwrap().to_str().unwrap();
    let dest_file=if ext.len()==0{format!("{}/{}_{}", dir, base, post_ts)}else{format!("{}/{}_{}.{}", dir, base, post_ts, ext)};
    match FileStatus::copy_file(pathstr, &dest_file){
        Ok(_) => (),
        Err(err) => {
            error!("In rename_log_with_timestamp, copy_file({}, {}):{}", pathstr, &dest_file, err);
            return Err("Failed in rename_log_with_timestamp!");
        }
    };
    match FileStatus::delete_file(pathstr){
        Ok(_) => (),
        Err(err) => {
            error!("In rename_log_with_timestamp, delete_file({}):{}", pathstr, err);
            return Err("Failed in rename_log_with_timestamp!");
        }
    };
    Ok(())
}

/** # Get file status, along with basic file operation
 ## File status list:
 - folder (dir name of the file, not include file name)
 - file name 
 - isdir (bool)
 - type (just file extension)
 - file size 
 - timestamp for modifying
 - timestamp for accessing
 - timestamp for creating
 */
#[allow(dead_code)]
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
    /** # get file status
     * timestamp uses format "%Y-%m-%d %H:%M:%S" for PostgreSQL
     */
    #[allow(dead_code)]
    pub fn get_status(path: &PathBuf) -> Result<FileStatus, &'static str> {
        let ext=if path.extension().is_none(){""}else{path.extension().unwrap().to_str().unwrap()};
        // let base=path.file_stem().unwrap().to_str().unwrap();
        let filename=path.file_name().unwrap().to_str().unwrap();
        let dir=path.parent().unwrap().to_str().unwrap();
        let md = std::fs::metadata(path).unwrap();
        let res=FileStatus {
            path: dir.to_string()
            // , name: if ext.len()==0 {base.to_string()} else {format!("{}.{}",base,ext)}
            , name: filename.to_string()
            , isdir: md.is_dir()
            , filetype: ext.to_string()
            , size: md.len()
            ,dt_modified: if let Ok(time) = md.modified() {
                let datetime: chrono::DateTime<chrono::offset::Local> = time.into();
                // datetime.format("%Y%m%d_%H%M%S").to_string()
                datetime.format("%Y-%m-%d %H:%M:%S").to_string() // format for postgreSQL
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
    /** # get file status for all files under one folder
     - Output is String, not FileStatus struct
       - each line with one file status
       - the delimiter for fields in one line can be modified
       - new line symbol and delimiter are used when import data into PostgreSQL
     - argument path as string slide can be relative path from root path (executable file path)
     - argument mntpoint is used to replace /mnt by //Ip_address for workstation independence
     - The method walkdir::WalkDir is used for all sub folders
     */
    #[allow(dead_code)]
    pub fn get_file_status_under_folder(path: &str, 
        delimiter: &str,     // for data base import
        mntpoint: &str,      // instead of /mnt/public, use //192.168.1.241/public
                             //            /mnt/movie,  use //192.168.1.243/movie
    ) -> Result<String, &'static str> {
        let pathori=PathBuf::from(path);
        let pathcrt=if pathori.is_relative(){
            std::env::current_dir().unwrap().join(path)
        } else {pathori};
        // info!("path is: {}", pathcrt.to_str().unwrap());
        let dir=WalkDir::new(pathcrt.to_str().unwrap());
        let mut res="".to_string();
        for e in dir.into_iter().filter_map(|e| e.ok()){
            if e.metadata().unwrap().is_file(){
                // info!("e is: {:?}", e.path());
                let buf = PathBuf::from(e.path());
                let fs=FileStatus::get_status(&buf).unwrap();
                let mntpointpath=&fs.path.replace("/mnt/", mntpoint);
                // info!("fs is: {:?}", fs);
                res.push_str(&fs.name);res.push_str(delimiter);
                res.push_str(mntpointpath);res.push_str(delimiter);
                res.push_str(&fs.filetype);res.push_str(delimiter);
                res.push_str(mntpointpath);res.push_str("/");
                res.push_str(&fs.name);res.push_str(delimiter);
                res.push_str(&fs.size.to_string());res.push_str(delimiter);
                res.push_str(&fs.dt_created);res.push_str("\n");
            }
        }
        Ok(res)
    }
    /** # delete file
     * ignore others like folder, link, etc.
     */
    #[allow(dead_code)]
    pub fn delete_file(path: &str, 
    ) -> Result<(), &'static str> {
        let md = match std::fs::metadata(path){
            Ok(res) => res,
            Err(err) => {
                error!("In FileStatus::delete_file(), std::fs::metadata errored:\n{}: {}", path, err);
                return Err("Failed to delete_file!");
            },
        };
        if md.is_file(){
            match std::fs::remove_file(path){
                Ok(_) => (),
                Err(err) => {
                    error!("In FileStatus::delete_file({}), std::fs::remove_file errored:\n{}", path, err);
                    return Err("Failed to delete_file!");
                },
            }
        } else {
            error!("In FileStatus::delete_file({}), it is not a file!", path);
            return Err("Failed to delete_file!");
        }
        Ok(())
    }
    /** # copy file
     * Do not copy if destination file exist
     * Create destination folder if not exist
     * check destination file existence before calling this method to avoid error
     */
    #[allow(dead_code)]
    pub fn copy_file(ori_path: &str, 
        dest_path: &str,
    ) -> Result<(), &'static str> {
        let md = match std::fs::metadata(ori_path){
            Ok(res) => res,
            Err(err) => {
                error!("In FileStatus::copy_file(), std::fs::metadata errored:\n{}: {}", ori_path, err);
                return Err("Failed to copy_file!");
            },
        };
        if !md.is_file(){
            error!("In FileStatus::copy_file({}), it is not a file!", ori_path);
            return Err("Failed to copy_file!");
        }
        if std::path::Path::new(dest_path).exists(){
            error!("In FileStatus::copy_file(), destination file {} already exists!", dest_path);
            return Err("Failed to copy_file!");
        }
        let dir=std::path::Path::new(dest_path).parent().unwrap();
        if !dir.exists() {
            match std::fs::create_dir_all(dir.to_str().unwrap()){
                Ok(_) => (),
                Err(err) => {
                    error!("In FileStatus::copy_file({}, {}), std::fs::create_dir_all errored:\n{}", ori_path, dest_path, err);
                    return Err("Failed to copy_file!");
                }
            };
        }
        match std::fs::copy(ori_path, dest_path){
            Ok(_) => (),
            Err(err) => {
                error!("In FileStatus::copy_file({}, {}), std::fs::copy errored:\n{}", ori_path, dest_path, err);
                return Err("Failed to copy_file!");
            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

