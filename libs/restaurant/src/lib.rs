// // #[cfg(test)]
// mod sinnudtests {
//     // #[test]
//     pub fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
// pub fn libtest(){
//     sinnudtests::it_works();
// }

mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // front_of_house::add_to_waitlist();
}