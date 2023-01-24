// ======================================================================================
// Llamado del modulo de todos los syscalls names
// ======================================================================================
pub mod system_call_names;

extern crate execute;
// ======================================================================================
// Bibliotecas necesarias para la realización del mod.rs del módulo tracer
// ======================================================================================
use std::collections::HashMap;
use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use std::os::unix::process::CommandExt;
use std::io;
use std::io::Write;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process::{exit, Stdio};
use execute::command;
// ======================================================================================
// El uso de ArgumentsSummary y TRunner (Tracer Runner)
// ======================================================================================
use crate::arguments_summary::ArgumentsSummary as ArgumentsSummary;
use crate::runner::t_runner::TRunner;

#[derive(Default)]
// ======================================================================================
// Se define la estructura de Tracer la cual consiste en:
//      arguments_summary: los argumentos datos en el módulo arguments_summary se contienen en esta estructura
//      sys_calls_summary: Será un hashmap para el resumen de todos los syscalls
// ======================================================================================
pub struct Tracer {
    pub arguments_summary: ArgumentsSummary,
    pub sys_calls_summary: HashMap::<usize, u16>
}

// ======================================================================================
// La implementación de tracer
// ======================================================================================
impl Tracer {
    // ======================================================================================
    // Se crean nuevas instancia para almacenar toda información aportada por parte de 
    // arguments_summary y sys_calls_summary explicadas al inicio de este archivo  
    // ======================================================================================
    pub fn new(summary: ArgumentsSummary) -> Tracer {
        Tracer {
            arguments_summary: summary,
            sys_calls_summary: HashMap::<usize, u16>::new()
        }
    }

    // ======================================================================================
    // Esta función se encarga de ir añadiendo cada system call en una lista para luego,
    // esa misma lista es pasada a las funciones de print para que los impriman continuamente
    // ======================================================================================
    fn add_sys_call(&mut self, sys_call: usize){
        self.sys_calls_summary.entry(sys_call).and_modify(|counter| *counter += 1).or_insert(1);
    }

    // ======================================================================================
    // Esta función se encarga de imprimir el resumen de los syscalls diciendo la cantidad de veces que fueron usados
    // ======================================================================================
    fn print_sys_call_summary(&self){
        println!("******SYSTEM CALLS SUMMARY******");
        // ======================================================================================
        // Ciclo que itera en todos los system calls y los imprime continuamente añadiendo su respectivo valor,
        // para que de esta manera imprima las veces que se repite un syscall en el sistema
        // ======================================================================================
        for (key, val) in self.sys_calls_summary.iter() {
            println!("{} {}",  system_call_names::SYSTEM_CALL_NAMES[*key], val);
         }
    }
    // ======================================================================================
    // Esta función se encarga de imprimir los system calls en su totalidad, a partir
    // del módulo de system_call_names.rs que este mismo contiene la totalidad de los system calls
    // ======================================================================================
    fn print_sys_call(registers: libc::user_regs_struct){
        let sys_call = (registers.orig_rax) as usize;
        println!("\x1b[96m{:?}\x1b[0m",system_call_names::SYSTEM_CALL_NAMES[sys_call]);
        println!("\r{:?}\r\n\r",registers);
    }

    // ======================================================================================
    // La función run_tracer consistira en ejecuta las instrucciones dependiendo que sea -v y -V
    // ======================================================================================
    fn run_tracer(&mut self, child: Pid) {
        
        let mut stdout = io::stdout().into_raw_mode().unwrap();
        // ======================================================================================
        // El uso de flush() es porque garantiza que la salida se emita inmediatamente.
        // Y el unwrap() es para obtener el resultado del cálculo, y si hubo un error, entra en pánico y detén el programa
        // ======================================================================================
        stdout.flush().unwrap();
        // ======================================================================================
        // Se usa asynchronous stdin:
        // Esto le permite leer desde la entrada estándar sin bloquear el hilo actual. Específicamente, 
        // funciona activando otro subproceso para manejar el flujo de eventos
        // ======================================================================================
        let mut stdin = termion::async_stdin().keys();
        
        // ======================================================================================
        // La función entra en un ciclo infinito para recorrer la totalidad de los syscalls
        // ====================================================================================== 
        loop {
            wait().unwrap();

            let sys_call_result = ptrace::getregs(child);
            // ======================================================================================
            // En dado caso de error hacer break
            // ======================================================================================
            if let Err(_) = sys_call_result {
                break;
            }
            else {
                // ======================================================================================
                // Se declara la variable sys_call para poder almacenar la totalidad de los syscalls en el caso del -v
                // ======================================================================================
                let sys_call = (sys_call_result.unwrap().orig_rax) as usize;

                // ======================================================================================
                // En caso que se encuentre el syscall se hace match para que se pueda imprimir dicho syscall
                // ======================================================================================
                if self.arguments_summary.print_syscall_found {
                    Tracer::print_sys_call(sys_call_result.unwrap());
                }
                // ======================================================================================
                // Se añade el syscall y se vuelve al ciclo hasta que no haya más syscalls
                // ======================================================================================
                self.add_sys_call(sys_call);              
            }
            // ======================================================================================
            // En el caso de el resumen de los argumentos tenga pause_when_syscall_found eso querrá decir
            // que es la funcionalidad del -V, ya que este mismo necesita que el usuario presione cualquier
            // tecla para la continuación de la impresión de los syscalls y las cantidad de veces usadas
            // dichos llamados de syscall
            // ======================================================================================
            if self.arguments_summary.pause_when_syscall_found{
                loop {
                    // ======================================================================================
                    // Si es el caso que se presione una tecla sigue con el siguiente syscall
                    // hasta que se termine dichas sentencias
                    // ======================================================================================
                    if let Some(Ok(key)) =  stdin.next() {
                        // ======================================================================================
                        // Cosa añadida mía jeje:
                        // En el dado caso que uno se aburra de estar presionando las teclas por la cantidad de 
                        // syscalls se tendrá que presionar la letra q del teclado para evitar fatiga jeje.
                        // ======================================================================================
                        match key {
                            // ======================================================================================
                            // Si se presiona "q" sale del programa
                            // ======================================================================================
                            termion::event::Key::Char('q') => return,
                            // ======================================================================================
                            // De lo contrario, imprime la tecla presionada
                            // ======================================================================================
                            _ => break
                        }
                    }
                }
            }
            // ======================================================================================
            // En el caso de match se evalua si se rompe el ciclo o si se continua, ya que se evalua
            // si el hilo de ejecución tiene más hijos o no. 
            // ======================================================================================
            match ptrace::syscall(child, None) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
    }
    // ======================================================================================
    // Funcion que analiza el comando con los parametros que estan después del -v y -V
    // ======================================================================================
    fn run_tracee(&self) {
        // ======================================================================================
        // El método de traceme() indica que este proceso debe ser rastreado por su padre.
        // ======================================================================================
        ptrace::traceme().unwrap();
        // ======================================================================================
        // Esta caja es un envoltorio seguro de tipo alrededor de la función de personalidad de Linux.
        // ======================================================================================
        personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();

        // ======================================================================================
        // Se almacena el resumen de los argumentos y la linea de comando en la variable mutable command
        // ======================================================================================
        let mut command = command(self.arguments_summary.program_command.clone());
        
        // ======================================================================================
        // Construye un nuevo identificador para la salida estándar del proceso actual
        // ======================================================================================
        command.stdout(Stdio::piped());
        // ======================================================================================
        // Construye un nuevo controlador para el error estándar del proceso actual
        // Algo importante a considerar es que stderr() no está almacenado en búfer
        // ======================================================================================
        command.stderr(Stdio::piped());
        // ======================================================================================
        // Se ejecuta command
        // ======================================================================================
        command.exec();
        exit(0)
    }
}

// ======================================================================================
// Esta implementación permitirá poder ejecutar un proceso hijo y padre
// ======================================================================================
impl TRunner for Tracer {
    // ======================================================================================
    // Se declara execute el cual se encargará de ejecurar los hilos de ejecución padre e hijo
    // ======================================================================================
    fn execute(&mut self) 
    {
        match unsafe { fork() } {
            // ======================================================================================
            // En dado cado caso que sea el hijo de ejecución del hijo
            // ======================================================================================
            Ok(ForkResult::Child) => {
                self.run_tracee();
            } 
            // ======================================================================================
            // En dado cado caso que sea el hijo de ejecución del padre se ejecutará run_tracer y print_sys_call_summary
            // que este mismo imprimirá los syscalls y su resumen de las veces usadas de dicho syscall
            // ======================================================================================
            Ok(ForkResult::Parent { child }) => {
                self.run_tracer(child);
                self.print_sys_call_summary();
            }
            // ======================================================================================
            // En dado cado caso de un error retornar un mensaje de error
            // ======================================================================================
            Err(err) => {
                panic!("[main] fork() failed: {}", err);
            }
        }
    }
}