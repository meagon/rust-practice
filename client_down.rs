#![crate_name = "client_down"]

extern crate debug;
extern crate http;
extern crate url;
use http::client::RequestWriter;
use http::method::Get;
use http::method::Head;
use http::headers::HeaderEnum;
use std::os;
use std::str;
use std::io::println;
use url::Url;

fn main() {
    format!("{}", Get);
    let args = os::args();
    match args.len() {
        0 => unreachable!(),
        2 => { 
           let mut aa =  get_url_content(args.get(1).as_slice());
           if aa == "error".to_string() {
               println!("eroror");
               return 
            }
             println!("{}", aa);
           // println! ("{}", str::from_utf8(aa.as_slice()).expect("Ub oh, response was't UTF-8"));
           //println!("{}", aa.to_string() );
        }

        _ => {
            println!("Usage: {} URL", args.get(0));
            return;
        },
    };
}

pub fn get_url_content(url: &str) -> String {
    let url = Url::parse(url).ok().expect("Invalid URL");
    let request: RequestWriter = RequestWriter::new(Head, url).unwrap();

    // for header in request.headers.iter() {
    //    println!(" - {}: {}", header.header_name(), header.header_value());
    // }

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };

     // for header in response.headers.iter() {
     //     println!(" - {}: {}", header.header_name(), header.header_value());
     // }
     let mut location  :String = "".to_string();     // String::new();
     //for i  in response.headers.iter().enumerate() {
     //   println!("{}", i);
     //}
     // println!("{}", response.headers.iter().enumerate());

     for header in response.headers.iter() {
            //   println! ( "{}" , header);
               
               if  header.header_name() == "Location".to_string(){
                   println!("{}: {}", header.header_name(), header.header_value());
                   location = header.header_value();
              
               } 
       
    }
    if location == "".to_string() {
        let ret=  "error".as_slice().clone().to_string();
        return ret;
    }
    let mut local = location.as_slice(); 
    let mut local = Url::parse(local).ok().expect("Invalid URL");
    let request_lo : RequestWriter = RequestWriter::new(Get, local).unwrap();


    let mut response_lo = match request_lo.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };  


    let body = match response_lo.read_to_end() {
        Ok(body) => body,
        Err(err) => fail!("Reading response failed: {}", err),
    };
     let ret =  str::from_utf8(body.clone().as_slice()).expect("Ub oh, response was't UTF-8").clone().to_string();
     return ret;
    // return body.as_slice(); // .to_string();
    //println(str::from_utf8(body.as_slice()).expect("Uh oh, response wasn't UTF-8"));
}


