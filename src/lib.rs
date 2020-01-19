//-- ----------------------------------------------------------------
//-- Author : Almir Lima  / iSoft Sistemas                         --
//-- Date: 19 jan 2019                                             --
//-- Decode the request HTTP 1.1, and result a Struct Request      --
//-- Decodifica a requisição HTTP 1.1, e devolve a Struct Request  --
//-- ----------------------------------------------------------------

///--------------------------------------------------------------------------------------------
///----  Decode HTTP REQUEST and converte in Http_Request Struct
///--------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct HttpRequest {
    pub verb: String,
    pub app: String,
    pub res: String,
    pub method: String,
    pub qry: Vec<String>,
    pub data: String,
}

impl HttpRequest {
  
  pub fn new( req_http: &str ) -> Option<HttpRequest>{
        
     let mut e_data = req_http.split("\r\n\r\n"); // Split Data and Head of Http request
     let head = e_data.next()?;
     let data = e_data.next()?;

     let mut parts = head.split(" "); // Split verb and uri 
     let verb = parts.next()?;
     let uri = parts.next()?;

     let mut parts = uri.split("?"); // Split query of uri
     let path = parts.next()?;
     let query = parts.next()?;

     let mut parts = path.split("/"); // split path of uri 
     let _ = parts.next()?;
     let app = parts.next()?; // Aplication
     let res = parts.next()?; // Resource
     let method = parts.next()?; // method 
      
     let qry = query.split("&").map( str::to_string ).collect(); // create a array of query
        
     // return Some if ok or None case have a error
     Some( 
          HttpRequest{
                        verb: verb.to_string(),
                        app: app.to_string(),
                        res: res.to_string(),
                        method: method.to_string(),
                        qry,
                        data: data.to_string(),
          }
     )
  }
}

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

/*
Request: POST /teste HTTP/1.0
Host: 127.0.0.1:1509
Keep-Alive: 300
Connection: keep-alive
User-Agent: Mozilla/4.0 (compatible; Synapse)
Content-Type: application/x-www-form-urlencoded/json
Content-Length: 24
*/

#[cfg(test)]
mod tests {

    use super::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }
}
