struct Employee {
    first_name: String,
    last_name:  String,
    salary:     f32,
    active:     bool
}

fn get_record() -> Employee {

    let emp = Employee {
        first_name: String::from("Fred"),
        last_name: String::from("Sanford"),
        salary: 45000.00,
        active: true,
    };

    emp;
}


fn main() {
    
    let mut emp = get_record();
    
    if emp.active == true {
    
        println!("{} {} annual salary is {}",
            emp.first_name,
            emp.last_name,
            emp.salary);
    
            
        emp.salary = emp.salary * 1.06;
    
        println!("after payraise: {0}", 
                              emp.salary);
        
    } else {
        
        println!("{} {} is not active",
            emp.first_name, 
            emp.last_name);
    }
    
}