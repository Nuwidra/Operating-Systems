use crate::argument::t_argument::TArgument;
use crate::arguments_summary::ArgumentsSummary;
// ======================================================================================
// La estructura de TracerOptionArgument es almacenar la información de los argumentos que 
// tenga la linea de comando en caso que exista -v y -V
// ======================================================================================
pub struct TracerOptionArgument {
    pub tracer_argument_text: String
}
// ======================================================================================
// Se crea una implementacion de TracerOptionArgument para almacenar la información de los argumentos
// ======================================================================================
impl TracerOptionArgument {
    pub fn new(option: String) -> TracerOptionArgument {
        TracerOptionArgument {
            tracer_argument_text: option
        }
    }
}

impl TArgument for TracerOptionArgument {
    // ======================================================================================
    // Se obtiene el texto del texto ya confirmado en caso que tenga las sentencias -v y -V
    // ======================================================================================
    fn get_text(&self) -> &String
    {
        &self.tracer_argument_text
    }
    // ======================================================================================
    // En dado caso que se haya encontrado un syscall se levanta una bandera para indicar que se encontró
    // ======================================================================================
    fn summarize_argument(&self, summary:&mut ArgumentsSummary)
    {
        summary.print_syscall_found = true;
        // ======================================================================================
        // Con los argumentos que tenga la variable tracer_argument_text 
        // Como la mayoría de las operaciones de indexación, el conteo comienza desde cero, 
        // por lo que nth(0) devuelve el primer valor, nth(1) el segundo, y así sucesivamente.

        // Se usa unwrap para que me pueda dar el cálculo, y si hubo un error, entra en pánico y detén el programa
        // ======================================================================================
        let ch = self.tracer_argument_text.chars().nth(1).unwrap();
        // ======================================================================================
        // El metodo is_uppercase servirá para saber cuando se usa -V en el programa.
        // ======================================================================================
        summary.pause_when_syscall_found = ch.is_uppercase();
    }
}
