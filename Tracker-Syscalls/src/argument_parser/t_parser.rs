use crate::argument::t_argument::TArgument;
// ======================================================================================
// Es una interfaz que se define un metodo el cual es parse
// ======================================================================================
pub trait TParser {
    fn parse(&self, args: Vec<String>) -> Result<Vec::<Box<dyn TArgument>>, String>;
}
