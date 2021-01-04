/** # main function executable program start from.
 
- Use return Result<(), &'static str> such that ? operator can be used if error is &str.
- Assume log4rs.yaml file is stored under folder config
under the same folder the executable program located `config/log4rs.yaml`.
- Assume log folder is located the same folder the executable program located `log/wdinfo.log`.
  - When the production (released) program is executed from other folder, log folder 
    defined in log4rs.yaml need to manually created with full path
  - log file will be archived using timestamp at the beginning of run.
- Support command line arguments. See [envargs](../lib_wd/envargs/index.html).
 */
/*
fn main()-> Result<(), &'static str>{
    let root=wdinfo::file_status::log_config_path()?;
    
    // log file is log/wdinfo.log. see config/log4ts.yaml
    let logfile=format!("{}/log/wdinfo.log", root);
    let log4rs_yaml=format!("{}/config/log4rs.yaml", root);
    
    wdinfo::file_status::rename_log_with_timestamp(&logfile)?;
    log4rs::init_file(&log4rs_yaml, Default::default()).unwrap();

    let opt=envargs()?;
    println!("Work on {}...", opt);
    
    let mut wd=wdinfo::wdinfo::WDInfo::default();

    if opt.contains("music"){
        wd.wdrefresh("/mnt/music", "wdinfo", "music243")?;
        wd.wdrefresh("/mnt/public/music", "wdinfo", "music241")?;
        wd.wdsync("wdinfo", "music243", "music241")?;
    }

    if opt.contains("data"){
        wd.wdrefresh("/mnt/data", "wdinfo", "data243")?;
        wd.wdrefresh("/mnt/public/data", "wdinfo", "data241")?;
        wd.wdsync("wdinfo", "data243", "data241")?;
    }

    if opt.contains("photos"){
        wd.wdrefresh("/mnt/photos", "wdinfo", "photos243")?;
        wd.wdrefresh("/mnt/public/photos", "wdinfo", "photos241")?;
        wd.wdsync("wdinfo", "photos243", "photos241")?;
    }

    if opt.contains("movie"){
        wd.wdrefresh("/mnt/movie", "wdinfo", "movie243")?;

        wd.wdrefresh("/mnt/public/newmovies", "wdinfo", "movie241")?;
    }
    
    Ok(())
}
/** # check command line argument
 - no argument is default, do all works: data, music, photos, movie
 - can be arbitrary argument(s) from the above four
 - If duplicated, choose one
 - if other than above four, error out
 - return clean string
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
*/
fn main()-> Result<(), &'static str>{
    let mut ms=wdinfo::libmysql::LibMySQL::default();
    Ok(())
}