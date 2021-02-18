/*! read database related pem file file disk */
/** # sync WDInfo /mnt/public/data/other/pem folder to .ssh
 * ubuntu_sinnud.pem  ubuntu_user.pem used for connect to 192.168.1.213 (ubuntu)
 * config_gp_sinnud config_pg_sinnud config_ms_sinnud user connection config to greenplum, postgresql, and mysql
 * 
 */
use std::io::BufRead;
pub fn db_pem(filename: &str) -> Result<Vec<String>, &'static str> {
    /*! # Giving pem file with pull path, read containt as pem info
     - ignore comment lines start with hash symbol
     - use first non-comment line
     - split using space
     - discard empty elements
    */ 
    info!("DEBUG: file name is {}", filename);
    let file = match std::fs::File::open(filename){
        Ok(res) => res,
        Err(err) => {
            error!("In pem::db_pem(), std::fs::File::open errored:\n{}: {}", filename, err);
            return Err("Failed to open pem file!");
        },
    };
    let reader = std::io::BufReader::new(file);

    let mut vs: Vec<String> = Vec::with_capacity(5);
    for line in reader.lines() {
        let mystr = match line{
            Ok(res) => res,
            Err(err) => {
                error!("In pem::db_pem(), get string from reader.lines errored:\n{}: {}", filename, err);
                return Err("Failed to get one line!");
            },
        };
        let ch = mystr.chars().nth(0).unwrap();
        if ch == '#' {continue;}
        let v: Vec<&str> = mystr.split(' ').collect();
        for thisstr in v{
            if thisstr.len() > 0 {vs.push(thisstr.to_owned());}
        }
        break;
        // let v: Vec<&str> = mystr.split(' ').collect();
        // return Ok(v);
        // println!("{:#?}", line);
    };

    Ok(vs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_file_containts() {
        let filename="/home/user/.ssh/config_pg_sinnud";
        let rst = db_pem(filename).unwrap();
        println!("Ip is {}, port is {}, database is {}, username is {}, and password is '{}'.", rst[0], rst[1], rst[2], rst[3], rst[4]);
        assert_eq!(filename, "~/.ssh/config_pg_sinnud");
    }
}

