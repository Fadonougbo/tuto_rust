// mod my_mode {

//     pub fn get_age()->i32{
//         return 30;
//     }
    
//     fn get_number()->i32{
//         get_age()
//     }

//     pub fn get_ville()->String{
//         return String::from("John doe");
//     }

//     pub mod private_nested {

//         pub fn function()->i8 {
//             return 55;
//         }

//     }
// }

mod my;

use my::{function};

fn main() {
    my::nested::function();
}