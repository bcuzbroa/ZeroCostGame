/* ======================================================
CHALLENGE 9: SMART POINTER
=========================================================
*/

use std::rc::Rc;

#[derive(Debug)]
struct Folder {
    name: String,
    // We use Rc to allow multiple children to share ownership of the same parent
    parent: Option<Rc<Folder>>, 
}

/* 1. The "create_child" function
@args:    &str (name), &Rc<Folder> (parent)
@returns: Folder

Task: Create a new Folder instance. 
The 'parent' field should be a clone of the provided Rc pointer.
*/

// TODO: Write the create_child function here


/* 2. The "get_parent" function
@args:    &Folder
@returns: Option<Rc<Folder>>

Task: 
- If the folder's name is "/", return None.
- Otherwise, return a clone of the parent (Option<Rc<Folder>>).

Hint: Cloning an Rc only increments the reference count; 
it does not deep-copy the data.
*/

// TODO: Write the get_parent function here


/* 3. The "get_depth" function
@args:    &Folder
@returns: i32

Task: 
Calculate how many levels deep the folder is.
- If the folder has no parent (like "/"), return 0.

Hint: This is a recursive problem ! 
Since 'parent' is an Option, 'match' or 'if let' will be very useful.
*/

// TODO: Write the get_depth function here

// Main Section
fn main() {
   //Implement your tests here
}