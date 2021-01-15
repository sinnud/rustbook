/*! TRAIT for data base like postgresql and mysql */

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
    let enc = utf8_percent_encode(ori, FRAGMENT).to_string();
    enc
}
/** # trait SQL
 * function definition. implementation will be in postgresql.rs and libmysql.rs (may more later)
 * With default define here.
 * One exception: create_truncate_table just define here since it just call functions defined here.
 */
pub trait SQL {
    fn execute_queries_no_return(&mut self, qry: &str) -> Result<(), &'static str>{
        error!("Check if you implement execute_queries_no_return({})!!!", qry);
        Err("See above...")
    }
    fn check_table_exists(&mut self, skm: &str, tbl: &str) -> Result<bool, &'static str>{
        error!("Check if you implement check_table_exists({}, {})!!!", skm, tbl);
        Err("See above...")
    }
    fn drop_table(&mut self, skm: &str, tbl: &str) -> Result<(), &'static str>{
        error!("Check if you implement drop_table({}, {})!!!", skm, tbl);
        Err("See above...")
    }
    fn truncate_table(&mut self, skm: &str, tbl: &str) -> Result<(), &'static str>{
        error!("Check if you implement truncate_table({}, {})!!!", skm, tbl);
        Err("See above...")
    }
    fn create_table(&mut self, skm: &str, tbl: &str, tbl_str: &str) -> Result<(), &'static str>{
        error!("Check if you implement create_table({}, {}, {})!!!", skm, tbl, tbl_str);
        Err("See above...")
    }
    fn create_truncate_table(&mut self, skm: &str, tbl: &str, tbl_str: &str) -> Result<(), &'static str>{
        if match self.check_table_exists(skm, tbl){
            Ok(res) => res,
            Err(err) => {
                error!("in SQL::check_table_exists: {:?}", err);
                return Err("Failed to run check_table_exists!");
            }
        }{
            match self.truncate_table(skm, tbl){
                Ok(_) => (),
                Err(err) => {
                    error!("in SQL::truncate_table: {:?}", err);
                    return Err("Failed to run truncate_table!");
                }
            };
        } else {
            match self.create_table(skm, tbl, tbl_str){
                Ok(_) => (),
                Err(err) => {
                    error!("in SQL::create_table: {:?}", err);
                    return Err("Failed to run create_table!");
                }
            };
        }
        Ok(())
    }
    fn import_data(&mut self, skm: &str, tbl: &str, datastring: String) -> Result<(), &'static str>{
        error!("Check if you implement create_table({}, {}, {})!!!", skm, tbl, &datastring);
        Err("See above...")
    }
    fn execute_query_with_return(&mut self, qry: &str) -> Result<Vec<String>, &'static str>{
        error!("Check if you implement execute_query_with_return({})!!!", qry);
        Err("See above...")
    }
}