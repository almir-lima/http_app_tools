///--------------------------------------------------------------------------------------------
///----  HTTP REQUEST and converte in Http_Request Struct
///--------------------------------------------------------------------------------------------

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpResponse{
    pub head : HashMap<String, String>,
    body : String,
    pub status : i8,
}

impl HttpResponse{

  pub fn new() -> HttpResponse {
       
     HttpResponse {
                    head : HashMap::new(),
                    body : String::new(),
                    status : 0,
     }
  }
  
  // Write message-body HTTP	
  pub fn write_body( &mut self, _str : String ){
  	
  	&self.body.push_str( &_str );

  }
  
  // Get response to send	
  pub fn data( &mut self ) -> String {

    let mut _data = format!( "{}\r\n\r\n{}", &self.get_head() , &self.body );

  	println!(" este ? o data {}", &_data );
  	  	
  	format!( "{}", _data )
  	
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
  
}