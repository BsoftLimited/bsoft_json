#[derive(Debug)]
pub enum Token{ String(String), Number(f32), Boolean(bool), Character(char) }

#[derive(Debug)]
pub enum Output<T>{ Succeed(T), Error(String) }

pub struct Lexer{ index:usize, current:char, data: String }

impl Lexer{
    pub fn new(data:&str)->Self{
        let string = String::from(data);
        let init = string.chars().nth(0).unwrap();
        Lexer{ index:0, current:init, data: string, }
    }
    
    pub fn has_next(&mut self)->bool{
        while self.index < self.data.len(){
            self.current = self.data.chars().nth(self.index).unwrap();
            let passable = (self.current == ' ') || (self.current == '\n') || (self.current == '\t');
            if ! passable { return true; }
            self.index += 1;
        }
        return false;
    }
    
    fn pop(&mut self)->char{
        let init = self.data.chars().nth(self.index).unwrap();
        self.index += 1;
        return init;
    }
    
    pub fn get_next_token(&mut self)->Output<Token>{
		if self.current.is_alphabetic(){
			return self.get_boolean_token();
		}else if self.current.is_numeric(){
			return self.get_number_token();
		}else{
		    match self.current{
		        '{' => { return Output::Succeed(Token::Character(self.pop())); }
				'}' => { return Output::Succeed(Token::Character(self.pop())); }
				'[' => { return Output::Succeed(Token::Character(self.pop())); }
				']' => { return Output::Succeed(Token::Character(self.pop())); }
				'"' => { return self.get_string_token(); }
				'\''=> { return self.get_string_token(); }
				',' => { return Output::Succeed(Token::Character(self.pop())); }
				':' => { return Output::Succeed(Token::Character(self.pop())); }
				_   => { return Output::Error(format!("unexpected token {} encountered", self.pop())); }
			}
		}
	}
	
	fn get_string_token(&mut self)->Output<Token>{
		let open = self.pop();
		let mut builder = String::new();
		while self.index < self.data.len(){
		    let close = self.data.chars().nth(self.index).unwrap();
			if close == open{
                self.pop();
				return Output::Succeed(Token::String( builder));
			}else{
				builder.push(self.pop());
			}
		}
		return Output::Error(format!("Expecting a closing {}", if open == '\''{ "'"} else {"\""}));
	}
	
	fn get_number_token(&mut self)->Output<Token>{
		let mut builder = String::new();
		builder.push(self.pop());
		while self.has_next() && self.current.is_numeric() || self.current == '.'{
			builder.push(self.pop());
		}
		
		let init = builder.parse::<f32>();
		match init{
		    Ok(value) =>{ return Output::Succeed(Token::Number(value)); }
		    Err(_) =>{ return Output::Error(format!("Invalid number token: {}", builder)); }
		}
	}
		
	fn get_boolean_token(&mut self)->Output<Token>{
		let mut builder = String::new();
		builder.push(self.pop());
		while self.has_next() && self.current.is_alphabetic(){
			builder.push(self.pop());
		}
		
		if builder.eq("true") || builder.eq("false"){
			return Output::Succeed(Token::Boolean(builder.eq("true")));
		}
		return Output::Error(format!("Wrap token {} inside two \" or '", builder));
	}
}