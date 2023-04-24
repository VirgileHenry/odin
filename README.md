# Odin

Created by Eclipse (Virgile HENRY)

#### Intro :

Lexer and Parser for Magic: the Gathering cards abilities. 

Inspired from Petr Hudeček's work : https://hudecekpetr.cz/a-formal-grammar-for-magic-the-gathering/

#### Usage :

The easiest way to use it is to use the `FromStr` implementation of the `AbilityTree` struct. This will call the lexer and parser, and the result, `AbilityTree` is a tree containing all the data of the parsed ability.

However, the input need to be slightly modified : 
- All instances of the card's name need to be replaced with "~'.
- The end of input symbol "$" need to be added at the end of the input. (this may be removed.)

For instance :
```
let text: &str = "Flying\nMentor (Whenever this creature attacks, put a +1/+1 counter on target attacking creature with lesser power.)\nWhen ~ dies, put a +1/+1 counter on each white creature you control.$";
let ab_tree = AbilityTree::from_str(text).unwrap();
println!("{ab_tree}");
```

#### State :

Currently, can only parse one card, light of the legion, because it was the first test card. 
Now the the structure is done, I will go and expand it.
