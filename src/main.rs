use num::traits::{CheckedSub};


fn main() {
     let mut test_vec:Vec<interger> = Vec::new();
     for i in 10..20 {
         test_vec.push(i as interger)
     }
     let ans = check_sum(9, test_vec);
     println!("{:?}",ans)
 }

 type interger = u64;

 fn check_sum(target:interger, list:Vec<interger>) -> Vec<(interger,interger)> {

    let mut result_vec:Vec<(interger,interger)>= Vec::new();

     for (index, value) in list.iter().enumerate(){
         let mut return_values = (index as interger, *value);

         let mut remainder = target.checked_sub(return_values.1).unwrap();

         for (index_2 , value_2) in list.iter().enumerate(){
            

            if remainder == *value_2 && return_values.1 != *value_2 {

                 let mut result_tuple = (return_values.1, remainder);
                 
                 result_vec.push(result_tuple);
             }
         }
         
     }
     result_vec
 }

#[cfg(test)]
 assert_eq!(check_sum(5, vec![1,2,3,4,5]), vec![(1,4),(2,3),(3,2),(4,1)]);
