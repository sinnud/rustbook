/*! read database related pem file file disk */
/** # sync WDInfo /mnt/public/data/other/pem folder to .ssh
 * ubuntu_sinnud.pem  ubuntu_user.pem used for connect to 192.168.1.213 (ubuntu)
 * config_gp_sinnud config_pg_sinnud config_ms_sinnud user connection config to greenplum, postgresql, and mysql
 * 
 */
use std::io::BufRead;
extern crate csv;

use std::fs::File;

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

use serde::Deserialize;
#[derive(Deserialize)]
struct Record {
    name: String,
    url: String,
    username: String,
    password: String,
}
pub fn web_login_from_csv(csvfile: &str, webname: &str) -> Result<Vec<String>, &'static str>{
    /*! Get login information from password file in csv format
     * password file have format of name,url,username,password
     * the argument webname need to match with name in password file
     * */
    let mut vs: Vec<String> = Vec::with_capacity(2);
    let file = File::open(csvfile).expect("Couldn't open input");
    // let mut csv_file = csv::Reader::from_reader(file).delimiter(b'|').has_headers(false);
    let mut reader = csv::Reader::from_reader(file);
    for record in reader.deserialize() {
        let record: Record = match record{
            Ok(res) => res,
            Err(err) => {
                error!("In Pem::web_login_from_csv(), failed to get records from {}:\n{}", csvfile, err);
                return Err("Failed to parse csv file!")
            }
        };
        if record.name.contains(webname) {
            let str = format!("{},{},{},{}", record.name, record.url, record.username, record.password);
            vs.push(str);
        }
    }

    Ok(vs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_file_containts() {
        let filename="/home/user/.ssh/password.csv";
        // let rst = db_pem(filename).unwrap();
        // println!("Ip is {}, port is {}, database is {}, username is {}, and password is '{}'.", rst[0], rst[1], rst[2], rst[3], rst[4]);
        let webname = "citi";
        let rst = web_login_from_csv(filename, webname).unwrap();
        println!("{:#?}", rst);
        assert_eq!(filename, "/home/user/.ssh/password.csv");
    }
}

