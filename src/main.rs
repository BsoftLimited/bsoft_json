mod json;

use crate::json::JSon;
use crate::json::parser::{ JSonParser, JSonResult};

fn main(){
    let json = JSonParser::from("{ 'name' : 'bobby', 'email' : 'bsoftlimited@gmail.com', 'ismarried' : true, 'age' : 12 , 'foods' : ['rice', 'Beans', 'Potato Porrage']}");

    if let JSonResult::Object(object) = json {
        println!("{}", object.get_string("name").unwrap());   
        println!("{}", object.get_string("email").unwrap());
        println!("{}", object.get_boolean("ismarried").unwrap());
        println!("{}", object.get_number("age").unwrap());
        let foods = object.get_array("foods").unwrap();
        for i in 0..foods.size(){
            print!("{} ", foods.get_string(i).unwrap());
        }

        println!("\n{}", object.to_string());
    }
}