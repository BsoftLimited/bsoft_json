use crate::json::object::JSonObject;
use crate::json::array::JSonArray;
use crate::json::lexer::{Token, Output, Lexer};

pub enum JSonItem{ Number(f32), String(String), Boolean(bool), Array(JSonArray), Object(JSonObject), Error(String) }
pub enum JSonResult{ Array(JSonArray), Object(JSonObject), Error(String) }

pub struct JSonParser{ lexer:Box<Lexer>}

impl JSonParser{
    pub fn from(data:&str)->JSonResult{
        let mut parser = JSonParser{ lexer: Box::new(Lexer::new(data)) };
        return parser.get_json();
    }
 
    fn check(&mut self, current:Token)->JSonItem{
        if let Token::Character(value) = current{
            match value{
                '[' => { return self.get_array(); },
                '{' => { return self.get_object(); },
                _ => ()
            }
        }else if let Token::String(string) = current{
            return JSonItem::String(string);
        }else if let Token::Number(value) = current{
            return JSonItem::Number(value);
        }else if let Token::Boolean(value) = current{
            return JSonItem::Boolean(value);
        }
        return JSonItem::Error(format!("Unexpected token found: {:?}", current));
    }
    
    fn check_next(&mut self)->JSonItem{
        if self.lexer.has_next(){
            let init = self.lexer.get_next_token();
            match init{
                Output::Succeed(token)=>{
                    return self.check(token);
                },
                Output::Error(error) =>{ return JSonItem::Error(error); }
            }
        }
        return JSonItem::Error(format!("unexpected end of json string"));
    }
    
    fn check_for_colon(&mut self)->JSonItem{
        if self.lexer.has_next(){
            let init = self.lexer.get_next_token();
            match init{
                Output::Succeed(token)=>{
                    if let Token::Character(value) = token{
                        if value == ':'{ return self.check_next(); }
                    }
                    return JSonItem::Error(format!("expecting a semi colon"));
                },
                Output::Error(error) =>{ return JSonItem::Error(error); }
            }
        }
        return JSonItem::Error(format!("unexpected end of json string"));
    }
    
    fn get_json(&mut self)->JSonResult{
        let init = self.check_next();
        if let JSonItem::Array(array) = init{
            return JSonResult::Array(array);
        }else if let JSonItem::Object(object) = init{
            return JSonResult::Object(object);
        }else if let JSonItem::Error(error) = init{
            return JSonResult::Error(error);
        }
        return JSonResult::Error(format!("invalid json data"));
    }

    fn get_object(&mut self)->JSonItem{
        let mut init_object = JSonObject::new();
        while self.lexer.has_next(){
            let init = self.lexer.get_next_token();
            match init{
                Output::Succeed(token) => {
                    if let Token::Character(value) = token{
                        if value == '}'{
                            return JSonItem::Object(init_object); 
                        }else if value == ',' && init_object.size() > 0 {
                            if self.lexer.has_next(){
                                let sub = self.lexer.get_next_token();
                                match sub{
                                    Output::Succeed(sub_token) =>{
                                        if let Token::String(value) = sub_token{
                                            let next = self.check_for_colon();
                                            if matches!(next, JSonItem::Error(_)){
                                                return next;
                                            }
                                            init_object.add(value.as_ref(), next);
                                        }else{
                                            return JSonItem::Error(format!("expecting a string token after the ','"));
                                        }
                                    },
                                    Output::Error(error) =>{ return JSonItem::Error(error);}
                                }
                            }else{
                                return JSonItem::Error(format!("unexpected end of string after {:?}", token));
                            }
                        }else{
                            return JSonItem::Error(format!("encountered an unexpected token: {:?}", token));
                        }
                    }else if init_object.size() == 0 &&  matches!(token, Token::String(_)){
                        if let Token::String(string) = token{
                            let next = self.check_for_colon();
                            if matches!(next, JSonItem::Error(_)){
                                return next;
                            }
                            init_object.add(string.as_ref(), next);
                        }
                    }
                },
                Output::Error(error) =>{ return JSonItem::Error(error); }
            }
        }
        return JSonItem::Error(format!("Unexpected end of object definition, expecting '}}'"));
    }
    
    fn get_array(&mut self)->JSonItem{
        let mut init_array = JSonArray::new();
        while self.lexer.has_next(){
            let init = self.lexer.get_next_token();
            match init{
                Output::Succeed(token) =>{
                    if let Token::Character(value) = token{
                        match value{
                            ']' => { return JSonItem::Array(init_array); },
                            ',' => {
                                let next = self.check_next();
                                if matches!(next, JSonItem::Error(_)){
                                    return next;
                                }
                                init_array.add(next);
                            },
                            _ => ()
                        }
                    }else if init_array.size() == 0{
                        let current = self.check(token);
                        if matches!(current, JSonItem::Error(_)) {
                            return current;
                        }
                        init_array.add(current);
                    }else{
                        return JSonItem::Error(format!("unexpected end of object definition, expecting ]"));
                    }
                },
                Output::Error(error) => { return JSonItem::Error(error); }
            }
        }
        return JSonItem::Error(format!("Unexpected end of array definition, expecting ]"));
    }
}
