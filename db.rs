extern crate postgres;
extern crate time;
extern crate collections;

// extern crate avl;


use time::Timespec;

use std::collections::HashMap;

use postgres::{PostgresConnection, NoSsl};

struct Person {
                    md5          :   String ,
                    crc32        :   String ,
                    sha1        :   String ,
                    avpscan      :   String ,
                    mcascan      :   String ,
                    navscan      :   String ,
                    kavscan      :   String ,
                    ravscan      :   String ,
                    msescan      :   String ,
                    bdpscan      :   String ,
                    atvscan      :   String ,
                    nodscan      :   String ,
                    clmscan      :   String ,
                    sohscan      :   String ,
                    avlscan      :   String ,
                    batch_id     :   String ,
                    scan_cp      :   String
}


fn main(){
   println!("hello");
}

pub fn db_store_avml(scan_result_in :  HashMap<collections::string::String, collections::string::String>) {
    let conn = PostgresConnection::connect("postgres://pg@127.0.0.1/postgres",
                                           &NoSsl).unwrap();
   let mut scan_result = scan_result_in.clone();
    println!("{}", scan_result.clone());
    //let mut scan_result = HashMap::new();
    //scan_result.insert("MD5".to_string()     ,    "md5_values".to_string());
    //scan_result.insert("crc32".to_string()   ,  "crc32_values".to_string());
    //scan_result.insert("sha1".to_string()    , "sha1_values".to_string());
    //scan_result.insert("pscan".to_string() , "avp_values".to_string());
    //scan_result.insert("ascan".to_string(), "mca_values".to_string());

    //conn.execute("CREATE TABLE person (
    //                md5             VARCHAR ,
    //                crc32           VARCHAR ,
    //                sha1           VARCHAR ,
    //                pscan         VARCHAR ,
    //                ascan         VARCHAR ,
    //                hvscan         VARCHAR ,
    //                ivscan         VARCHAR ,
    //                jvscan         VARCHAR ,
    //                kescan         VARCHAR ,
    //                pscan         VARCHAR ,
    //                vscan         VARCHAR ,
    //                dscan         VARCHAR ,
    //                mscan         VARCHAR ,
    //                hscan         VARCHAR ,
    //                lscan         VARCHAR ,
    //                batch_id        VARCHAR ,
    //                scap_cp         VARCHAR 
    //              )", []).unwrap();


    let mut me = Person {
        md5:       scan_result.find_or_insert_with("MD5".to_string(),      |ref key|  "NULL".to_string()).to_string(),         // 1 
        crc32:     scan_result.find_or_insert_with("CRC32".to_string(),    |ref key|  "NULL".to_string()).to_string(),       // 2 
        sha1:      scan_result.find_or_insert_with("SHA1".to_string(),     |ref key|  "NULL".to_string()).to_string(),        // 3  
        avpscan :  scan_result.find_or_insert_with("PScan".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 4  
        mcascan :  scan_result.find_or_insert_with("AScan".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 5
        navscan :  scan_result.find_or_insert_with("hVScan".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 6 
        kavscan :  scan_result.find_or_insert_with("iVScan".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 7
        ravscan :  scan_result.find_or_insert_with("jVScan".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 7
        msescan :  scan_result.find_or_insert_with("kESCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 8 
        bdpscan :  scan_result.find_or_insert_with("lPSCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 9 
        atvscan :  scan_result.find_or_insert_with("mVSCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 10
        nodscan :  scan_result.find_or_insert_with("DSCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 11
        clmscan :  scan_result.find_or_insert_with("MSCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 12
        sohscan :  scan_result.find_or_insert_with("HSCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 13
        avlscan :  scan_result.find_or_insert_with("LSCAN".to_string(),  |ref key|  "NULL".to_string()).to_string(),  // 14
        batch_id : scan_result.find_or_insert_with("BATCHID".to_string(),  |ref key|  "NULL ".to_string()).to_string(), // 15
        scan_cp :  scan_result.find_or_insert_with("SCANCP".to_string(),   |ref key|  "NULL".to_string()).to_string()    // 16
    };
    match conn.execute("INSERT INTO person (
                        md5,     crc32,   sha1,    pscan, ascan, vscan, 
                        oscan, vscan, escan, pscan, vscan, dscan, 
                        mscan, hscan, lscan, seq_id, check_point )

                    VALUES ($1, $2, $3, $4 ,$5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17  )",
                  //  VALUES ($1, $2, $3, $4 ,$5  )",
                 [&me.md5,   &me.crc32,      &me.sha1,      &me.pscan,     &me.ascan,   &me.kvscan, 
                  &me.vscan, &me.hvscan,  &me.escan,   &me.pscan,     &me.ovscan,   &me.dscan, 
                  &me.mscan, &me.hscan,  &me.lscan,   &me.seq_id,  &me.scan_cp ]).unwrap(){
            Ok(a)  => {}, 
            Err(e) => {println!(" Unexpected result {}", e)},
}



}

