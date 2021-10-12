use std::collections::HashMap;

fn main() {
   let mut map = HashMap::new();
   let mut interger = vec![5,5,2,4,5];
   interger.sort();
   

   for i in interger.iter(){
       let count = map.entry(i).or_insert(0);
       *count += 1

   }

   let mut interger = interger.clone();
   interger.dedup();

   for el in 0..interger.len(){
      let mut val:u32 = match map.get(&interger[0]){
           Some(num) => *num,
           None => 0,
       };

       let mut max:u32 = match map.get(&interger[el]){
           Some(num) => *num,
           None => 0,
       }; 

       if max > val {
           val = max;
           return println!("The value is: {} appear {} times",interger[el],val)
       }

   }
     
}





