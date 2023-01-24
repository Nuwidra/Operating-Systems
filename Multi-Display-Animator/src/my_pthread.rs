/* ======================================================================================
 
   ____    ____           ___    ____   ___   ___
  / ___/  / __ \         |__ \  / __ \ |__ \ |__ \ 
  \__ \  / / / /  _____  __/ / / / / / __/ / __/ /
 ___/ / / /_/ /  /____/ / __/ / /_/ / / __/ / __/
/____/  \____/         /____/ \____/ /____//____/

Instituto Tecnológico de Costa Rica

Carrera: 
        Bachillerato en Ingeniería en Computación
Curso: 
        Principios de Sistemas Operativos
Profesor: 
        Kevin Moraga García
Alumnos:
        Alberto Zumbado Abarca
        Jonathan Quesada Salas
			
Proyecto 1: Multidisplay Animator

// ====================================================================================== */

// ======================================================================================
// Ejecución en Terminal para el funcionamiento del proyecto:
//                       [1] "./animar -c config.txt"
// ======================================================================================
use std::{mem};
use std::time::Duration;
use libc::{c_int, c_char, c_void, getcontext, makecontext, setcontext, sigemptyset, sigset_t, swapcontext, ucontext_t};
use nix::sys::signal;
use nix::sys::signal::{raise, SIGALRM, SigEvent, SigevNotify, SigHandler};
use nix::sys::timer::{Expiration, Timer, TimerSetTimeFlags};
use nix::time::ClockId;
use rand::Rng;


#[derive(Clone, PartialEq)]
pub struct ThreadControlBlock {
    pub thread_id : usize,
    pub function: extern fn(),
    pub args: *mut c_void,
    pub tickets: (usize, usize),
    pub priority: usize,
}

// ======================================================================================
// Se declara una estructura publica llamada ThreadPool
// ======================================================================================
#[derive(Clone, PartialEq)]
pub struct ThreadPool {
    pub created_threads_counter: usize,
    pub round_robin_threads: Vec<ThreadControlBlock>,
    pub real_time_threads: Vec<ThreadControlBlock>,
    pub lottery_threads: Vec<ThreadControlBlock>,
    pub actual_lottery_id: usize
}

// ======================================================================================
// Se declara una estructura publica llamada MutexStruct
// ======================================================================================
#[derive(Clone, PartialEq)]
pub struct MutexStruct {
    pub value: isize,
    pub processes: isize,
}


impl MutexStruct {
    // ======================================================================================
    // La función inicializa el mutex al que hace referencia mp con el tipo especificado 
    // por type . Tras una inicialización exitosa, el estado del mutex se inicializa y desbloquea.
    // ======================================================================================
    pub extern fn my_mutex_init(value: isize) -> Self {

        if value == 0 {
            panic!("Mutex value should 1 or more");
        }
        Self { value, processes: 0 }
    }
    // ======================================================================================
    // La función destruirá el objeto mutex al que hace referencia mutex; el objeto mutex 
    // queda, en efecto, sin inicializar. Una implementación puede hacer que establezca el 
    // objeto al que hace referencia mutex en un valor no válido.
    // ======================================================================================
    pub extern fn my_mutex_destroy(&mut self) {
        self.processes = -1;
    }
    // ======================================================================================
    // El bloqueo mutex solo lo liberará el subproceso que lo bloqueó. Por lo tanto, 
    // esto garantiza que una vez que un subproceso haya bloqueado un fragmento de código, 
    //ningún otro subproceso podrá ejecutar la misma región hasta que el subproceso que lo 
    // bloqueó lo desbloquee.
    // ======================================================================================
    pub extern fn my_mutex_lock(&mut self) {
        let current_locks = self.processes + 1;
        if current_locks > self.value {
            raise(SIGALRM).unwrap();
        }
        self.processes = current_locks;
    }
    // ======================================================================================
    // Si hay subprocesos bloqueados en el objeto mutex cuando se llaman, lo que hace que 
    // el mutex esté disponible, la política de programación se usa para determinar qué
    // ====================================================================================== 
    // subproceso adquiere el mutex.
    pub extern fn my_mutex_unlock(&mut self) {
        self.processes -= 1;
    }
    // ======================================================================================
    // Se permite que esta función falle espuriamente y devuelva falso incluso si el mutex 
    // no está actualmente bloqueado por ningún otro subproceso.
    // ======================================================================================
    pub extern fn my_mutex_trylock(&mut self) -> usize {
        let current_locks = self.processes + 1;
        if current_locks > self.value {
            return 1;
        }
        self.processes = current_locks;
        return 0;
    }
}


impl ThreadPool {

    pub fn new() -> Self {
        Self { created_threads_counter: 0, round_robin_threads: Vec::new(), real_time_threads: Vec::new(),
            lottery_threads: Vec::new(), actual_lottery_id: 0 }
    }
    // ======================================================================================
    // Esta función tiene como entradas el scheduler a crear con el hilo, la función que se 
    // quiera ingresa, adicionalmente se tiene los argumentos de la misma función y por último 
    // tendrá el número de tickets que tendrá el hilo a crear. Esta funcón principalmente se 
    // encarga de función inicia un nuevo hilo en la llamada proceso.
    // ======================================================================================
    pub unsafe extern fn my_thread_create(&mut self, scheduler: String, thread_function: extern fn(),
                                          function_args: *mut c_void, tickets: usize, priority: usize) -> usize {

        self.created_threads_counter += 1;

        // ======================================================================================
        // Se inicializa el nuevo hilo
        // ======================================================================================
        let mut new_thread: ThreadControlBlock = ThreadControlBlock {
            thread_id: self.created_threads_counter.clone(),
            function: thread_function,
            args: function_args,
            tickets: (0, 0),
            priority
        };
        // ======================================================================================
        // En caso que el planificador sea Lottery
        // ======================================================================================
        if scheduler == "Lottery".to_string() {
            if tickets <= 0 {
                panic!("Tickets has to be more than 0");
            }
            if self.created_threads_counter == 1 {
                new_thread.tickets = (1, tickets)
            } else {
                let last_thread_tickets = self.lottery_threads[self.lottery_threads.len() - 1].tickets.1;
                new_thread.tickets = (last_thread_tickets + 1, last_thread_tickets + tickets)
            }

            self.actual_lottery_id = new_thread.thread_id;
            self.lottery_threads.push(new_thread.clone());
        // ======================================================================================
        // En caso que el planificador sea RoundRobin
        // ======================================================================================
        } else if scheduler == "RoundRobin".to_string() {
            self.round_robin_threads.push(new_thread.clone());
        
        // ======================================================================================
        // En caso que el planificador sea RealTime
        // ======================================================================================
        } else if scheduler == "RealTime".to_string() {
            self.lottery_threads.push(new_thread.clone());

        } else {
            panic!("Unknown scheduler");
        }

        return self.created_threads_counter.clone();
    }
    // ======================================================================================
    // El hilo que espera, debe llamar a la función. Esta llamada hace que el hilo se "duerma" 
    // hasta que el otro hilo termine. Si el otro hilo ya había terminado, la función sale 
    // inmediatamente.
    // ======================================================================================
    pub unsafe extern fn my_pthread_join(mut self, parent_thread_id: usize, child_thread_id: usize) {

        let mut parent_thread: usize = 0;
        let mut child_thread: usize = 0;

        let mut parent_sched: usize = 0;
        let mut child_sched: usize = 0;

        let mut counter: usize = 0;
        // ======================================================================================
        // En el caso que sea para hilos de round robin
        // ======================================================================================
        for thread in self.round_robin_threads {
            if thread.thread_id == parent_thread_id {
                parent_thread = counter;
                parent_sched = 0;

            } else if thread.thread_id == child_thread_id {
                child_thread = counter;
                child_sched = 0;
            }

            counter += 1;
        }

        counter = 0;
        // ======================================================================================
        // En el caso que sea para hilos de Lottery
        // ======================================================================================
        for thread in self.lottery_threads {
            if thread.thread_id == parent_thread_id {
                parent_thread = counter;
                parent_sched = 1;

            } else if thread.thread_id == child_thread_id {
                child_thread = counter;
                child_sched = 1;
            }
            counter += 1;
        }

        counter = 0;
        // ======================================================================================
        // En el caso que sea para hilos de real time
        // ======================================================================================
        for thread in self.real_time_threads {
            if thread.thread_id == parent_thread_id {
                parent_thread = counter;
                parent_sched = 2;

            } else if thread.thread_id == child_thread_id {
                child_thread = counter;
                child_sched = 2;
            }
        }

        join_aux(parent_thread , parent_sched, child_thread, child_sched);

    }

    // ======================================================================================
    // La función permite que un subproceso ceda el control de un procesador para que otro 
    // subproceso pueda tener la oportunidad de ejecutarse. El parámetro de la función debe 
    // ser NULL, porque los valores que no son NULL están reservados.
    // ======================================================================================
    pub unsafe extern fn my_pthread_yield(self) {
        yield_aux();

    }
    // ======================================================================================
    // ejecuta los hilos
    // ======================================================================================
    pub unsafe extern fn run_threads(self) {
        run(self)

    }
}


static mut SET: Option<sigset_t> = None;

static mut ACTUAL_THREAD: Option<ucontext_t> = None;
static mut SCHEDULER_CONTEXT: Option<ucontext_t> = None;

static mut ROUND_ROBIN_ACTUAL_THREAD: Option<ucontext_t> = None;
static mut ROUND_ROBIN_VEC: Option<Vec<ucontext_t>> = None;
static mut ROUND_ROBIN_COUNTER: usize = 0;

static mut LOTTERY_ACTUAL_THREAD: Option<ucontext_t> = None;
static mut LOTTERY_VEC: Option<Vec<ucontext_t>> = None;
static mut LOTTERY_TICKETS_VEC: Option<Vec<(usize, usize)>> = None;
static mut LOTTERY_COUNTER: usize = 0;


static mut SCHEDULERS_FUNCTION_MAP: Vec<unsafe extern fn()> = Vec::new();
static mut ACTUAL_ALGORITHM: usize = 0;

static mut SCHEDULED: usize = 0;
static mut COUNTER: usize = 0;

static mut DETACHED: Option<Vec<(String, usize)>> = None;


// ======================================================================================
// Es el manipulador de la señal:
// Esta funcion se encargará se realizar un cambio en el contexto a los respectivos 
// planificadores que puedan llegar a ejecutar un hilo, dicho contexto se extraerá con 
// ayuda de ucontext.
// ======================================================================================
extern "C" fn signal_handler(__a: c_int) {
    unsafe {
        if ACTUAL_ALGORITHM == 0 {
            swapcontext(get_actual_round_robin_thread() as *mut ucontext_t, scheduler_context() as *const ucontext_t);
        } else if ACTUAL_ALGORITHM == 1 {
            swapcontext(get_actual_lottery_thread() as *mut ucontext_t, scheduler_context() as *const ucontext_t);
        }
    }
}
// ======================================================================================
// Este scheduler se caracteriza por la espera circular que tiene, ya que tenemos una 
// cola donde los procesos están ordenados por orden de llegada. En este caso se tiene 
// que validar si la cantidad actual de los hilos esta vacía, para saber de ser lo contrario 
// se determine un vector que contendrá los hilos de round robin.
// ======================================================================================
unsafe extern "C" fn round_robin() {

    if ROUND_ROBIN_VEC.clone().unwrap().is_empty() {
        setcontext(scheduler_context() as *mut ucontext_t);
    }

    let mut thread: Option<ucontext_t>;
    let mut counter = ROUND_ROBIN_COUNTER;

    if ROUND_ROBIN_ACTUAL_THREAD.is_none() {
        ROUND_ROBIN_COUNTER = 0;
        thread = Some(ROUND_ROBIN_VEC.clone().unwrap()[ROUND_ROBIN_COUNTER]);
        ROUND_ROBIN_ACTUAL_THREAD = thread;

    } else {
        counter += 1;
        if counter == ROUND_ROBIN_VEC.clone().unwrap().len() {
            ROUND_ROBIN_COUNTER = 0;
        } else {
            ROUND_ROBIN_COUNTER = counter;
        }
        thread = Some(ROUND_ROBIN_VEC.clone().unwrap()[ROUND_ROBIN_COUNTER]);
        ROUND_ROBIN_ACTUAL_THREAD = thread;
    }
}
// ======================================================================================
// Es un algoritmo de programación probabilística para procesos en un sistema operativo. 
// A cada proceso se le asigna una cierta cantidad de boletos de lotería, y el planificador 
// extrae un boleto al azar para seleccionar el siguiente proceso. La distribución de boletos 
// no necesita ser uniforme; otorgar a un proceso más boletos le brinda una probabilidad 
// relativamente mayor de selección.
// ======================================================================================
unsafe extern "C" fn lottery() {

    if LOTTERY_VEC.clone().unwrap().is_empty() {
        setcontext(scheduler_context() as *mut ucontext_t);
    }

    let mut threads = LOTTERY_VEC.clone().unwrap();
    let lottery_threads_len = threads.len() - 1;
    let mut tickets = LOTTERY_TICKETS_VEC.clone().unwrap();

    let mut rng = rand::thread_rng();

    let mut winner_lottery = rng.gen_range(1..tickets[lottery_threads_len].1 + 1);
    let mut index: usize = 0;

    for thread in tickets {

        if (thread.0..thread.1).contains(&winner_lottery) {
            LOTTERY_ACTUAL_THREAD = Some(threads[index]);
            LOTTERY_COUNTER = index;
            break;
        }
        index += 1;
    }
}
// ======================================================================================
// Esta función tendrá como entrada un vector que tiene todo el bloque de control de los 
// hilos de round robin y adicionalmente una respectiva parada, en esta función se establece 
// que a partir de un vector se pueda obtener un respectivo contexto del los hilos del 
// planificador, para que de esta manera se pueda realizar un contexto, haciendo que pueda 
// seguir avanzando con el siguiente proceso sucesivamente.
// ======================================================================================
unsafe extern fn set_round_robin_contexts(stop: usize, round_robin: Vec<ThreadControlBlock>) {

    if stop == 0 {
        return;
    }
    let mut round_robin_copy = ROUND_ROBIN_VEC.clone().unwrap();
    round_robin_copy.push(mem::MaybeUninit::uninit().assume_init());
    getcontext(&mut round_robin_copy[ROUND_ROBIN_COUNTER] as *mut ucontext_t);

    round_robin_copy[ROUND_ROBIN_COUNTER]
        .uc_stack.ss_sp = stack().as_mut_ptr() as *mut c_void;

    round_robin_copy[ROUND_ROBIN_COUNTER]
        .uc_stack.ss_size = mem::size_of_val(&round_robin_copy[ROUND_ROBIN_COUNTER].uc_stack.ss_sp);


    makecontext(&mut round_robin_copy[ROUND_ROBIN_COUNTER] as *mut ucontext_t,
         round_robin[ROUND_ROBIN_COUNTER].function, 1, round_robin[ROUND_ROBIN_COUNTER].args);

    ROUND_ROBIN_COUNTER += 1;
    ROUND_ROBIN_VEC = Some(round_robin_copy);
    set_round_robin_contexts(stop - 1, round_robin);
}

// ======================================================================================
// Esta función tendrá como entrada un vector que tiene todo el bloque de control de los 
// hilos de lottery y adicionalmente una respectiva parada, en esta función se establece 
// que a partir de un vector se pueda obtener un respectivo contexto del los hilos del 
// planificador, para que de esta manera se pueda realizar un contexto, haciendo que pueda 
// seguir avanzando con el siguiente proceso sucesivamente.
// ======================================================================================
unsafe extern fn set_lottery_contexts(stop: usize, lottery: Vec<ThreadControlBlock>) {

    if stop == 0 {
        return;
    }

    let mut lottery_tickets = LOTTERY_TICKETS_VEC.clone().unwrap();
    let mut lottery_copy = LOTTERY_VEC.clone().unwrap();

    lottery_copy.push(mem::MaybeUninit::uninit().assume_init());

    getcontext(&mut lottery_copy[LOTTERY_COUNTER] as *mut ucontext_t);

    lottery_copy[LOTTERY_COUNTER]
        .uc_stack.ss_sp = stack().as_mut_ptr() as *mut c_void;

    lottery_copy[LOTTERY_COUNTER]
        .uc_stack.ss_size = mem::size_of_val(&lottery_copy[LOTTERY_COUNTER].uc_stack.ss_sp);

    lottery_tickets.push(lottery[LOTTERY_COUNTER].tickets);

    makecontext(&mut lottery_copy[LOTTERY_COUNTER] as *mut ucontext_t, lottery[LOTTERY_COUNTER].function, 1, 0);

    LOTTERY_COUNTER += 1;
    LOTTERY_VEC = Some(lottery_copy);
    LOTTERY_TICKETS_VEC = Some(lottery_tickets);
    set_lottery_contexts(stop - 1, lottery);

}

// ======================================================================================
// Esta funcion se encarga de inicializar los hilos que esta en el Pool de hilos
// ======================================================================================
unsafe extern "C" fn init(threads: ThreadPool) {

    let scheduler_handler = SigHandler::Handler(signal_handler);
    signal::signal(SIGALRM, scheduler_handler).unwrap();

    LOTTERY_TICKETS_VEC = Some(Vec::new());

    SCHEDULERS_FUNCTION_MAP.push(round_robin);
    SCHEDULERS_FUNCTION_MAP.push(lottery);

    SET = Some(mem::MaybeUninit::uninit().assume_init());
    ROUND_ROBIN_VEC = Some(Vec::new());
    LOTTERY_VEC = Some(Vec::new());

    let round_robin = threads.round_robin_threads;
    let lottery = threads.lottery_threads;

    let mut len = round_robin.len();
    set_round_robin_contexts(len, round_robin);
    ROUND_ROBIN_COUNTER = 0;

    len = lottery.len();
    set_lottery_contexts(len, lottery);
    LOTTERY_COUNTER = 0;


}

// ======================================================================================
// Esta función se encargará de manejar una señal para poder determinar si un proceso 
// en el planificador se haya terminado, de manear que no se reinicie y vuelva a inicializarse 
// con un valor incorrecto de dirección de memoria.
// ======================================================================================
unsafe extern "C" fn set_signal() {

    let clock_id = ClockId::CLOCK_MONOTONIC;
    let signal_event = SigEvent::new(SigevNotify::SigevSignal {
        signal: SIGALRM,
        si_value: 0,
    });

    let mut timer = Timer::new(clock_id, signal_event).unwrap();
    let expiration = Expiration::OneShot(Duration::from_millis(80).into());
    let flags = TimerSetTimeFlags::empty();
    timer.set(expiration, flags).expect("could not set timer");

    if ACTUAL_ALGORITHM == 0 {
        setcontext(get_actual_round_robin_thread() as *mut ucontext_t);

    } else if ACTUAL_ALGORITHM == 1 {
        setcontext(get_actual_lottery_thread() as *mut ucontext_t);
    }

    let expiration = Expiration::OneShot(Duration::from_secs(0).into());
    let flags = TimerSetTimeFlags::empty();
    timer.set(expiration, flags).expect("could not set timer");

}

// ======================================================================================
// Esta funcion se encargará de establecer hilos padres e hilos hijos para que estos 
// mismos puedan llegar a ejecutarse de manera que puedan iniciar los vectores que deben
// de tener los planificadores
// ======================================================================================
extern "C" fn join_aux(parent_thread: usize, parent_sched: usize, child_thread: usize, child_sche: usize) {
    unsafe {
        // ======================================================================================
        // Planificador hijo "false"
        // ======================================================================================
        if parent_sched == 0 {
            ACTUAL_THREAD = Some(ROUND_ROBIN_VEC.clone().unwrap()[parent_thread]);
        
        // ======================================================================================
        // Planificador hijo "true"
        // ======================================================================================
        } else if  parent_sched == 1 {
            ACTUAL_THREAD = Some(LOTTERY_VEC.clone().unwrap()[parent_thread]);
        }
        // ======================================================================================
        // Planificador hijo "false"
        // ======================================================================================
        if child_sche == 0 {
            let mut a = ACTUAL_THREAD.unwrap();
            a.uc_link = &mut ROUND_ROBIN_VEC.clone().unwrap()[child_thread] as *mut ucontext_t;
            ACTUAL_THREAD = Some(a);
        // ======================================================================================
        // Planificador hijo "true"
        // ======================================================================================
        } else if child_sche == 1 {
            let mut a = ACTUAL_THREAD.unwrap();
            a.uc_link = &mut LOTTERY_VEC.clone().unwrap()[child_thread] as *mut ucontext_t;
            ACTUAL_THREAD = Some(a);
        }
        setcontext(actual_context() as *mut ucontext_t);
    }
}

extern "C" fn yield_aux() {
    raise(SIGALRM).expect("TODO: panic message");
}
// ======================================================================================
// Es la función encargada de almancenar en un vector la información del hilo de manera 
// que este mismo vector pueda ser tratado con un respectivo planificador y que este mismo 
// pueda sumarse a su vector principal de almacenamiento donde el planificador tendrá todos 
// los hilos respectivos.
// ======================================================================================
extern "C" fn scheduler() {
    unsafe {

        if SCHEDULED == 1 {
            let mut contexts_vec: Vec<ucontext_t>;
            // ======================================================================================
            // En el caso que sea round robin
            // ======================================================================================
            if ACTUAL_ALGORITHM == 0 {
                contexts_vec = ROUND_ROBIN_VEC.clone().unwrap();
                contexts_vec[ROUND_ROBIN_COUNTER] = get_actual_round_robin_thread().clone();
                ROUND_ROBIN_VEC = Some(contexts_vec);
            // ======================================================================================
            // En el caso que sea lottery
            // ======================================================================================
            } else if ACTUAL_ALGORITHM == 1 {
                contexts_vec = LOTTERY_VEC.clone().unwrap();
                contexts_vec[LOTTERY_COUNTER] = get_actual_lottery_thread().clone();
                LOTTERY_VEC = Some(contexts_vec);
            } 

        }

        let alternator: usize;

        alternator = 0;

        SCHEDULED = 1;
        SCHEDULERS_FUNCTION_MAP[alternator]();
        ACTUAL_ALGORITHM = alternator;

        set_signal();
    }
}
// ======================================================================================
// run() se encarga de ejecutar los hilos según el contexto que tengan
// ======================================================================================
pub fn run(threads: ThreadPool) {
    unsafe {
        // ======================================================================================
        // Se inicializa los hilos
        // ======================================================================================
        init(threads);
        SCHEDULER_CONTEXT = Some(mem::MaybeUninit::uninit().assume_init());
        // ======================================================================================
        // Se obtiene el contexto el planificador
        // ======================================================================================
        getcontext(scheduler_context() as *mut ucontext_t);

        let mut stack: [c_char; 8192] = [mem::zeroed(); 8192];
        scheduler_context().uc_stack.ss_sp = stack.as_mut_ptr() as *mut c_void;
        scheduler_context().uc_stack.ss_size = mem::size_of_val(&stack);
        sigemptyset(&mut scheduler_context().uc_sigmask);

        // ======================================================================================
        // Se realiza el contexto del respectivo planificador
        // ======================================================================================
        makecontext(scheduler_context() as *mut ucontext_t, scheduler, 0);

        setcontext(scheduler_context() as *mut ucontext_t);

    }
}

// ======================================================================================
// Se setea un contexto acutual
// ======================================================================================
unsafe fn actual_context() -> &'static mut ucontext_t {
    match ACTUAL_THREAD {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

// ======================================================================================
// Se setea un contexto del planificador
// ======================================================================================
unsafe fn scheduler_context() -> &'static mut ucontext_t {
    match SCHEDULER_CONTEXT {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

// ======================================================================================
// Obtiene el hilo actual de round robin
// ======================================================================================
unsafe fn get_actual_round_robin_thread() -> &'static mut ucontext_t {
    match ROUND_ROBIN_ACTUAL_THREAD {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

// ======================================================================================
// Obtiene el hilo actual de lottery
// ======================================================================================
unsafe fn get_actual_lottery_thread() -> &'static mut ucontext_t {
    match LOTTERY_ACTUAL_THREAD {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

// ======================================================================================
// Funcion que ayudará a almacenar datos en una pila
// ======================================================================================
unsafe fn stack() -> [c_char; 8192] {
    [mem::zeroed(); 8192]

}