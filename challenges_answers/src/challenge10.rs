/* /!\ HARD ONE /!\*/

fn select_best <'a, T, K, F>(
    items: &'a [T],
    key: F,
) -> Option<&'a T>
where 
    F : Fn(&T) -> K,
    K : Ord,
{
    // Cache in order to compute each key only once
    let mut iter = items.iter();
    let mut best = iter.next()?;
    let mut best_key = key(best);

    for t in iter {
        let k = key(t);
        if k > best_key{
            best = t;
            best_key = k
        }
    }

    Some(best)
}


fn select_best_of_each_window<'a, T, K, F>(
    items: &'a [T],
    key: F,
    window_size : usize,
) -> Vec<&'a T>
where 
    K : Ord,
    F : Fn(&T) -> K,
{
    if window_size == 0 || window_size > items.len()  {
        return Vec::new()
    }

    let mut best_of_each_window : Vec<&T> = Vec::new();
    
    for window in items.windows(window_size){
        best_of_each_window.push(select_best(window, &key).unwrap());
    }
    
    best_of_each_window
    
}