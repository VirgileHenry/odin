use std::fmt;

pub trait AbilityDisplay {
    /// Display mtg abilities in a tree-like manner.
    /// The padding represent the spacing to put, with a bar (│) for true and space for false.
    /// The is last tells wether this elements was the last.
    fn display(&self, f: &mut fmt::Formatter<'_>, padding: &mut Vec<bool>) -> fmt::Result;
}

/// This macro allow easy ability display for multiple elements that need to be displayed as multiple branches of a single root.
/// The elements of the vec need to implement AbilityDisplay.
/// we simply need to call the macro with formatter, padding of the root and the elements. 
#[macro_export]
macro_rules! ability_display_elems {
    ($f:expr; $padding:expr; $last:expr) => {
        {
            for is_bar in $padding.iter() {
                if *is_bar { write!($f, "│   ")?; }
                else { write!($f, "    ")?; }
            }
            write!($f, "└─ ")?;
            $padding.push(false);
            $last.display($f, $padding)?;
            $padding.pop();
        }
    };
    ($f:expr; $padding:expr; $first:expr, $($rest:expr),*) => {
        {   
            for is_bar in $padding.iter() {
                if *is_bar { write!($f, "│   ")?; }
                else { write!($f, "    ")?; }
            }
            write!($f, "├─ ")?;
            $padding.push(true);
            $first.display($f, $padding)?;
            write!($f, "\n")?;
            $padding.pop();
            ability_display_elems!($f; $padding; $($rest),*);
        }
    };
}

/// A macro to ease the display of vecs. Each elements of the vec will be displayed in it's own branch.
/// The elements of the vec need to implement AbilityDisplay.
/// If the vec is empty, this will display a single branch saying "(Empty)".
#[macro_export]
macro_rules! ability_display_vec {
    ($f:expr; $padding:expr; $vec:expr) => {
        {
            if $vec.is_empty() {
                for is_bar in $padding.iter() {
                    if *is_bar { write!($f, "│   ")?; }
                    else { write!($f, "    ")?; }
                }
                write!($f, "└─ (Empty)")?;
            }
            else {
                for i in 0..$vec.len()-1 {
                    for is_bar in $padding.iter() {
                        if *is_bar { write!($f, "│   ")?; }
                        else { write!($f, "    ")?; }
                    }
                    write!($f, "├─ ")?;
                    $padding.push(true);
                    $vec.get(i).unwrap().display($f, $padding)?;
                    write!($f, "\n")?;
                    $padding.pop();
                }
                for is_bar in $padding.iter() {
                    if *is_bar { write!($f, "│   ")?; }
                    else { write!($f, "    ")?; }
                }
                write!($f, "└─ ")?;
                $padding.push(false);
                $vec.get($vec.len()-1).unwrap().display($f, $padding)?;
                $padding.pop();
            }
        }
    };
}