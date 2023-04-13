//Problem 2 atmospheric temperature reading module
use std::thread;
use std::sync::{Arc,Mutex};
use std::time::Duration;
use rand::{self, thread_rng, Rng};
/*Part 1 of the assignment Kariel Sanchez Ruiz */
fn main() {
    let sensors = 8; //sensors to 8
    let mut handles = vec![]; //mutable vector store all of our threads
    let count = Arc::new(Mutex::new(1)); //initialize count to 1
    let maximo = Arc::new(Mutex::new(0)); //maximum difference to zero
    let temp_list: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![])); //makes list for us
    for index in 0..sensors{ //this for loop decides how many threads for example 0..8 gives us 8 threads
        let count = Arc::clone(&count); //reference to count to use in threads
        let temp_list = Arc::clone(&temp_list);
        let maximo = Arc::clone(&maximo);
        

        let h = thread::spawn(move||{ 
            let mut rng = thread_rng(); //allows us to use rng
            let mut count = count.lock().unwrap(); //lock variable to use between threads
            let mut maximo = maximo.lock().unwrap(); 
            let mut temp_list = temp_list.lock().unwrap(); 
            while *count < 61{ //this will represent our hour
                let random_number = rng.gen_range(-100..70); //creates random number from that range
                temp_list.push(random_number);
                temp_list.sort();
                if *count % 10 == 0{ //this if loop will get the difference every 10 minutes to save the max difference in the 10 minute interval as count represents a minute
                    let mut min = temp_list[0];
                    let mut max = 0;
                    for i in 0..temp_list.len(){
                        if temp_list[i] < min{
                            min = temp_list[i];
                        }
                        else if temp_list[i] - min > max{
                            max = temp_list[i] - min;
                        }
                    }
                    if *maximo < max && *count < 60{
                        *maximo = max;
                    }
                }
                if *count == 60{
                    let top_5 = temp_list.as_slice()[temp_list.len()-5..temp_list.len()].to_vec();
                    println!("top 5 temperatures {:?}", top_5); // prints top 5 tempertures in the hour report
                    let bot_5 = temp_list.as_slice()[0..5].to_vec();
                    println!("bottom 5 temperatures {:?}", bot_5);// prints bot 5 tempertures in the hour report
    
                }
                thread::sleep(Duration::from_millis(1)); //simulates one minute 
                *count = *count + 1; // increase count everytime we go to sleep foor a minute because at 60 minutes our while loop gives the one hour report
                
            }

        });
        handles.push(h);
    }
    for h in handles.into_iter(){ //join all the threads before main thread to let them run before the main thread
        h.join().unwrap();
    }

    println!("max difference in an hour in a 10 minute interval: {}", *maximo.lock().unwrap()); 
    println!("temperatures: {:?}", *temp_list.lock().unwrap()); 

}
