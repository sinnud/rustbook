/*! MySQL related functions and metods */
// mysql
use mysql::*;
#[allow(unused_imports)]
use mysql::prelude::*;
// URL encoding for connecting to postgresql
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/** # URL encode
 * in case when your password have symbol out of URL set, like !@ (see src code)
 */
#[allow(dead_code)]
pub fn url_encode(ori: &str) -> String {
    /// https://url.spec.whatwg.org/#fragment-percent-encode-set
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"')
        .add(b'<').add(b'>').add(b'`')
        .add(b'!').add(b'@')
        ;
    let ori = utf8_percent_encode(ori, FRAGMENT).to_string();
    ori
}

/** # LibMySQL
 * Connect to MySQL
 * Execute queries to MySQL
 * import data into MySQL
 */
pub struct LibMySQL{
    pub conn: PooledConn,
}
/** # Default initialization of LibMySQL
 * use as defalt
 */
impl Default for LibMySQL {
    /** # default method
     * Use mysql::Pool method
     * MySQL database installed in 192.168.1.213
     */
    #[allow(dead_code)]
    fn default() -> Self {
        let pw_url=url_encode("Jeffery45!@");
        let constr=format!("mysql://sinnud:{}@192.168.1.213/test", pw_url);
        let pool = match mysql::Pool::new(&constr){
            Ok(res) => res,
            Err(err) => {
                error!("in default() for LibMySQL: {:?}", err);
                println!("Pool::new failed: {:?}", err);
                println!("Failed to connect database using default() function!");
                std::process::exit(1);
            }
        };
        LibMySQL{
            conn: match pool.get_conn(){
                Ok(res) => res,
                Err(err) =>{
                    error!("in default() for LibMyQL: {:?}", err);
                    println!("get_conn failed: {:?}", err);
                    println!("Failed to connect database using default() function!");
                    std::process::exit(1);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_default() {
        let mut ms=LibMySQL::default();
    }
}