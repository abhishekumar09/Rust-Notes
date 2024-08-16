//Rust :  Rust is a general-purpose, low-level, statically-typed programming language that focuses on safety,
//        performance, and concurrency




// fn main() {
//      let  x:i32= -5;
//      let y:i32  = 2000;
//      let z:f64  = 21490.45;

//      print!("x: {}",z);
// } 




//String -- Dynamic Length strings - Heap Allocated
//  &str -- Fixed Length Strings - Special memory
// fn main(){
//        let mut string_literal:String = String::from("Hi, code Eaters!!!!");
//        string_literal.push_str("what's up?");
//        println!("This is string literals {}",string_literal);
// }





// Tuple
// fn main(){
//     let emp_info:(&str,u8) = ("Sachin", 50);

//     let emp_name = emp_info.0;
//     let emp_age = emp_info.1;

// //destructing
//     let (employee_name, employee_age) = emp_info;


//     println!("Employee Name={}, Employee Age={}",emp_name,emp_age);
//     println!("Employee Name={}, Employee Age={}",emp_name,emp_age);

// }






//Funcions

// fn  main(){
//    print_value();
// }

// fn print_value(){
//     println!("Hello world");
// }

// fn main(){
//     print_value(5);
// }

// fn print_value(item:u8){
//     println!("{}",item);   
// }




// fn main(){
//     let num1:u8 = 10;
//     let num2:u8 = 20;
//     let result:u8 = add(num1,num2);
//     println!("THE SUM of num1 and num2 is {}",result);

// }


// fn add(item1:u8,item2:u8)->u8{     // write type also to show in ewhich data type u want the return value
//     return item1+item2;
// }





// scope : scope is something that the accessable of  variable we made or decalre  like GLOBE_CONST variable is accessible for throughout the codde and let inside_variable is accessble only on inside the curly braces
// const GLOBAL_CONST:u8 = 100;
// fn main(){
//     let outside_varable = 5;
//     {
//         let inside_variable = 10;
//         println!("Inside variable: {}", inside_variable);
//         println!("Outside variable: {}", outside_varable);
//     }  // inside_variable goes out of scop here

//     //println!("Inside vaiable: {}", inside_variable);   //Thhis line would result in a compiler


//     print!("Outside variable: {}", outside_varable);
//     print_value();


// }


// fn print_value(){
//     println!("{}", GLOBAL_CONST);
// }












// Now we understand about Ownership

// fn main(){
// let str1 = String::from("Hello");//str1 is thee owner
// let str2 = str1;//transfer of ownership.str2 is the new owner of Hello
// println!("str1={}",str1);
// println!("str2={}",str2);
// }

//eg-: for stack or fixed variable
// let a =5;
// let b = a;
// println!("a={}",a);
// println!("b={}",b);







//ownership and functions

//it's working on heap
// fn main(){
//   let x:String = String::from("Hello");               //x is the owner of Hello
//   process_string(x);                            //transfer of wonership                                      // x is working as pointer
//   println!("THE VALUE OF x in main() is {}", x);
// }

// fn process_string(item:String){                          // Hello-new owner is item
//     println!("The value of x in process_string() is {}",item);
// }



//stack  pe km kr rha hei                                     //yhan copy ho jata
// let x: i32 =5;                       // x ki memory hogi
// process_integer(x);
// println!("The value of x in main() is {}",x);
// }

// fn process_integer(item:i32){                            // item ki bhi ek memory hogi jo ki 5 pass kregi 
//     println!("The value of x in process_integer() is {}",item);








// ownership mein apne pdha ki rust jo hei iske through memory management kafi achi trah se kr rha hei  yhan garbage collection nhi hei na memory allocation or dealloction manually  kuch nhi hei sb rush behind the scene kr rha hei bs hmein uske rules ko follow krna hei aur yadd rkna hei 
//Question

// fn main() {
//     let s1:String = get_string();   //s1 is the owner of hello
//     println!("This is s1:{}",s1);

//     let s2:String= String::from("world"); // s2 is the owner of world
//     let s3:String= send_get_string(s2);   // Step1. transfer of ownership from s2  to received_string

//     println!("This is s3:{}",s3);   // s3 is the new owner of world
// }  

// fn get_string()->String{
//     let new_string:String = String::from("hello"); // new_string ower
//     return new_string;     // transferring the ownership
// }
// //received_string owner of world
// fn send_get_string(recieved_string:String)->String{
//     return recieved_string;          // transfer of ownership from received_string to s3
// }












// we learn about ki ownership hm ne phle transfer ki thi function ko  phir us se leli kyunki usko hum scope mein use kr skein   this thing e do with the help of (tuple)
// fn main(){
//     let s1:String = String::from("hello");    //s1 owner
//     let (s2,len) = calculate_length(s1);       //ownership transfer, new owner s2
//     println!("The length of {} is {}",s2,len);
// }

// fn calculate_length(s:String)->(String,usize){  // s will be the new owner
//     let length: usize = s.len();
//     return (s,length);    // return ownership transfer,5
// }






// with the help of (cloning) not need o give ownership to anyone  tuple here previous question

// fn main(){
//     let s1:String = String::from("hello");   // s1owner
//     let len  = calculate_length(s1.clone());
//     println!("The length of {} is {}",s1,len);
// }
// fn calculate_length(s:String)->usize{
//      let length:usize = s.len();
//      return length; //5
// }







// With the help of Reference(only use for read) -:  it means we can borrow the s1 for temporary purpose but cannot change or modify anything      example -:  like if u borrow a copy from ur friend for completing ur hoework then u return back to him after using but u frnd only give u a copy by a condition like u cannot write anything in their or something else same like that in refernce  
// // when u borrow rust will make sure about that u cannot perform mutable operation  
// fn main(){
//     let s1:String = String::from("Heloo");
//     let len:usize = calcuate_length(&s1);//borrow operation
//     println!("The length f {} is {}",s1,len);
// }

// fn calcuate_length(s2:&String)->usize{
//     return s2.len();
// }







// we want modification and also not giving ownership to anyone means by the help of refernce   means mein borrow bhi de rha hoon aur useee control bhi de rha hoon changes krne ke liye
// we did it by simply adding (mut) in refence

// fn main(){
//         let mut s1:String = String::from("Heloo");
//         let len:usize = calcuate_length(&mut s1);//borrow operation
//         println!("The length of {}" ,s1);
//     }
    
//     fn calcuate_length(s2:&mut String)->usize{
//         return s2.len();
//     }
    


    // one more solution and in this we also pushing variable 
    //  fn main(){
    //         let mut s1:String = String::from("Heloo");
    //         append_string(&mut s1);
    //         println!("The length of is {}",s1);
    //     }
        
    //     fn append_string(s3:&mut String){
    //         s3.push_str("World");
    //     }
        








// Rules  1. 2 mutable thing ek variable pr nhi hogi like 2 persons are writing in the same copy
// right solution first use first mut than go further to take another mut
// philosphy is multiple write operation nhi krna , multiple read lo  koi dikkat nhi is happens because of code mein error na aaya or data race condition na aaye 
// fn main(){
//     let mut s1:String = String::from("Hello");
//     let w1 = &mut s1;
//     w1.push_str(" World");
//     println!("w1={}",w1);


//     let w2 = &mut s1;
//     w2.push_str(" Code");
//     println!("w2={}",w2);
// }



// Multiple Read operation

// fn main(){
//     let mut s1:String = String::from("Hello");
//     let r1 = &s1;
//     let r2 = &s1;
//     println!("r2={}",r1);
// }


// fn main(){
//     let x =5;
//     let y = &x;     // y is refernce to the alue of x, value of x is 5
//     println!("y={}",y);
// }








// ARRAY
// fn main(){
//     // let arr1:[u8;5];   //array declaration 

//  let mut arr1;
//  arr1=[1,2,4,5,6,4];


//  println!("arr1[0]={}",arr1[0]);
//  println!("arr1={:?}",arr1);

//  arr1[2] = 30;
//  println!("arr1={:?}",arr1);

//  println!("Array length is {}",arr1.len());

// }




//Pass the Aray as an argument or a funtion 
// 1st way pass directly Array
// 2nd way pass the refernce of Array 



// Directly pass Array as an Argument or function 
//expensive way
// fn main(){
//     let arr: [&str; 3] = ["Hello", "World","Coders"];      
//     write_arr(arr);   //array directly pass  // means we do changes in array
//     println!("arr={:?}",arr);
// }

// fn write_arr(mut arr1:[&str; 3]){   // srr1 new copy of arr
//       arr1[0]= " Fellow";
//       println!("arr1={:?}",arr1);
// }           // alternative of this is refernce one is to next to this code







//refernce one
//inexpensive way
// fn main(){
//     let mut  arr: [&str; 3] = ["Hello", "World","Coders"];      
//     write_arr(&mut arr);  
//     println!("arr={:?}",arr);
// }

// fn write_arr(arr2:&mut [&str; 3]){
//     arr2[0]="Fellow";
//     println!("arr2={:?}",arr2);
// }

// we also do same as read_arr 






// VECTOR - Dynamic Array          // is heap allocated data hei      //yhan hemin ownershipk rules use hone wle hein 
// fn main(){
//   // let mut v:Vec<i32> = Vec::new(); //declaration
// //   let mut v = Vec::<i32>::new();

// //    v.push(1);
// //    v.push(2);
// //    v.push(3);

// let mut  v = vec![1,2,3,4,5,10];   // initialise
//       v.push(10);//1,2,3,4,5,10
//       v.pop(); //1,2,3,4,5

//    println!("Vector v={:?}",v);
// }





// Shadowing   -: allows developers to declare a new variable with the same name as a previously declared one.
// fn main(){
//     let x = 5;  //integer
//     println!("x={}",x);
//     let x = "Hello world";//string type
//     println!("x={}",x);
//     let x= x.len();//integer
//     println!("x={}",x);
// }








//For loop

// fn main(){
//     let arr = [1,2,3,4,5];

//     // println!("arr[0]={}",arr[0]);
//     // println!("arr[1]={}",arr[1]);
//     // println!("arr[2]={}",arr[2]);

//     for element in &arr{
//         println!("{}",element);
//     }
// }






//match
// fn main(){
//     let number = 5;        // it match with number is five


//     match number {
//         1=>println!("Number is one"),
//         2=>println!("Number is two"),
//         5=>println!("Number is five"),
//         _=>println!("Number is not recognized")
//     }

// }





// fn main(){
//     let number = 2;      


//     match number {
//         1  | 3=>println!("Number is one or three"),
//         2 | 4 =>println!("Number is two or four"),
//         5=>println!("Number is five"),
//         _=>println!("Number is not recognized")
//     }

// }




// for even number -:
// fn main(){
    

//     fn is_even(num:i8)-> bool{
//        return num%2 == 0;
//         // if num%2==0{
//         //     return true;
//         // }
//         // return false;
//     }

//    let number = 5;
//        match number {
//         x if is_even(x)=>println!("Even"),
//         x if !is_even(x)=>println!("Odd"),
//         _=>println!("Number is not recognized")
//     }
// }







// for taking input from user -:

// use std::io;          // by using liraray for taking inputs
// fn main(){
//        let mut input = String::new();
//        println!("Please input yor  name:");
//        io::stdin()
//        .read_line(&mut input)
//        .expect("Failed to read line ");
//        println!("User input:{}",input);
// }














// Project1 -  guess game 

//Game.rs

// use std::io;
// use rand::prelude::*;

// fn main() {
//     let guess_list: [&str; 3] = ["grapes", "banana", "orange"];
//     let mut rng = thread_rng();

//     let index = rng.gen_range(0..guess_list.len());
//     let random_fruit: &str = guess_list[index];
//     println!("A random fruit has been selected. Try to guess it!");

//     loop {
//         let mut input: String = String::new();
//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let fruit_selected = input.trim().to_lowercase();
//                 println!("Fruit Selected: {}", fruit_selected);

//                 if !guess_list.contains(&fruit_selected.as_str()) {
//                     println!("Fruit entered does not found!");
//                     continue;
//                 }

//                 if guess_checker(&fruit_selected, random_fruit) {
//                     println!("You are winner!");
//                     break;
//                 } else {
//                     println!("Retry!");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//             }
//         }
//     }
// }

// fn guess_checker(guessed_fruit: &str, random_selected: &str) -> bool {
//     guessed_fruit == random_selected
// }










// Project 2 - Tic Tac Toe









