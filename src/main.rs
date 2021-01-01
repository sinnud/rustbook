// log to file
#[macro_use]
extern crate log;
extern crate log4rs;

#[allow(unused_imports)]
use crate::wdinfo::WDInfo;

mod postgresql;
mod file_status;
mod wdinfo;

#[allow(unused_imports)]
use crate::file_status::rename_log_with_timestamp;


fn main()-> Result<(), &'static str>{
    // see config/log4ts.yaml
    rename_log_with_timestamp("log/wdinfo.log")?;
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    
    let mut wd=WDInfo::default();

    wd.wdrefresh("/mnt/music", "wdinfo", "music243")?;
    wd.wdrefresh("/mnt/public/music", "wdinfo", "music241")?;
    wd.wdsync("wdinfo", "music243", "music241")?;

    wd.wdrefresh("/mnt/data", "wdinfo", "data243")?;
    wd.wdrefresh("/mnt/public/data", "wdinfo", "data241")?;
    wd.wdsync("wdinfo", "data243", "data241")?;

    wd.wdrefresh("/mnt/photos", "wdinfo", "photos243")?;
    wd.wdrefresh("/mnt/public/photos", "wdinfo", "photos241")?;
    wd.wdsync("wdinfo", "photos243", "photos241")?;

    wd.wdrefresh("/mnt/movie", "wdinfo", "movie243")?;

    wd.wdrefresh("/mnt/public/newmovies", "wdinfo", "movie241")?;

    Ok(())
}
