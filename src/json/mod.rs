use crate::json::array::JSonArray;
use crate::json::object::JSonObject;

pub mod lexer;
pub mod array;
pub mod object;
pub mod parser;

pub trait JSon<T>{
    fn get_number(&self, key:T)->Option<f32>;
    fn get_string(&self, key:T)->Option<&str>;
    fn get_boolean(&self, key:T)->Option<bool>;
    fn get_object(&self, key:T)->Option<&JSonObject>;
    fn get_array(&self, key:T)->Option<&JSonArray>;
    fn to_string(&self)->String;
}