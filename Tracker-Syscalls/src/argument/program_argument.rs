use crate::argument::t_argument::TArgument;
use crate::arguments_summary::ArgumentsSummary;
// ======================================================================================
// Se define la función para el string que pueda tener la entrada del programa
// ======================================================================================
pub struct ProgramArgument {
    // ======================================================================================
    // La linea de comando del pograma se declara como string y publica
    // ======================================================================================
    pub program_command: String
}
// ======================================================================================
// ProgramArgument se encarga de dar formato a los argumentos del comando ingresado
// ======================================================================================
impl ProgramArgument {
    // ======================================================================================
    // Se inicializa una nueva variable con lo argumentos
    // ======================================================================================
    pub fn new(slice: &[String]) -> ProgramArgument {
        // ======================================================================================
        // slice es un puntero a un bloque de memoria
        // Los segmentos se pueden utilizar para acceder a porciones de datos almacenados ç
        // en bloques de memoria contiguos
        // ======================================================================================
        ProgramArgument {
            // ======================================================================================
            // Los segmentos usan números de índice para acceder a porciones de datos.
            // ======================================================================================
            program_command: slice.join(" ").to_string()
        }
    }
}

impl TArgument for ProgramArgument {
    // ======================================================================================
    // Se obtiene el texto de la linea de comando
    // ======================================================================================
    fn get_text(&self) -> &String
    {
        &self.program_command
    }
    // ======================================================================================
    // Se declara el resumen de los argumentos de la linea de comando
    // ======================================================================================
    fn summarize_argument(&self, summary:&mut ArgumentsSummary)
    {
        summary.program_command = self.program_command.clone();
    }
}

