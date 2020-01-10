use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::net::TcpStream;
use std::collections::HashMap;

//-- ----------------------
//-- Protocol HTTP 1.1   --
//-- Protocolo HTTP 1.1  --
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


#[derive(Debug)]
pub struct CouchDB {
    host: String,
    db: String,
    pub connected : bool,
    pub error : bool,
    pub msg : String,
} 


impl CouchDB {

    // Initialize the struct
	pub fn connect( _host : &str, _db : &str ) -> CouchDB {
	
	    CouchDB{
	             host : _host.to_string(),
		         db : _db.to_string(),
		         connected : false,
		         error : false,
		         msg : String::new(),
	    }
	}
}


fn main() {

    if let Err(err) = connect() {
        println!("err = {}", err);
    }

}

fn connect() -> Result<()> {

    let conn = CouchDB::connect("127.0.0.1:5984", "lumiari");

    let mut stream = TcpStream::connect("127.0.0.1:5984")?;

    let mut http = Http::new();
    
    http.head.insert( "GET /lumiari HTTP/1.1".to_string(), " ".to_string());
    http.head.insert( "Host".to_string(), "127.0.0.1:1509".to_string());
    http.head.insert( "Connection".to_string(), "close".to_string()); 
    http.head.insert( "Content-Type".to_string(), "application/json; charset=utf-8".to_string());
    http.head.insert( "Content-Length".to_string(),"0".to_string());

/*    
    let mut data = String::new();
    
    data.push_str(
    r#"{"_id": "07598",
                      "id": 0,                              
	                  "inclusao" : "2019-09-20T13:23:00Z",
	                  "nome" : "Almir Lima",
	                  "cpf" : "029.595.629-17",
	                  "rg" : "7882470-0",
	                  "dtnasc" : "26/07/1981",
	                  "ender":{ "cep" : "85.907-000",
	                            "lograd" : "Rua Bruno Dante Palma",
	                            "numero" : 1535,
	                            "bairro" : "Santa Clara 4",
	                            "cidade" : "Toledo",
	                            "uf" : "PR",
	                            "fone1" : "(45) 9 9960-0519",
	                            "fone2" : "(45) 3277-5307",
	                            "fone3" : ""
	                           }, 
	                  "res_ficha" : 0,
	                  "res_nome" : "",
	                  "res_cpf" : "",
	                  "res_rg" : "",
	                  "res_dtnasc" : "",
	                  "ind_nome" : "",
	                  "ind_ficha" : 0,
	                  "obs" : "Primeira consulta",
	                  "status" : "A",
	                  "historico" : ["|01/08/2019|Primeira Consulta","|17/09/2019|Segunda Consulta"]
                   }"#
    );
    
    
  
    let result = format!( "{}",  &data.len() );
    
    println!("Tamanho {}", result );

    request_data.push_str( &data );

    let request_data = request_data.replace( "$$", &result );
    
    
    println!("request_data = {:?}", request_data );

    let request = stream.write_all(request_data.as_bytes())?;
  //  println!("request = {:?}", request);

    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf)?;
   
    println!("result = {}", result);
    println!("buf = {}", buf);

 */
    Ok(())
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
