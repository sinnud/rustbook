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
    #[allow(dead_code)]
    pub fn delete_file(path: &str, 
    ) -> Result<(), &'static str> {
        let md = match std::fs::metadata(path){
            Ok(res) => res,
            Err(err) => {
                println!("In FileStatus::delete_file(), std::fs::metadata errored:\n{}: {}", path, err);
                return Err("Failed to delete_file!");
            },
        };
        if md.is_file(){
            match std::fs::remove_file(path){
                Ok(_) => (),
                Err(err) => {
                    println!("In FileStatus::delete_file(), std::fs::remove_file errored:\n{}", err);
                    return Err("Failed to delete_file!");
                },
            }
        } else {
            println!("In FileStatus::delete_file({}), it is not a file!", path);
            return Err("Failed to delete_file!");
        }
        Ok(())
    }
    #[allow(dead_code)]
    pub fn rename_file(ori_path: &str, 
        dest_path: &str,
    ) -> Result<(), &'static str> {
        let md = match std::fs::metadata(ori_path){
            Ok(res) => res,
            Err(err) => {
                println!("In FileStatus::rename_file(), std::fs::metadata errored:\n{}: {}", ori_path, err);
                return Err("Failed to rename_file!");
            },
        };
        if !md.is_file(){
            println!("In FileStatus::rename_file({}), it is not a file!", ori_path);
            return Err("Failed to rename_file!");
        }
        if std::path::Path::new(dest_path).exists(){
            println!("In FileStatus::rename_file(), destination file {} already exists!", dest_path);
            return Err("Failed to rename_file!");
        }
        match std::fs::rename(ori_path, dest_path){
            Ok(_) => (),
            Err(err) => {
                println!("In FileStatus::rename_file(), std::fs::rename errored:\n{}", err);
                return Err("Failed to rename_file!");
            },
        };
        Ok(())
    }
    #[allow(dead_code)]
    pub fn copy_file(ori_path: &str, 
        dest_path: &str,
    ) -> Result<(), &'static str> {
        let md = match std::fs::metadata(ori_path){
            Ok(res) => res,
            Err(err) => {
                println!("In FileStatus::copy_file(), std::fs::metadata errored:\n{}: {}", ori_path, err);
                return Err("Failed to copy_file!");
            },
        };
        if !md.is_file(){
            println!("In FileStatus::copy_file({}), it is not a file!", ori_path);
            return Err("Failed to copy_file!");
        }
        if std::path::Path::new(dest_path).exists(){
            println!("In FileStatus::copy_file(), destination file {} already exists!", dest_path);
            return Err("Failed to copy_file!");
        }
        match std::fs::copy(ori_path, dest_path){
            Ok(_) => (),
            Err(err) => {
                println!("In FileStatus::copy_file(), std::fs::copy errored:\n{}", err);
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
    #[test]
    fn test_rename_file() {
        let ori_filename="ori.txt";
        let dest_filename="dest.txt";
        FileStatus::rename_file(ori_filename, dest_filename).unwrap();
    }
}

