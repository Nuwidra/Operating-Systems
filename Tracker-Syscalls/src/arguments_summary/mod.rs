use crate::argument::t_argument::TArgument;
#[derive(Debug, Default)]

// ======================================================================================
// La estructura de ArgumentsSummary consistirá en:
// print_syscall_found: Esta parte nos ayudará para -v y -V ya que es en caso que se encuentre el syscall
// pause_when_syscall_found: Esta parte de la estructura nos servirá para determinar cuando usar el -V
// program_command: El cual consiste en el strig que uno ingrese en la terminal para la ejecución del programa
// ======================================================================================
pub struct ArgumentsSummary {
    pub print_syscall_found : bool,
    pub pause_when_syscall_found : bool,
    pub program_command : String
}
// ======================================================================================
// ArgumentsSummary se encargará de tomar los argumentos de agument_parser y summary
// Para recibir una lista de argumentos y resumirlos en un objeto llamado ArgumentsSummary
// Para que el tracer no se preocupe en los argumentos y solo llame a ArgumentsSummary de la información que necesita
// ======================================================================================
impl ArgumentsSummary {
    pub fn summarize(arguments: Vec::<Box<dyn TArgument>>) -> ArgumentsSummary {

        let mut summary = ArgumentsSummary::default();
        for f in arguments {
            f.summarize_argument(&mut summary);
        }
        summary
    }
}
