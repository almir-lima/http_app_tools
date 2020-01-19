
<<<<<<< HEAD
pub mod request;
pub mod response;
=======
pub struct Foo{}
>>>>>>> 53171bee7b5855216c9ef6fd1e32061bbb37268f

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
