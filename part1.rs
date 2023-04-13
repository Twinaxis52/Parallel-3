//Problem 1: The Birthday Presents Party

#[derive(Clone)]
enum address {
    address(Box<myList>),
    Nil,
}
#[derive(Clone)]
struct myList {
    value: u32,
    next: address,
}

impl myList {
    fn append(&mut self, elem: u32) { //this inserts a node to our linked list
        match self.next {
            address::address(ref mut next_address) => {
                next_address.append(elem);
            }
            address::Nil => {
                let node = myList {
                    value: elem,
                    next: address::Nil,
                };
                self.next = address::address(Box::new(node))
            }
        }
    }

    fn delete(&mut self, elem: u32) { // this deletes a node from our linked list
        match self.next {
            address::address(ref mut next_address) => {
                if next_address.value == elem {
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } 
            }
        }
    }
//  fn list(&self) { //this will print our list but i commented it out cus i only needed it to check my lsit
//             println!("{}", self.value);
//             match self.next {
//                 address::address(ref next_address) => next_address.list(),
//                 address::Nil => {}
//             }
//     }
}

//need under dependencies of cargo.toml
//rand = "0.8.5"
use rand;
use rand::seq::SliceRandom;
use std::thread;
use std::sync::{Arc,Mutex};

fn main() {
    let presents  = 500000; //the amount of presents we will have
    let mut rng = rand::thread_rng();
    //make array size of 500000 all 0
    let mut arr = [0; 500];

    //insert in ascending order of array length
    for i in 0..arr.len(){
        arr[i] = i+1;
    }
    //shuffle the array
    //we have the unordered bag
    arr.shuffle(&mut rng);
    let mut handles = vec![]; 
    
    let mut head = myList {
            value: 0,
            next: address::Nil,
        };
    //first servant will organize presents
    let h = thread::spawn(move||{  //thread 2
        arr.sort();
    });
    handles.push(h);

    //hook presents to the chain in order
    for i in 0..arr.len(){ //this is thread 1 because main is a thread
         head.append(arr[i].try_into().unwrap());
    }

    //will write thank you card to everyone that gave a present
    let h = thread::spawn(move||{ //thread 3 

        for i in 0..presents{
            println!(" Thank you guest {}", i);
        }
    });
    handles.push(h);

    //will unhook the presents from linked list
    let h = thread::spawn(move||{ //thread 4

        for i in 0..arr.len(){
            head.delete(arr[i].try_into().unwrap());
        }
    });
    handles.push(h);

    for h in handles.into_iter(){ //join all the threads and waits for them to finish
        //println!("Got: {:?}", h);
        h.join().unwrap();
    }
}
