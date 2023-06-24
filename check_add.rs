/*
  The add function returns the datatype: Result 
  where 
    the successful result is of type i8 and the error message, 
    an error result is a string slice (&'static str) that has a static lifetime.
    
    The static lifetime ensures that the error message remains valid 
    for the entire duration of the program.
*/

fn add(x: i8, y: i8) -> Result<i8, &'static str> {

    let result = x.checked_add(y);
    
    /*The match statement examines the value of result. 
     *If it contains a valid sum it returns Ok(sum)
     *If exceeds the maximum value of i8 and results in overflow
     *if the addition exceeds the maximum value of i8 and results in overflow, 
     *represented by None, it returns 
     *Err("Addition result exceeds maximum i8 (-128 to 127)") 
     */
    
    match result {
        Some(sum) => Ok(sum),
        None => Err("Addition result exceeds maximum i8 (-128 to 127)")
    }
    
}

fn main() {

    let n1 : i8 = 60;
    let n2 : i8 = 70;
    
    match  add(n1,n2) {
        Ok(sum) => println!("{} + {} = {}",n1,n2,sum),
        Err(error) => println!("Error: {}",error),
    }
}