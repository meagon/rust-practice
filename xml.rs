// xml to scan 
// Copyright (c) 2013, 2014 mtr
//
// This project is MIT licensed.
// Please see the COPYING file for more information.

extern crate xml;
use std::io::File;
use std::io::Reader;
use std::path::Path;

use std::collections::HashMap;
use std::collections;
use std::string::String;

//use std::collections::string;
// use std;


fn main()
{
    let args = std::os::args();
    let f = &match args.as_slice() {
        [_, ref path] => Path::new(path.as_slice()),
        [ref name, ..] => {
            println!("Usage: {} <file>", name);
            return;
        }
        _ => fail!("argv had length 0")
    };
    let mut rdr = match File::open(f) {
        Ok(file) => file,
        Err(err) => {
            println!("Couldn't open file: {}", err);
            std::os::set_exit_status(1);
            return;
        }
    };

    let string = match rdr.read_to_string() {
                Ok(string) => string,
                Err(err) => {
                    println!("Reading failed: {}", err);
                    std::os::set_exit_status(1);
                    return;
                }
    };

    let mut scan_result = parse_scan_to_hash(string);
    //println!("{}", scan_result);

}


fn  parse_scan_to_hash (string : String ) -> HashMap<String ,String> {

//fn  parse_scan_to_hash <K : &str,V: &str> (string : String ) -> std::collections::hashmap::HashMap <K,V> {
//fn parse_scan_to_hash  <K : collections::string::String  ,V : collections::string::String > (string :String ) -> HashMap <K,V> {
    // std::collections::hashmap::HashMap<String,String> {
//HashMap<K,V> {

    let mut string = string;
    let mut scan_result : HashMap <String, String> = HashMap::new();

    let mut p = xml::Parser::new();
    let mut e = xml::ElementBuilder::new();


    p.feed_str(string.as_slice());
    for event in p {
        match event {
            Ok(event) => match e.push_event(event) {
                Ok(Some(e)) => {
                    // println!("{}", e.get_children("SCAN", None));
                    // println!("{}", e.show());
                    // println!("{}", e.get_children("SCAN", None)[0].get_children("SCANNER", None)[0].get_attribute("name", None).unwrap() );
                    // println!("{}", e.get_children("SCAN",  None)[0]);
                    // println!("names = {}", e.get_children("SCAN", None)[0].name);

                    //if e.get_children("SCAN", None)[0]



                    //if e.get_children("SCAN", None)[0].name == "SCANNER".to_string() {
                    //    println!("{}", e.get_children("SCAN", None)[0].get_attribute("name",None).unwrap())
                    //}
                    let mut count = 0u;
                    for child in e.get_children("SCAN", None)[0].children.iter() {
                        match *child {
                            xml::ElementNode( ref elem) =>{
                                if elem.name == "SCANNER".to_string() {
                                    // println!("{}", elem.show());
                                    // println!("scanner = {}, scan_result= {}", 
                                    let  mut eng = elem.get_attribute("name", None).unwrap(); 
                                    //   e.get_children("SCAN", None)[0].get_children("SCANNER", None)[count].get_attribute("name",None).unwrap();

                                    let  mut eng_virname = elem.get_children("NAME",None)[0].children[0].clone(); 
                                    //   e.get_children("SCAN", None)[0].get_children("SCANNER", None)[count].get_children("NAME",None)[0].children[0];
                                    // let mut eeng = format!("{}", eng);
                                    // let mut eeng_virname = format!("{}", eng_virname);
                                    scan_result.insert( eng.to_string(), eng_virname.to_string() );
                                    //);
                                    count = count+1 ;
                                    //println!("children name {}", &elem.name);
                                }else if elem.name == "HASH".to_string(){
                                    println!("{}", elem);
                                }

                            }

                            _ => {}
                        }
                    }

                    // println!("{}", scan_result);
                    return scan_result;

                }

                Ok(None) => (),
                Err(e) => {
                    println!("{}", e);
                    return  scan_result;
                }
            },
            Err(e) => {
                println!("Line: {} Column: {} Msg: {}", e.line, e.col, e.msg);
                return scan_result;
            }
        }
        // println!("{}", event);
    }
     return scan_result;
}
