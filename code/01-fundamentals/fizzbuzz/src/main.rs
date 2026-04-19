// fn main() {
//     for i in 1..=100{
//         if i % 3 == 0 && i % 5 == 0 {
//             println!("FizzBuzz");
//         } else if i % 3 == 0 {
//             println!("Fizz");
//         } else if i % 5 == 0 {
//             println!("Buzz");
//         } else {
//             println!("{}",i);
//         }
//     }
// }
fn main() {                                                        
      for i in 1..=100 {                                               
          match (i % 3, i % 5) {
              (0, 0) => println!("FizzBuzz"),                          
              (0, _) => println!("Fizz"),                   
              (_, 0) => println!("Buzz"),                              
              (_, _) => println!("{i}"),
          }                                                            
      }                                                     
  } 
