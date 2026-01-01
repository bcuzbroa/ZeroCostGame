//BORROWING

//returns low caps trimed string
fn coktail(s : String) -> String{
    low_caps(space(&s))
}


/*
argument : a string s
return : s without trailing white space
eg: "       test     " -> "test"
hint : a string method exists to do so
but you can try to implement it yourself
*/
fn space(s : &str) -> &str {
    s.trim()
}

/*
argument : a string s
return : s to lowercase
hint : a string method exists to do so
but you can try to implement it yourself (Good luck with that lol)
*/
fn low_caps(s : &str) -> String{
    //TODO
    s.to_lowercase()
}

