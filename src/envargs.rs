/*! check command line argument */
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