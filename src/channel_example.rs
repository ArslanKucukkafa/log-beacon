// // use std::sync::mpsc;
//
// fn deneme() {
//     let (tx, rx) = mpsc::channel();
//
//     std::thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }