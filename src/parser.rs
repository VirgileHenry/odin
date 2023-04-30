use std::collections::VecDeque;

use crate::tokens::{
    terminals::Terminal,
    tree::{
        AbilityTree,
        tree_token::TreeNode, rules::try_rule
    }
};

use self::error::OdinParserError;

pub fn parse(input: Vec<Terminal>) -> Result<AbilityTree, OdinParserError> {
    let input: VecDeque<TreeNode> = input.into_iter().map(|e| e.into()).collect();
    match try_parse(input, Vec::new())? {
        TreeNode::Text(result) => Ok(result),
        _ => Err(OdinParserError::InvalidRoot),
    }
}

fn try_parse(mut input: VecDeque<TreeNode>, mut stack: Vec<TreeNode>) -> Result<TreeNode, OdinParserError> {
    loop {
        // recursive call that allow backtracking if we find an error.
        // look all possible rules match on our stack
        // start by biggest slices as there are less chaces of failure on them
        if !stack.is_empty() {
            for i in 0..stack.len() {
                match try_rule(&stack[i..]) {
                    // some match can lead to multiple rules, so try them all !
                    Ok(new_nodes) => {
                        // try different rules
                        for new_node in new_nodes.into_iter() {
                            println!("Managed to parse rule {:?} from {:?}", new_node, &stack[i..]);
                            // recursive call to try
                            let new_input = input.clone();
                            let mut new_stack = stack.clone();
                            for _ in i..stack.len() {
                                new_stack.pop();
                            }
                            // if we manage to go all the way to the end, we parsed the whole thing.
                            // if we do, return the tree
                            // otherwise, try next input
                            new_stack.push(new_node);
                            match try_parse(new_input.clone(), new_stack.clone()) {
                                Ok(ab_tree) => return Ok(ab_tree),
                                Err(_) => {
                                    println!("failed at branch :\nstack: {:?}\ninput: {:?}\n", new_stack, new_input);
                                    /* this branch could not succeed, keep trying */},
                            }
                        }
                    },
                    Err(_) => {/* simply no rules */},
                }
            }
        }

        // put the next input token on the stack
        match input.pop_front() {
            Some(elem) => stack.push(elem),
            None => {
                // no more elements in the input: check the stack
                if stack.len() == 1 {
                    // stack of length one and no input : check for root
                    match stack.pop().unwrap() {
                        TreeNode::Text(result) => return Ok(TreeNode::Text(result)),
                        _ => {},
                    }
                }
                // and we haven't managed to fully parse : return error
                return Err(OdinParserError::Stuck{ stack, input })
            }
        }
    }
}


pub mod error;