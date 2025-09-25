pub mod nested;
mod inaccessible;

pub fn function() {
    inaccessible::public_function();
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}

pub fn indirect_access2() {
    print!("called `2222`, that\n> ");

}