// Generics

use std::cmp::PartialOrd;



/*  Exercice I
Implement a largest fonction with a given list that
return the largest element of the lsit
*/
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T { 
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


//  Exercice 2 *
// Lot of point here but both exercices must be done !

struct Book<'a> {
    
    author : &'a str,
    title: &'a str,
    pages : i32,

}

/*
Implement the PartialEq trait for 
Book<'a> (only on the number of pages)
*/
impl<'a> PartialEq for Book<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.pages == other.pages
    }
}

/*
Implement the Partial Ord trait for 
Book<'a>
Compare the number of pages. If equals : compare the title
hint. Use 
Some (
    xxxxxx.then(yyyyyyy)
    )
*/
impl<'a> PartialOrd for Book<'a>{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.pages.cmp(&other.pages)
            .then(self.title.cmp(other.title))
        ) // If pages are equals, compare the title
    }
}