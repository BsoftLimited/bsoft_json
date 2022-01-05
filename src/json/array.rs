use std::slice::Iter;
use crate::json::JSonObject;
use crate::json::JSon;
use crate::json::parser::JSonItem;

pub struct JSonArray{ data:Vec<JSonItem> }

#[allow(dead_code)]
impl JSonArray{
    pub fn new()->Self{
        JSonArray{ data:Vec::new() }
    }
    
    pub fn add(&mut self, item:JSonItem){
        self.data.push(item);
    }
    
    pub fn get(&self, index:usize)->Option<&JSonItem>{
        if self.data.len() > index{
            return Some(&self.data[index]);
        }
        return None;
    }
    
    pub fn remove(&mut self, index:usize){
        if self.data.len() > index{
            self.data.remove(index);
        }
    }

    pub fn size(&self)->usize{ self.data.len() }

    pub fn get_itr(&self)->Iter<JSonItem>{
        return self.data.iter();
    }
}

impl JSon<usize> for JSonArray{
    fn get_number(&self, index:usize) -> Option<f32> {
        match self.get(index){
            Some(item) =>{
                if let JSonItem::Number(value) = item {
                    return Some(*value);      
                }
            },
            _ =>()
        }
        return None;
    }
    fn get_string(&self, index:usize) -> std::option::Option<&str> { 
        match self.get(index){
            Some(item) =>{
                if let JSonItem::String(value) = item {
                    return Some(value);      
                }
            },
            _ =>()
        }
        return None;
     }
    fn get_boolean(&self, index:usize) -> std::option::Option<bool> { 
        match self.get(index){
            Some(item) =>{
                if let JSonItem::Boolean(value) = item {
                    return Some(*value);      
                }
            },
            _ =>()
        }
        return None;
    }
    
    fn get_object(&self, index:usize) -> Option<&JSonObject> { 
        match self.get(index){
            Some(item) =>{
                if let JSonItem::Object(value) = item {
                    return Some(&value);      
                }
            },
            _ =>()

        }
        return None;
    }
    
    fn get_array(&self, index:usize) -> Option<&JSonArray> {
        match self.get(index){
            Some(item) =>{
                if let JSonItem::Array(value) = item {
                    return Some(value);      
                }
            },
            _ =>()
        }
        return None;
    }
    
    fn to_string(&self) -> String {
        let mut builder = String::from("[");
		for i in 0..self.data.len(){
            match &self.data[i]{
                JSonItem::Boolean(value) => { builder.push_str(format!("{}", value).as_ref()); },
                JSonItem::Array(value) =>   { builder.push_str(format!("{}", value.to_string()).as_ref()); },
                JSonItem::Number(value) =>  { builder.push_str(format!("{}", value).as_ref()); },
                JSonItem::Object(value) =>  { builder.push_str(format!("{}", value.to_string()).as_ref()); },
                JSonItem::String(value) =>  { builder.push_str(format!("'{}'", value).as_ref()); },
                JSonItem::Error(_) =>{}
            }
            if i < self.data.len() - 1{ builder.push_str(", "); }
		}
        builder.push(']');
		return builder;
    }
}