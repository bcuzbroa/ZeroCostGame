// ITERATORS

pub fn scramble(numbers : Vec<u32>) -> impl Iterator<Item = u32>{
    //Create an iterator that:
    /*
    Filters even number
    Filters number n such as: n (mod 3) == 0
    Multiple the rest of numbers by 2
    return the iterator but don't collect it !
    hint : use into_iter

    eg : [1,2,3,4,5,6,7,8,9,10] -> [1,5,7] (into inter)
     */
    numbers
    .into_iter()
    .filter(|&x| x % 3 != 0) 
    .filter(|&x| x % 2 != 0)  
    .map(|x| x * 2)           
}

pub fn sumup(scrambled : impl Iterator<Item = u32>) -> u32{
    /*
    Todo: return the sum of the iterator returned by scramble
    */
    scrambled.sum()
}

pub fn collect_and_sort(scrambled : impl Iterator<Item = u32>) -> Vec<u32>{
    /* Todo: Collect the iterator and then
    sort it in the ascending order
    return it as a vector.
     */
    let mut v : Vec<u32> = scrambled.collect();
    v.sort();
    v
}

/*
Used to generate the challenge Verifier
fn main() {
    let numbers: Vec<u32> = (0..=255).collect();
    println!("{:?}", numbers);
    let scrambled = scramble(numbers.clone());
    
    let sum = sumup(scrambled);
    
    let collect = collect_and_sort(scramble(numbers.clone()));
    
    println!("Sum: {}", sum);
    println!("Sorted values: {:?}", collect);
}
*/