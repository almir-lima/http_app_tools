//-- ----------------------------------------------------------------
//-- Author : Almir Lima  / iSoft Sistemas                         --
//-- Date: 19 jan 2019                                             --
//-- Decode the request HTTP 1.1, and result a Struct Request      --
//-- Decodifica a requisição HTTP 1.1, e devolve a Struct Request  --
//-- ----------------------------------------------------------------

//!--------------------------------------------------------------------------------------------
//!----  Decode HTTP REQUEST and converte in Http_Request Struct
//!--------------------------------------------------------------------------------------------

use std::net::{TcpStream};
use std::io::{Read};

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
  
  pub fn new( mut stream: TcpStream ) -> Option<HttpRequest>{

     let mut buffer = [0u8; 3072 ]; // Create a vector/slice UTF8 with 3072 bytes *3MB

     stream.read( &mut buffer ).unwrap(); // Read the content of request and put in variable buffer
    
     let req_http = String::from_utf8( buffer.to_vec() ).unwrap();  // Convert vecto UTF8 to String

     let req_http = req_http.trim_matches( char::from(0) );
             
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
     let app = parts.next()?;    // Name of the Aplication
     let res = parts.next()?;    // Resource to consume
     let method = parts.next()?; // method require
      
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
