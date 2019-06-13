extern crate reqwest;

// long way
// fn main() {

//     match reqwest::get("http://quotes.toscrape.com/") {
//         Ok(mut response) => {
//             // check if 200 OK
//             if response.status() == reqwest::StatusCode::Ok {
//                 match response.text() {
//                     Ok(text) => println!("response text: {}", text),
//                     Err(_) => println!("could not read response text!")
//                 }
//             } else {
//                 println!("Response was not 200 OK.");
//             }
//         }
//         Err(_) => println!("Could not make the request!")
//     }
// }

// short way
fn main() {
    let response_text = reqwest::get("http://quotes.toscrape.com/")
        .expect("couldnt make request")
        .text().expect("could not read response text");
    println!("{}",response_text)
}