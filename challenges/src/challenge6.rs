//Life TIME

/* 
TODO : return the longest of the two given string slices in argument
What do you think about the link between the return type and the argument types ?
*/
fn longest<'a>(x : &'a str, y :&'a str) -> &'a str{
    if x.len()> y.len(){
        return x;
    }
    y
}



