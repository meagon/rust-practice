// RustyXML
// Copyright (c) 2013, 2014 Florian Zeitz
//
// This project is MIT licensed.
// Please see the COPYING file for more information.

extern crate xml;
use std::io::File;
use std::io::Reader;
use std::path::Path;

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

    let mut p = xml::Parser::new();
    let mut e = xml::ElementBuilder::new();

    let string = match rdr.read_to_string() {
        Ok(string) => string,
        Err(err) => {
            println!("Reading failed: {}", err);
            std::os::set_exit_status(1);
            return;
        }
    };

    p.feed_str(string.as_slice());
    for event in p {
        match event {
            Ok(event) => match e.push_event(event) {
                Ok(Some(e)) => {
                    // println!("{}", e.get_children("SCAN", None));
                    //println!("{}", e.show());
                    // println!("{}", e.get_children("SCAN", None)[0].get_children("SCANNER", None)[0].get_attribute("name", None).unwrap() );
                    println!("{}", e.get_children("SCAN",  None)[0]);
                    // println!("names = {}", e.get_children("SCAN", None)[0].name);

                    //if e.get_children("SCAN", None)[0]



                    //if e.get_children("SCAN", None)[0].name == "SCANNER".to_string() {
                    //    println!("{}", e.get_children("SCAN", None)[0].get_attribute("name",None).unwrap())
                    //}
                    for child in e.get_children("SCAN", None)[0].children.iter() {
                        match *child {
                            xml::ElementNode(ref elem) =>{
                                if elem.name == "SCANNER".to_string() {
                                    println!("scanner = {}, scan_result= {}", 
                                        e.get_children("SCAN", None)[0].get_children("SCANNER", None)[0].get_attribute("name",None).unwrap(),
                                        e.get_children("SCAN", None)[0].get_children("SCANNER", None)[0].get_children("NAME",None)[0].children[0]
                                    );
                                    println!("children name {}", &elem.name);
                                }
                            }

                            _ => {}
                        }
                    }


                    ////    println!("scanner : {}", i.get_children("SCANNER", None)[0].get_attribute("name", None).unwrap())
                    //      // if i &xml::XML
                    //            for sub_el_item in el.children.iter() {
                    //             let mut sub_sub_item =  format_args!( fmt::format, "{}",sub_el_item);
                    //              let mut sub_sub_el = xml::ElementBuilder::new();
                    //               let mut sub_sub_pp = xml::Parser::new();
                    //                sub_sub_pp.feed_str(sub_sub_item.as_slice());


                    //                 for sub_sub_event in sub_sub_pp {
                    //                  match sub_sub_event {
                    //                   sub_sub_el.push_event(sub_sub_event) {
                    //                    Ok(Some(sub_sub_el)) => {
                    //                     // println! ( "sub_sub_event ---- {}",sub_sub_el.name);
                    //                      if sub_sub_el.name == "table".to_string() {
                    //                       info!("{}", sub_sub_el)
                    //                        }
                    //                         }
                    //      
                          
                          //
                          //
                          //
                          //
                    //     if i.name == 
                     //     let mut  c= i .clone();                          
                          //println!("children  : {}",i);
                    //}
                }




                Ok(None) => (),
                Err(e) => println!("{}", e),
            },
            Err(e) => println!("Line: {} Column: {} Msg: {}", e.line, e.col, e.msg),
        }
        // println!("{}", event);
    }
}
