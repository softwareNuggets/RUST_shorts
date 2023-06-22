/*
 * This code defines a function named "type_of"
 * that takes a generic parameter T
 *
 * This function has one parameter, but the 
 * underscore _ means that we don't actually 
 * use the parameter in the function body.
 * It's just there to satisfy the Rust syntax.
 *
 * The return type of this function 
 * is &'static str, 
 * which means it returns a reference to 
 * a string slice that has a static lifetime.
 */
fn type_of<T>( _: &T ) -> &'static str {
    std::any::type_name::<T>()
}


fn main() {
    let site_name : String = 
            String::from("Software Nuggets 1");
            
    println!("Site Name: {} {}", 
            site_name,
            type_of(&site_name));


    let site_name2 = 
            "Software Nuggets 2".to_string();
            
    println!("Site Name: {} {}", 
            site_name2, 
            type_of(&site_name2));


    let site_name3 = "Software Nuggets 3";
    
    println!("Site Name: {} {}", 
            site_name3, 
            type_of(&site_name3));
            
            
/* In Rust, &str is not a datatype but rather 
 * a string slice type. It is commonly used to 
 * handle string data without taking ownership 
 * of the underlying memory.
 */
}
