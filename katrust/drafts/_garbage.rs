





use std::str;
use std::collections::HashMap;
use std::{cmp::Eq, hash::Hash};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Rc, Weak};




struct Cacher<U, T> where T: FnMut(U) -> U{
    closure: T,
    map: HashMap<U, U>,
    result: Option<U>,
}

impl<U, T> Cacher<U, T> where T: FnMut(U) -> U, U: Eq + Hash + Display + Copy{
    fn new(_: U, closure: T) -> Cacher<U, T>{
        Cacher{
            closure,
            map: HashMap::new(),
            result: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.result{
            Some(v) => v,
            None => {
                let result = self.map.entry(arg).or_insert((self.closure)(arg));
                self.result = Some(*result);
                *result
            }
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let a_simple_var: u8 = 34;
	let callback = move |num: u32| -> u32 {
            a_simple_var = 56;
            println!("a simple var just moved here");
            println!("calculating slowly...");
            num+1 // we can add one to the num because this closure can mutate its environment vairable and it moves them to its scope!
        
      };
      
    let mut expensive_result = Cacher::new(34, callback);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", expensive_result.value(intensity)
            );
        }
    }
}


fn main(){

    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            println!("{},{}", x, y);
            if y == 3 {
                break 'outer;
            }
        }
    }


// 	::::::::::iterator for struct::::::::::
	struct Alternate {
	    state: i32,
	}

	impl Iterator for Alternate {
	    type Item = i32;

	    fn next(&mut self) -> Option<i32> {
		let val = self.state;
		self.state = self.state + 1;

		// if it's even, Some(i32), else None
		if val % 2 == 0 {
		    Some(val)
		} else {
		    None
		}
	    }
	}

	let mut iter = Alternate { state: 0 };

	// we can see our iterator going back and forth
	assert_eq!(iter.next(), Some(0));
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), None);



// 	::::::::::async future example::::::::::
	async fn blocks() {
	    let my_string = "foo".to_string();

	    let future_one = async {
		// ...
		println!("{}", my_string);
	    };

	    let future_two = async {
		// ...
		println!("{}", my_string);
	    };

	    // Run both futures to completion, printing "foo" twice:
	    let ((), ()) = futures::join!(future_one, future_two);
	}
    // =============================================================================================================================
        let first_name = "Pascal"; // str - &str is a reference to String some where in the heap
        let last_name = "Precht".to_string(); // turn to String
        let another_last_name = String::from("Precht");
        greet(first_name); // first_name is &str by default
        greet(&last_name); // last_name is passed by reference
        greet(&another_last_name); // another_last_name is passed by reference

            fn greet(name: &str) {
            println!("Hello, {}!", name);
            }

            
        let name = String::from("erfan"); // String
        let another_name = "another erfan"; // str
        let combined = name + &another_name;
        // name.push_str(&another_name); // name moved due to above operator
        println!("{}", combined);
        // println!("{}", name); // error - borrowed after move
        println!("{}", another_name);

        let sample_string = String::from("wildonion");
        let bytes = sample_string.bytes(); // turn a string into buffer (asccii)
        println!("[..] two first bytes of the string are : {}", &sample_string[0..2]); // byte indices
        println!("[..] the string bytes : {:?}", bytes);

        let text = "hello hello from wildonion here double again again wildonion";
        let mut map = HashMap::new();
        for word in text.split_whitespace(){
            let count = map.entry(word).or_insert(0); // return a mutable reference inserted or the old value
            *count += 1; // updating the old value by dereferencing it, cause count is a mutable reference of the value 
        }

        println!("{:?}", map);
    // =============================================================================================================================

        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;
        generate_workout(simulated_user_specified_value, simulated_random_number);



            //-- Rc<T> , RefCell<T> , Weak<T> example
            //-- using a graph and tree structure
            //---------------------------------------
            #[derive(Debug)]
            struct Node{
                value: i32,
                parent: RefCell<Weak<Node>>,
                children: RefCell<Vec<Rc<Node>>>,
            }



            let leaf = Rc::new(Node{
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });


            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf), //-- 1 , a reference to leaf Rc<Node>
                Rc::weak_count(&leaf), //-- 0 , because nothing is pointing to leaf thus no owner or reference is available for leaf.
            );

            
                // -------------------- NEW SCOPE ------------------------
                    {
                        let branch = Rc::new(Node {
                            value: 5,
                            parent: RefCell::new(Weak::new()),
                            children: RefCell::new(vec![Rc::clone(&leaf)]), //-- a tree could have strong Rc pointers from parent nodes to children
                        });

                        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //-- leaf is pointing to the branch parent thus the branch node has an owner or reference, Weak pointers from children back to their parents

                        println!(
                            "branch strong = {}, weak = {}",
                            Rc::strong_count(&branch), //-- 1 , a reference to baranch Rc<Node>
                            Rc::weak_count(&branch), //-- 1 , because leaf is pointing to the branch
                        );

                        println!(
                            "leaf strong = {}, weak = {}",
                            Rc::strong_count(&leaf), //-- 2 , because leaf is the branch children thus a reference is pointing to the leaf and it'll increament the strong_count
                            Rc::weak_count(&leaf), //-- 0
                        );
                    }
                // -------------------- END OF NEW SCOPE ------------------------

            
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf), //-- 1 , because we're out of the inner scope and we don't have the branch parent in here to reference to the leaf node child
                Rc::weak_count(&leaf), //-- 0
            );


            
            ///========================== GNN AND COLDRICE CODES ==========================


            mod PipeLine{
                struct Dataset{} //-- preprocessing steps for datasets in here like splitting dataset
                struct DataLoader{} //-- building dataloader object and filling its pipeline with preprcessed dataset
            
                impl Dataset{
                    fn new(){
                        //...
                    }
                }

                impl DataLoader{
                    fn new(){
                        //...
                    }
                }
            }
            
            mod Model{
                trait Train{async fn train();}
                trait Predict{async fn predict();}
                struct NN{}
                struct CNN{} //-- feeding dataloader object into the model
                struct Model{}
                impl Train for Model{
                    async fn train(){
                        //...
                    }
                }
                
                impl Predict for Model{
                    async fn predict(){
                        //...
                    }
                }
            }


            mod GNN{
                //-- bypass cyclic references using Weak<T>
                struct Neuron{
                    //-- neuron is a feature
                    //-- we want to have a reference to the data vector and also change its values inside of it
                    //-- variables are immutable by default so we're using RefCell in order to change the values
                    //-- of data vector at runtime and Rc in order to have multiple references to it from other Neurons.
                    data: RefCell<Rc<Vec<f64>>>,
                    parent: RefCell<Weak<Neuron>>,
                    children: RefCell<Vec<Rc<Neuron>>>,
                }
            }

            ///========================== END OF GNN AND COLDRICE CODES ==========================
            





}
