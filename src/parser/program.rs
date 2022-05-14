use std::collections::HashMap;

use super::grammar::Function;

pub struct Program {
    functions: HashMap<String, Function>,
}
