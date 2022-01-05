use std::collections::hash_map::Iter;
use crate::json::JSonArray;
use crate::json::JSon;
use crate::json::parser::JSonItem;
use std::collections::HashMap;

pub struct JSonObject{ data:HashMap<String, JSonItem> }

#[allow(dead_code)]
impl JSonObject{
    pub fn new()->Self{
        JSonObject{ data:HashMap::new() }
    }
    
    pub fn add(&mut self, key:&str, item:JSonItem){
        self.data.insert(String::from(key), item);
    }
    
    pub fn get(&self, key:&str)->Option<&JSonItem>{
        let init = &self.data.get(&String::from(key));
        if let Option::Some(item) = init {
            return Some(&item);
        }
        return None;
    }
    
    pub fn remove(&mut self, key:&str){
        self.data.remove(&String::from(key));
    }

    pub fn size(&self)->usize{ self.data.len() }

    pub fn contains(&self, key:&str)->bool{ self.data.contains_key(key) }

    pub fn get_itr(&self)->Iter<String, JSonItem>{ self.data.iter() }
}

impl JSon<&str> for JSonObject{
    fn get_number(&self, key: &str) -> Option<f32> {
        match self.get(key){
            Some(item) =>{
                if let JSonItem::Number(value) = item {
                    return Some(*value);      
                }
            },
            _ =>()
        }
        return None;
    }
    fn get_string(&self, key:&str) -> std::option::Option<&str> { 
        match self.get(key){
            Some(item) =>{
                if let JSonItem::String(value) = item {
                    return Some(value);      
                }
            },
            _ =>()
        }
        return None;
     }
    fn get_boolean(&self, key:&str) -> std::option::Option<bool> { 
        match self.get(key){
            Some(item) =>{
                if let JSonItem::Boolean(value) = item {
                    return Some(*value);      
                }
            },
            _ =>()
        }
        return None;
    }
    
    fn get_object(&self, key:&str) -> Option<&JSonObject> { 
        match self.get(key){
            Some(item) =>{
                if let JSonItem::Object(value) = item {
                    return Some(&value);      
                }
            },
            _ =>()
        }
        return None;
    }
    
    fn get_array(&self, key:&str) -> Option<&JSonArray> {
        match self.get(key){
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
        let mut builder = String::from("{");
        let mut i = 0;
        for (key, item) in self.data.iter(){
            builder.push_str(format!("'{}' : ", key).as_ref());
            match item{
                JSonItem::Boolean(value) => { builder.push_str(format!("{}", value).as_ref()); },
                JSonItem::Array(value) =>   { builder.push_str(format!("{}", value.to_string()).as_ref()); },
                JSonItem::Number(value) =>  { builder.push_str(format!("{}", value).as_ref()); },
                JSonItem::Object(value) =>  { builder.push_str(format!("{}", value.to_string()).as_ref()); },
                JSonItem::String(value) =>  { builder.push_str(format!("'{}'", value).as_ref()); },
                JSonItem::Error(_) =>{}
            }
            if i < self.data.len() - 1{ builder.push_str(", "); i+= 1;}
        }
        builder.push('}');
		return builder;
    }
}