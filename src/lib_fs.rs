// file status related libraries

// walk dir
extern crate walkdir;
use walkdir::WalkDir;
use std::path::PathBuf;

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
    #[allow(dead_code)]
    pub fn get_status(path: &PathBuf) -> Result<FileStatus, &'static str> {
        // println!("DEBUG in get_status: extension() is {:?}", path.extension());
        let ext=if path.extension().is_none(){""}else{path.extension().unwrap().to_str().unwrap()};
        // println!("DEBUG in get_status: ext is '{}' with length {}", ext, ext.len());
        let base=path.file_stem().unwrap().to_str().unwrap();
        let dir=path.parent().unwrap().to_str().unwrap();
        let md = std::fs::metadata(path).unwrap();
        let res=FileStatus {
            path: dir.to_string()
            , name: if ext.len()==0 {base.to_string()} else {format!("{}.{}",base,ext)}
            , isdir: md.is_dir()
            , filetype: ext.to_string()
            , size: md.len()
            ,dt_modified: if let Ok(time) = md.modified() {
                let datetime: chrono::DateTime<chrono::offset::Local> = time.into();
                // datetime.format("%Y%m%d_%H%M%S").to_string()
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
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
    #[allow(dead_code)]
    pub fn get_file_status_under_folder(path: &str, 
        delimiter: &str     // for data base import
    ) -> Result<String, &'static str> {
        let pathori=PathBuf::from(path);
        let pathcrt=if pathori.is_relative(){
            std::env::current_dir().unwrap().join(path)
        } else {pathori};
        // println!("DEBUG path is: {}", pathcrt.to_str().unwrap());
        let dir=WalkDir::new(pathcrt.to_str().unwrap());
        let mut res="".to_string();
        for e in dir.into_iter().filter_map(|e| e.ok()){
            if e.metadata().unwrap().is_file(){
                // println!("DEBUG e is: {:?}", e.path());
                let buf = PathBuf::from(e.path());
                let fs=FileStatus::get_status(&buf).unwrap();
                // println!("DEBUG fs is: {:?}", fs);
                res.push_str(&fs.name);res.push_str(delimiter);
                res.push_str(&fs.path);res.push_str(delimiter);
                res.push_str(&fs.filetype);res.push_str(delimiter);
                res.push_str(&fs.path);res.push_str("/");
                res.push_str(&fs.name);res.push_str(delimiter);
                res.push_str(&fs.size.to_string());res.push_str(delimiter);
                res.push_str(&fs.dt_created);res.push_str("\n");
            }
        }
        Ok(res)
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

