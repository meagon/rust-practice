rustc  db.rs  --crate-type lib    --extern postgres=../../pg/rust-postgres/target/libpostgres-0089d21a7aa043fc.rlib -L ../../pg/rust-postgres/target/deps/ 

rustc avl.rs  -L ../../pg/rust-postgres/target/deps/ \
     --extern xml=./libxml-cfc1aed2a0055651.rlib   \
     --extern postgres=../../pg/rust-postgres/target/libpostgres-0089d21a7aa043fc.rlib \
     --extern db=./libdb.rlib -L ./ \
     --extern client_down=./libclient_down.rlib
 
#rustc client_down.rs -L /root/dev/lang/rust/chris-morgan/target/deps -L /root/dev/lang/rust/chris-morgan/target/native/ --extern url=/root/dev/lang/rust/chris-morgan/target/deps/liburl-921578b148f50e06.rlib --extern openssl=/root/dev/lang/rust/chris-morgan/target/deps/libopenssl-fbe75530f7eda428.rlib  --extern http=/root/dev/lang/rust/chris-morgan/target/libhttp-7ef257768eddfad7.rlib

