fn type_of<T>( _: &T ) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {

    let person = ("Nicole", 17, "Orlando");


    // tuples are zero-indexed
    // meaning the first element in the tuple 
    // has an index of 0
    
    println!("===== Person Details =====");
    println!("Name: {}",        person.0);
    println!("Age: {}",         person.1);
    println!("Location: {}",    person.2);
    
    println!("name = {}", type_of(&person.0));
    println!("age = {}",  type_of(&person.1));
    
    println!("\n\n-----------------\n\n");
    
    
    
    let student: (u32, &str, &str) = 
                            (25, "John", "Doe");

    println!("student details:");
    println!("Age: {}",             student.0);
    println!("First Name: {}",      student.1);
    println!("Last Name: {}",       student.2);
    
    
    println!("first name = {}", 
                         type_of(&student.1));
                        
                        
    println!("age = {}", type_of(&student.0));
}