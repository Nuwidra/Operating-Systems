use crate::argument::program_argument::ProgramArgument as ProgramArgument;
use crate::argument::tracer_option_argument::TracerOptionArgument as TracerOptionArgument;
use crate::argument_parser::t_parser::TParser;
use crate::argument::t_argument::TArgument;
// ======================================================================================
 // Recibe una linea de texto y parsea una lista de strings consigo una lista de argumentos
 // ======================================================================================
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }
}
// ======================================================================================
// Implementacion de TParser de Parser
// ======================================================================================
impl TParser for Parser {

    fn parse(&self, args: Vec<String>) -> Result<Vec::<Box<dyn TArgument>>, String> 
    {
        let mut program_index = 2;
        let mut vector = Vec::<Box<dyn TArgument>>::new();
        if args.len() < 2 {
            return Err("Invalid amount of parameters".to_string());
        }
        // ======================================================================================
        // Se usa to_lowercase para determinar -v para luego seguidamente hacer un push
        // de los argumentos al tracer 
        // ======================================================================================
        if args[1].to_lowercase().eq("-v"){
            vector.push(Box::new(TracerOptionArgument::new(args[1].clone())));
            
        }
        // ======================================================================================
        // Si no es el caso el indice del programa cambia a 1
        // ======================================================================================
        else {
            program_index = 1;
        }
        // ======================================================================================
        // De los argumentos del programa determina el largo del mismo para declararlo
        // en una senntecia de un vector
        // ======================================================================================
        vector.push(Box::new(ProgramArgument::new(&args[program_index..args.len()])));

        // ======================================================================================
        // Ok(), que representa el éxito y contiene un valor, y Err(E), 
        // que representa el error y contiene un valor de error. Ya que se agregó los argumentos respectivo
        // ======================================================================================
        Ok(vector)
    }
}