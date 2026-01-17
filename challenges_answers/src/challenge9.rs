use std::{num::NonZero, rc::Rc};

#[derive(Debug)]
struct Folder {
    name: String,
    // We're using Rc to share the parent to multiple childs
    parent: Option<Rc<Folder>>, 
}


fn create_child(name: &str, parent: &Rc<Folder>) -> Folder {
    Folder { 
        name: name.to_string(),
        parent: Some(Rc::clone(parent)) 
    }
}

/*
Write a function name `get_parent`
args : Folder
returns : Option<Folder>

The function returns the parent folder unless
the current folder is '/' in this case it returns None
*/
fn get_parent(f : &Folder) -> Option<Rc<Folder>> {
    if f.name == "/" {
        None
    } else {
        f.parent.clone()
    }
}

/*
Write a function get_depth
that returns the depth of the given argument folder
args : Folder
returns : i32
if the folder is "/" return 0.

*/
fn get_depth(f : &Folder) -> i32{
    match &f.parent{
        None => 0,
        Some(p) => 1 + get_depth(p),
    }
}