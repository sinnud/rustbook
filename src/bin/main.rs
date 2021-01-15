// #[macro_use]
extern crate log;
extern crate log4rs;

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

    let opt=envargs()?;
    println!("Work on {}...", opt);
    
    //let mut wd=wdinfo::wdinfo::WDInfo::default();
    let pg=wdinfo::postgresql::PostgreSQL::default();
    let mut wdpg=wdinfo::wdinfo::WDInfo::initialization(pg);
    let ms=wdinfo::libmysql::LibMySQL::default();
    let mut wdms=wdinfo::wdinfo::WDInfo::initialization(ms);

    if opt.contains("music"){
        let fs = wdpg.fs_scan("/mnt/music")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "music243")?;
        wdms.wdrefresh(fsc, "wdinfo", "music243")?;
        let fs = wdpg.fs_scan("/mnt/public/music")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "music241")?;
        wdms.wdrefresh(fsc, "wdinfo", "music241")?;
        wdpg.wdsync("wdinfo", "music243", "music241")?;
    }
    if opt.contains("data"){
        let fs = wdpg.fs_scan("/mnt/data")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "data243")?;
        wdms.wdrefresh(fsc, "wdinfo", "data243")?;
        let fs = wdpg.fs_scan("/mnt/public/data")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "data241")?;
        wdms.wdrefresh(fsc, "wdinfo", "data241")?;
        wdpg.wdsync("wdinfo", "data243", "data241")?;
    }
    if opt.contains("photos"){
        let fs = wdpg.fs_scan("/mnt/photos")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "photos243")?;
        wdms.wdrefresh(fsc, "wdinfo", "photos243")?;
        let fs = wdpg.fs_scan("/mnt/public/photos")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "photos241")?;
        wdms.wdrefresh(fsc, "wdinfo", "photos241")?;
        wdpg.wdsync("wdinfo", "photos243", "photos241")?;
    }
    if opt.contains("movie"){
        let fs = wdpg.fs_scan("/mnt/movie")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "movie243")?;
        wdms.wdrefresh(fsc, "wdinfo", "movie243")?;
        let fs = wdpg.fs_scan("/mnt/public/newmovies")?;
        let fsc = fs.clone();
        wdpg.wdrefresh(fs, "wdinfo", "movie241")?;
        wdms.wdrefresh(fsc, "wdinfo", "movie241")?;
    }
    Ok(())
}
/** # handle command line arguments
 * Only allow data music photos movie
 * Repeat will be droped
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