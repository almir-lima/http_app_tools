use std::collections::HashMap;

//-- ----------------------
//-- Protocol HTTP 1.1   --
//-- ----------------------

#[derive(Debug, Clone)]
pub struct Http{
  
  pub head : HashMap<String, String>,
  body : String,
  pub status : i32,
}

impl Http{

  pub fn new() -> Http {
       
    Http {
           head : HashMap::new(),
  	       body : String::new(),
  	       status : 0,
    }
  }
  
  // convert head hashmap to string
  fn get_head(&mut self) -> String {
  	
    let mut temp = String::new();

    for (key, value) in &self.head {
        
        let sp = if value.len() == 0 {""} else {":"};  
        temp = format!("{}{}{}\r\n", key.as_str(), sp, value.as_str() );
    }
    
    temp  
  }
  
  // Write message-body HTTP	
  pub fn write_body( &mut self, _str : String ){
  	
  	&self.body.push_str( &_str );

  }
  
  // Get complete protocol from response	
  pub fn data( &mut self ) -> String {
  	
    let mut sucess = String::new();
    let mut res_code = String::new();

    match self.status {
    	200 => {
    	         sucess = "true".to_string();
    	         res_code = "200 OK".to_string();
    	       },
    	         
          _ => {
                 sucess = "false".to_string();
                 res_code = "200 OK".to_string();
               },
    }

    let mut _data = format!( "{}\r\n\r\n{}", &self.get_head() , &self.body );

  	println!(" este ? o data {}", &_data );
  	  	
  	format!( "{}", _data )
  	
  }
  
}

/*
Request: POST /teste HTTP/1.0
Host: 127.0.0.1:1509
Keep-Alive: 300
Connection: keep-alive
User-Agent: Mozilla/4.0 (compatible; Synapse)
Content-Type: application/x-www-form-urlencoded/json
Content-Length: 24
*/
