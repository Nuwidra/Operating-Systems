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

// ======================================================================================
// Bibliotecas necesarias para la animación, movimiento y el pintado de la misma
// ======================================================================================
use ncurses::ll::chtype;
use ncurses::ll::wborder;
use ncurses::mvwprintw;
use ncurses::initscr;
use ncurses::noecho;
use ncurses::refresh;
use ncurses::newwin;
use ncurses::endwin;
use std::fs::File;
use libc::sleep;
use std::io::Read;
use ncurses::wrefresh;
use ncurses::ll::werase;


extern crate core;
// ======================================================================================
// Se importa la biblioteca my_pthread
// ======================================================================================
use crate::my_pthread;
use my_pthread::MutexStruct;
use my_pthread::run;
use my_pthread::ThreadPool;
use std::{mem};
use libc::{c_void};

static mut MY_PTHREAD: Option<ThreadPool> = None;
static mut MY_MUTEX: Option<MutexStruct> = None;
// ======================================================================================
// Se define la estructura del lenguaje, la cual tendrá los siguientes parámetros
// ======================================================================================
#[derive(Debug)]
pub struct language_structure {

    protocol: String,
    monitor_number: i32,
    id_monitor: i32,
    width_canvas: i32,
    height_canvas: i32,
    start_time: String,
    id_animation: i32,
    object_ascii: Vec<String>,
    initial_position_in_x: i32,
    initial_position_in_y: i32,
    final_position_in_x: i32,
    final_position_in_y: i32,
    angle: i32,
    final_time: String,
    object_ascii_90: Vec<String>,
    object_ascii_120: Vec<String>,
    object_ascii_270: Vec<String>,
    figure_2: Vec<String>,
    initial_position_in_x_2: i32,
    initial_position_in_y_2: i32,
    final_position_in_x_2: i32,
    final_position_in_y_2: i32,
    figure_2_90: Vec<String>,
    figure_2_120: Vec<String>,
    figure_2_270: Vec<String>,
}
// ======================================================================================
// Esta función se encargará de inicializar el archivo y sus elementos, para que estos
// puedan ser inicializados de manera que se puedan establecer las figuras iniciales
// dadas en el config.txt, para que posteriormente puedan ejecutarse las dos funciones
// que estas mismas tienen un movimiento especifico, dicho llamado será a traves de la
// biblioteca de my_pthread
// ======================================================================================
pub fn animator(string: String) { unsafe{
    MY_MUTEX = Some(MutexStruct::my_mutex_init(1));
    // ======================================================================================
    // Necesario inicializar una x y una y para ncurses, ya que estas mismas determinan el 
    // eje X y el eje Y
    // ======================================================================================
    let x = 0;
    let y = 0;
    initscr();
    noecho();
    // ======================================================================================
    // Se parsea el archivo y se almacena en la variable file
    // ======================================================================================
    let file = parser(string);
    // ======================================================================================
    // config serán las posiciones que tendrá cada elemento del archivo, para que de estas 
    // mismas puedan ser manejadas con más facilidad
    // ======================================================================================
    let mut config = determinate_txt(file);
    // ======================================================================================
    // Se inicializan en variables el ancho y el alto que va a tener el canva total 
    // ======================================================================================
    let define_height_canva = config.height_canvas;
    let define_width_canva = config.width_canvas;

    // ======================================================================================
    // Se crea la ventana la cual tendrá presente las inicalizaciones anteriores para el 
    // ancho y el alto, para que de esta manera se pueda hacer referencia a terminal con newwin
    // ======================================================================================
    let window = newwin(define_height_canva, define_width_canva, y, x);
    // ======================================================================================
    // Se refresca la pantalla para que se actualice dadas las inidicaciones dadas
    // ======================================================================================
    refresh();

    // ======================================================================================
    // Se inicializan el aspecto que deberá tener el canvas, filas, columnas y esquinas
    // ======================================================================================
    let corner = chtype::from('.');
    let row = chtype::from('.');
    let column = chtype::from('.');

    // ======================================================================================
    // Con ayuda de wborder se escribe la ventana respectiva para que se pinte el límite del canvas
    // ======================================================================================
    unsafe{wborder(window, column, column, row, row, corner, corner, corner, corner)};
    // ======================================================================================
    // Se refresca la pantalla para que se actualice dadas las inidicaciones dadas
    // ======================================================================================
    refresh();

    // ======================================================================================
    // Se inicializa la primera figura dada una posicion inicial en el eje X y en el eje Y
    // ======================================================================================
    let initialization_y = config.initial_position_in_y; 
    let initialization_x = config.initial_position_in_x;
    // ======================================================================================
    // Imprime dicha inicializacion de la figura respectiva a las coordenadas en x y
    // ======================================================================================
    mvwprintw(window, initialization_y, initialization_x, "");

    // ======================================================================================
    // Se determina el largo del objeto ascii en cuestion para que este mismo pueda imprimirse
    // ======================================================================================
    let len_ascii = config.object_ascii.len();
    // ======================================================================================
    // Se recorre todo el largo de la figura caracter por caracter
    // ======================================================================================
    for character in 0..len_ascii {
        // ======================================================================================
        // Se tienen que sumar la posicion inicial en "y" y los caracteres porque se debe 
        // contemplar toda la figura, ya que es como una columna
        // ======================================================================================
        let traverse_ascii_y = config.initial_position_in_y + character as i32;
        // ======================================================================================
        // Dado la posicion en x no se le hace nada por que esta ya fue previamente leida en parser
        // ======================================================================================
        let traverse_ascii_x = config.initial_position_in_x;
        // ======================================================================================
        // Se inicializa el objeto en ascii que se quiera imprimir
        // ======================================================================================
        let figure_ascii = &config.object_ascii[character];
        // ======================================================================================
        // Se imprime todo lo anterior explicado
        // ======================================================================================
        mvwprintw(window, traverse_ascii_y, traverse_ascii_x, figure_ascii);
    }

    // ======================================================================================
    // Se determina el largo del objeto ascii en cuestion para que este mismo pueda imprimirse
    // ======================================================================================
    let len_ascii_2 = config.figure_2.len();
    // ======================================================================================
    // Se recorre todo el largo de la figura caracter por caracter
    // ======================================================================================
    for character in 0..len_ascii_2 {
        // ======================================================================================
        // Se tienen que sumar la posicion inicial en "y" y los caracteres porque se debe 
        // contemplar toda la figura, ya que es como una columna
        // ======================================================================================
        let traverse_ascii_y = config.initial_position_in_y_2 + character as i32;
        // ======================================================================================
        // Dado la posicion en x no se le hace nada por que esta ya fue previamente leida en parser
        // ======================================================================================
        let traverse_ascii_x = config.initial_position_in_x_2;
        // ======================================================================================
        // Se inicializa el objeto en ascii que se quiera imprimir
        // ======================================================================================
        let figure_ascii = &config.figure_2[character];
        // ======================================================================================
        // Se imprime todo lo anterior explicado
        // ======================================================================================
        mvwprintw(window, traverse_ascii_y, traverse_ascii_x, figure_ascii);
    }
    // ======================================================================================
    // En este reinicio de la pantalla se pueden apreciar las dos figuras respectivamente
    // su posicion incial
    // ======================================================================================
    wrefresh(window);
    // ======================================================================================
    // Tiempo de espera para que se pueda apreciar la figura
    // ======================================================================================
    unsafe{sleep(1)};
    // ======================================================================================
    // Se declara state_str la cual nos ayudará para la ejecución de las figuras con ayuda de my_pthread
    // ======================================================================================
    let config_ptr: *mut c_void = &mut config as *mut _ as *mut c_void;
    // let mut config_void: &mut language_structure = unsafe { &mut *(config_ptr as *mut language_structure) };
    // let mut config_parse = unsafe { mem::transmute::<*mut c_void, language_structure>(config_void) } ;

    // println!("{:?}", config_void);
    // ======================================================================================
    // Se declara un puntero para el main donde contiene la figura 1
    // ======================================================================================
    let fn_pointer = figure_1 as *mut ();
    // ======================================================================================
    // Se asigna espacio de memoria a lo que se refiere al puntero
    // ======================================================================================
    let extern_fn_pointer= unsafe { mem::transmute::<*mut (), extern fn()>(fn_pointer) };
    // ======================================================================================
    // Se declara un puntero para el main donde contiene la figura 2
    // ======================================================================================
    let fn_pointer2 = figure_2 as *mut ();
    // ======================================================================================
    // Se asigna espacio de memoria a lo que se refiere al puntero
    // ======================================================================================
    let extern_fn_pointer2= unsafe { mem::transmute::<*mut (), extern fn()>(fn_pointer2) };

    // ======================================================================================
    // Se inicializa una creación de un pool de hilos
    // ======================================================================================
    let mut thread_pool = ThreadPool::new();
    // ======================================================================================
    // Se crea un nuevo hilo, con el planificador de RoundRobin para que trabaje la figura 1
    // con un valor de ticketes de 20
    // ======================================================================================
    unsafe { thread_pool.my_thread_create("RoundRobin".to_string(), extern_fn_pointer, config_ptr, 20, 0); }
    // ======================================================================================
    // Se crea un nuevo hilo, con el planificador de RoundRobin para que trabaje la figura 2
    // con un valor de ticketes de 20
    // ======================================================================================
    unsafe { thread_pool.my_thread_create("RoundRobin".to_string(), extern_fn_pointer2, config_ptr, 20, 0); }
    // ======================================================================================
    // Se corren los hilos, los cuales ejecutarán el movimiento de las figuras
    // ======================================================================================
    unsafe { thread_pool.run_threads(); }
    // ======================================================================================
    // Se refresca la pantalla para que se pueda ver la finalización de las figuras
    // ======================================================================================
    wrefresh(window);
    // ======================================================================================
    // Tiempo de espera para que se pueda apreciar el final de las figuras
    // ======================================================================================
    unsafe{sleep(1)};
    // ======================================================================================
    // Se finaliza la pantalla
    // ======================================================================================
    endwin();}
}
// ======================================================================================
// Esta funcion determinará todas las posiciones que debe de tener el archivo de configuración
// ======================================================================================
pub(crate) fn determinate_txt (file: Vec<String>) -> language_structure {

    // =======================================FIGURA 1=======================================

    // ======================================================================================
    // Se recorre toda la figura 1 inicial 
    // ======================================================================================
    let mut object_ascii_txt = Vec::new();
    for character in 7..12 {
        object_ascii_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se recorre toda la figura 1 inicial rotada 90 grados
    // ======================================================================================
    let mut object_ascii_90_txt = Vec::new();
    for character in 18..24 {
        object_ascii_90_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se recorre toda la figura 1 inicial rotada 120 grados
    // ======================================================================================
    let mut object_ascii_120_txt = Vec::new();
    for character in 24..30 {
        object_ascii_120_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se recorre toda la figura 1 inicial rotada 270 grados
    // ======================================================================================
    let mut object_ascii_270_txt = Vec::new();
    for character in 30..36 {
        object_ascii_270_txt.push(file[character].to_string());
    }
    // =======================================FIGURA 2=======================================

    // ======================================================================================
    // Se recorre toda la figura 2 inicial
    // ======================================================================================
    let mut figure_2_txt = Vec::new();
    for character in 37..42 {
        figure_2_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se recorre toda la figura 2 inicial rotada 90 grados
    // ======================================================================================
    let mut figure_2_90_txt = Vec::new();
    for character in 52..57 {
        figure_2_90_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se recorre toda la figura 2 inicial rotada 120 grados
    // ======================================================================================
    let mut figure_2_120_txt = Vec::new();
    for character in 46..51 {
        figure_2_120_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se recorre toda la figura 2 inicial rotada 270 grados
    // ======================================================================================
    let mut figure_2_270_txt = Vec::new();
    for character in 58..63 {
        figure_2_270_txt.push(file[character].to_string());
    }
    // ======================================================================================
    // Se inicializa la estructura del lenguaje según todos los demás datos que no son un 
    // vector de caracteres por lo cual solo se retornan su posicion dada en el archivo de configuracion
    // ======================================================================================
    let mut configuration = language_structure {
        protocol: file[0].parse::<String>().unwrap(),
        monitor_number: file[1].parse::<i32>().unwrap(),
        id_monitor: file[2].parse::<i32>().unwrap(),
        width_canvas: (file[3].parse::<i32>().unwrap()),
        height_canvas: (file[4].parse::<i32>().unwrap()),
        start_time: (file[5].parse::<String>().unwrap()),
        id_animation: (file[6].parse::<i32>().unwrap()),
        object_ascii: object_ascii_txt,
        initial_position_in_x : file[12].parse::<i32>().unwrap(),
        initial_position_in_y : file[13].parse::<i32>().unwrap(),
        final_position_in_x : file[14].parse::<i32>().unwrap(),
        final_position_in_y : file[15].parse::<i32>().unwrap(),
        angle: (file[16].parse::<i32>().unwrap()),
        final_time: (file[17].parse::<String>().unwrap()),
        object_ascii_90: object_ascii_90_txt,
        object_ascii_120: object_ascii_120_txt,
        object_ascii_270: object_ascii_270_txt,
        figure_2: figure_2_txt,
        initial_position_in_x_2 : file[42].parse::<i32>().unwrap(),
        initial_position_in_y_2 : file[43].parse::<i32>().unwrap(),
        final_position_in_x_2 : file[44].parse::<i32>().unwrap(),
        final_position_in_y_2: file[45].parse::<i32>().unwrap(),
        figure_2_90: figure_2_90_txt,
        figure_2_120: figure_2_120_txt,
        figure_2_270: figure_2_270_txt,
    };
    // ======================================================================================
    // Se retorna la configuración establecida anteriormente
    // ======================================================================================
    return configuration;
}

// ======================================================================================
// Se recorre toda la figura 2 inicial rotada 90 grados
// ======================================================================================
pub fn parser(string: String) -> Vec<String> {
    // ======================================================================================
    // Se establece el archivo de extension.txt a parsear
    // ======================================================================================
    let mut file_to_parse = File::open(string).expect("Hay que poner un archivo para que sea analizado");
    // ======================================================================================
    // Se determina su contenido
    // ======================================================================================
    let mut contents = String::new();
    // ======================================================================================
    // Se lee todo lo que contenga dicho archivo
    // ======================================================================================
    file_to_parse.read_to_string(&mut contents). expect("Que salio algo mal con el txt");
    // ======================================================================================
    // Se inicializa lineas
    // ======================================================================================
    let mut lines: Vec<String> = Vec::new();
    // ======================================================================================
    // Se recorre todas las lineas del archivo
    // ======================================================================================
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    // ======================================================================================
    // Se retornan lineas
    // ======================================================================================
    return lines;
}

// ======================================================================================
// Esta función se encargará de ejecutar todo el proceso de movimiento de la figura 1
// descrita en la configuración del archivo de extensión .txt
// ======================================================================================
unsafe fn figure_1(mut config_void: *mut c_void) {

    // ======================================================================================
    // Necesario inicializar una x y una y para ncurses, ya que estas mismas determinan el 
    // eje X y el eje Y
    // ======================================================================================
    let x = 0;
    let y = 0;
    initscr();
    noecho();
    // ======================================================================================
    // Se inicializa config de manera que este apuntando a la estructura del lenguaje
    // ======================================================================================
    let mut config: &mut language_structure = unsafe { &mut *(config_void as *mut language_structure) };
    // println!("{:?}", config);
    // let file = parser();
    // let mut config = determinate_txt(file);
    // ======================================================================================
    // Se inicializa el ancho y el alto que debe de tener el canvas, para que consecuentemente
    // se defina la ventana a trabajar para que esta misma figura se imprima
    // ======================================================================================
    let define_height_canva = config.height_canvas;
    let define_width_canva = config.width_canvas;
    // ======================================================================================
    // Se crea la ventana la cual tendrá presente las inicalizaciones anteriores para el 
    // ancho y el alto, para que de esta manera se pueda hacer referencia a terminal con newwin
    // ======================================================================================
    let window = newwin(define_height_canva, define_width_canva, y, x);
    refresh();

    // ======================================================================================
    // Se inicializan el aspecto que deberá tener el canvas, filas, columnas y esquinas
    // ======================================================================================
    let corner = chtype::from('.');
    let row = chtype::from('.');
    let column = chtype::from('.');

    // ======================================================================================
    // Se pinta los bordes del canvas dado las columnas, filas y las esquinas
    // ======================================================================================
    unsafe{wborder(window, column, column, row, row, corner, corner, corner, corner)};
    // ======================================================================================
    // Se refresca la pantalla para que muestre los cambios
    // ======================================================================================
    refresh();

    // ------------------------------------FIGURA 1------------------------------------

    // ======================================================================================
    // Se inicializa la primera figura dada una posicion inicial en el eje X y en el eje Y
    // ======================================================================================
    let initialization_y = config.initial_position_in_y; 
    let initialization_x = config.initial_position_in_x;
    // ======================================================================================
    // Imprime dicha inicializacion de la figura respectiva a las coordenadas en x y
    // ======================================================================================
    mvwprintw(window, initialization_y, initialization_x, "");

    // ======================================================================================
    // Se establece el largo del objeto ascii especificamente de la figura 1
    // ======================================================================================
    let len_ascii = config.object_ascii.len();
    // ======================================================================================
    // Se recorre caracter por caracter de la figura ascii
    // ======================================================================================
    for character in 0..len_ascii {
        // ======================================================================================
        // Se inicializa la posicion en y la cual se va a sumar el indice de caracter la cual esto
        // debe de ser por ser en el eje Y, ya que se debe de ver contemplada toda la figura
        // ======================================================================================
        let traverse_ascii_y = config.initial_position_in_y + character as i32;
        // ======================================================================================
        // Se inicializa la posicion en x la cual no tendrá cambios ya que fue previamente leida 
        // por parser
        // ======================================================================================
        let traverse_ascii_x = config.initial_position_in_x;
        // ======================================================================================
        // Se almacena el objeto de ascii
        // ======================================================================================
        let figure_ascii = &config.object_ascii[character];

        mvwprintw(window, traverse_ascii_y, traverse_ascii_x, figure_ascii);
    }
    // ======================================================================================
    // Se refresca la ventana para mostrar los cambios
    // ======================================================================================
    wrefresh(window);
    // ======================================================================================
    // Tiempo de espera para apreciar la ejecución del programa
    // ======================================================================================
    unsafe{sleep(1)};
    // ======================================================================================
    // Se determina un contado para que nos ayude con las posiciones de las figuras
    // ======================================================================================
    let counter = 1;
    // ======================================================================================
    // While true para que recorra todos lo movimientos necesarios para la figura
    // ======================================================================================
    while true {
        // ======================================================================================
        // Cuando la posicion inicial y final en X sean diferentes entra al if
        // ======================================================================================
        if config.initial_position_in_x != config.final_position_in_x {
            // ======================================================================================
            // Cuando la posicion inicial y final en Y sean diferentes entra al if
            // ======================================================================================
            if config.initial_position_in_y != config.final_position_in_y {
                // DERECHA
                // ======================================================================================
                // Cuando la posicion inicial en x es menor a la posicion final en x se debe de mover 
                // hacia la DERECHA
                // ======================================================================================
                if config.initial_position_in_x < config.final_position_in_x {
                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura rotada a 90 grados para esta posicion
                    // ======================================================================================
                    for character in 0..len_ascii {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // Para caso en el eje Y la figura solo deberá sumar el caracter para que de esta manera se 
                        // contemple toda la figura
                        // ======================================================================================
                        let move_in_y = config.initial_position_in_y + character as i32;
                        // ======================================================================================
                        // Para el movimiento en X que es el que nos interesa
                        // ======================================================================================
                        let move_in_x = config.initial_position_in_x + counter;
                        // ======================================================================================
                        // Se declara la figura movida a 90 grados para que esta misma pueda ser impresa
                        // ======================================================================================
                        let object_90 = &config.object_ascii_90[character + 1];
                        // ======================================================================================
                        // Se imprime la figura rotada a 90 grados segun sus movimientos en Y y X
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y,
                                  move_in_x,
                                  object_90);
                        get_mutex().my_mutex_unlock();
                    }
                    // ======================================================================================
                    // Cuando finalice el movimiento hacia la derecha el contador deberá ser sumado con la 
                    // posicion inicial en X para futuros movimientos
                    // ======================================================================================
                    config.initial_position_in_x += counter;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);

                } else { // IZQUIERDA
                    // ======================================================================================
                    // Cuando la posicion inicial en x es mayor a la posicion final en x se debe de mover 
                    // hacia la IZQUIERDA
                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura rotada a 90 grados para esta posicion
                    // ======================================================================================
                    for character in 0..len_ascii {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // Para caso en el eje Y la figura solo deberá sumar el caracter para que de esta manera se 
                        // contemple toda la figura
                        // ======================================================================================
                        let move_in_y = config.initial_position_in_y + character as i32;
                        // ======================================================================================
                        // Para el movimiento en X que es el que nos interesa deberá restar el contador, ya que
                        // se esta retrocediendo en el eje X
                        // ======================================================================================
                        let move_in_x = config.initial_position_in_x - counter;
                        // ======================================================================================
                        // Se declara la figura movida a 270 grados para que esta misma pueda ser impresa
                        // ======================================================================================
                        let object_270 = &config.object_ascii_270[character + 1];
                        // ======================================================================================
                        // Se imprime la figura rotada a 270 grados segun sus movimientos en Y y X
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y,
                                  move_in_x,
                                  object_270);
                        get_mutex().my_mutex_unlock();
                    }
                    // ======================================================================================
                    // Cuando finalice el movimiento hacia la derecha el contador deberá ser restado con la 
                    // posicion inicial en X para futuros movimientos
                    // ======================================================================================
                    config.initial_position_in_x -= counter;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);
                }
                // ABAJO
                // ======================================================================================
                // Para el movimiento en X que es el que nos interesa
                // ======================================================================================
                if config.initial_position_in_y < config.final_position_in_y {
                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura rotada a 120 grados para esta posicion
                    // ======================================================================================
                    for character in 0..len_ascii {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // Se recorre toda la figura por las filas y se suma el contador para que la figura tenga 
                        // su respectivo movimiento
                        // ======================================================================================
                        let move_in_y = config.initial_position_in_y + counter + character as i32;
                        // ======================================================================================
                        // Para el movimiento en X no se ve contemplado ningun moviento ya que solo se mueve en 
                        // Y en esta condicional
                        // ======================================================================================
                        let move_in_x = config.initial_position_in_x;
                        // ======================================================================================
                        // Se declara la figura movida a 120 grados para que esta misma pueda ser impresa
                        // ======================================================================================
                        let object_120 = &config.object_ascii_120[character + 1];
                        // ======================================================================================
                        // Se imprime la figura rotada a 120 grados segun sus movimientos en Y y X
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y,
                                  move_in_x,
                                  object_120);
                        get_mutex().my_mutex_unlock();
                    }
                    // ======================================================================================
                    // La modificación del contador sera la posicion inicial en Y sumada
                    // ======================================================================================
                    config.initial_position_in_y += counter;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);

                } else {
                    // ARRIBA
                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura pero en estado normal
                    // ======================================================================================
                    for character in 0..len_ascii {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // Para el movimiento en y solo se ve contemplado la suma de caracter para que recorra
                        // toda la figura
                        // ======================================================================================
                        let move_in_y = config.initial_position_in_y + character as i32;
                        // ======================================================================================
                        // En el eje X no se ve modificado ya que el movimiento en el eje Y
                        // ======================================================================================
                        let move_in_x = config.initial_position_in_x;
                        // ======================================================================================
                        // Se declara la figura movida a estado normal para que esta misma pueda ser impresa
                        // ======================================================================================
                        let object_normal = &config.object_ascii[character];
                        // ======================================================================================
                        // Se imprime la figura normal segun sus movimientos en Y y X
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y,
                                  move_in_x,
                                  object_normal);
                        get_mutex().my_mutex_unlock();     
                    }
                    // ======================================================================================
                    // Para que se inicialice el contador cuando la figura sube se debe de restar
                    // ======================================================================================
                    config.initial_position_in_y -= counter;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);
                }
            }
        }
        // ======================================================================================
        // Se defina la posicion final que vaya a tener la figura
        // ======================================================================================
        let move_final_y = config.final_position_in_y - 1;
        let move_final_x = config.final_position_in_x;
        // ======================================================================================
        // Se imprime la figura final
        // ======================================================================================
        mvwprintw(window, move_final_y, move_final_x, "");
    }
    // ======================================================================================
    // Se muestra los cambios 
    // ======================================================================================
    wrefresh(window);
    // ======================================================================================
    // Se termina la ventana de la figura 1
    // ======================================================================================
    endwin();
}

// ======================================================================================
// Esta función se encargará de ejecutar todo el proceso de movimiento de la figura 2
// descrita en la configuración del archivo de extensión .txt
// ======================================================================================
unsafe fn figure_2(mut config_void: *mut c_void) {
    // ======================================================================================
    // Necesario inicializar una x y una y para ncurses, ya que estas mismas determinan el 
    // eje X y el eje Y
    // ======================================================================================
    let x = 0;
    let y = 0;
    initscr();
    noecho();
    // ======================================================================================
    // Se inicializa config de manera que este apuntando a la estructura del lenguaje
    // ======================================================================================
    let mut config: &mut language_structure = unsafe { &mut *(config_void as *mut language_structure) };
    // let file = parser();
    // let mut config = determinate_txt(file);

    // ======================================================================================
    // Se inicializa el ancho y el alto que debe de tener el canvas, para que consecuentemente
    // se defina la ventana a trabajar para que esta misma figura se imprima
    // ======================================================================================
    let define_height_canva = config.height_canvas;
    let define_width_canva = config.width_canvas;

    // ======================================================================================
    // Se crea la ventana la cual tendrá presente las inicalizaciones anteriores para el 
    // ancho y el alto, para que de esta manera se pueda hacer referencia a terminal con newwin
    // ======================================================================================
    let window = newwin(define_height_canva, define_width_canva, y, x);
    // ======================================================================================
    // Se refresca la pantalla para que muestre los cambios
    // ======================================================================================
    refresh();

    // ======================================================================================
    // Se inicializan el aspecto que deberá tener el canvas, filas, columnas y esquinas
    // ======================================================================================
    let corner = chtype::from('.');
    let row = chtype::from('.');
    let column = chtype::from('.');

    // ======================================================================================
    // Se pinta los bordes del canvas dado las columnas, filas y las esquinas
    // ======================================================================================
    unsafe{wborder(window, column, column, row, row, corner, corner, corner, corner)};
    // ======================================================================================
    // Se refresca la pantalla para que muestre los cambios
    // ======================================================================================
    refresh();

    // ------------------------------------FIGURA 2------------------------------------

    // ======================================================================================
    // Se obtiene el largo de la figura 2
    // ======================================================================================
    let len_ascii_2 = config.figure_2.len();
    // ======================================================================================
    // Se recorre la figura 2 del archivo de extension .txt
    // ======================================================================================
    for character in 0..len_ascii_2 {
        // ======================================================================================
        // Se incicializa el eje Y para la figura 2 por eso se suma el caracter
        // ======================================================================================
        let traverse_ascii_y = config.initial_position_in_y_2 + character as i32;
        // ======================================================================================
        // Se inicializa el eje X de la figura, pero como ser parsea por filas no tiene ningun cambio
        // ======================================================================================
        let traverse_ascii_x = config.initial_position_in_x_2;
        // ======================================================================================
        // Se inicializa la figura 2 inicial
        // ======================================================================================
        let figure_ascii = &config.figure_2[character];
        // ======================================================================================
        // Se imprime la figura con las anotaciones anteriores
        // ======================================================================================
        mvwprintw(window, traverse_ascii_y, traverse_ascii_x, figure_ascii);
    }
    // ======================================================================================
    // Se actualiza la ventana para mostrar los cambios
    // ======================================================================================
    wrefresh(window);
    // ======================================================================================
    // Tiempo de espera para ver la figura inicial
    // ======================================================================================
    unsafe{sleep(1)};
    // ======================================================================================
    // Se determina un contado para que nos ayude con las posiciones de las figuras
    // ======================================================================================
    let counter_2 = 1;
    // ======================================================================================
    // While true para que recorra todos lo movimientos necesarios para la figura
    // ======================================================================================
    while true {
        // ======================================================================================
        // Cuando la posicion inicial y final en X sean diferentes entra al if
        // ======================================================================================
        if config.initial_position_in_x != config.final_position_in_x {
            // ======================================================================================
            // Cuando la posicion inicial y final en Y sean diferentes entra al if
            // ======================================================================================
            if config.initial_position_in_y != config.final_position_in_y {
                // DERECHA
                // ======================================================================================
                // Cuando la posicion inicial en x es menor a la posicion final en x se debe de mover 
                // hacia la DERECHA
                // ======================================================================================
                if config.initial_position_in_x < config.final_position_in_x {
                    // --------------- Figura 2 ---------------
                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura
                    // ======================================================================================
                    for character in 0..len_ascii_2 {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // En Y no hay modificaciones
                        // ======================================================================================
                        let move_in_y_2 = config.initial_position_in_y_2 + character as i32;
                        // ======================================================================================
                        // Se suma el contador para que sume en X
                        // ======================================================================================
                        let move_in_x_2 = config.initial_position_in_x_2 + counter_2;
                        // ======================================================================================
                        // Se toma en cuenta la figura rotada 90 grados
                        // ======================================================================================
                        let object_90 = &config.figure_2_90[character];
                        // ======================================================================================
                        // Se imprime la figura con las caracteristicas anteriores
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y_2,
                                  move_in_x_2,
                                  object_90);
                        get_mutex().my_mutex_unlock();
                    }
                    config.initial_position_in_x_2 += counter_2;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);

                } else {
                    // IZQUIERDA
                    // ======================================================================================
                    // Cuando la posicion inicial en x es mayor a la posicion final en x se debe de mover 
                    // hacia la IZQUIERDA
                    // ======================================================================================
                    // --------------- Figura 2 ---------------

                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre la figura
                    // ======================================================================================
                    for character in 0..len_ascii_2 {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // En Y no hay modificaciones
                        // ======================================================================================
                        let move_in_y_2 = config.initial_position_in_y_2 + character as i32;
                        // ======================================================================================
                        // Se resta el contador para que la figura retrocesa
                        // ======================================================================================
                        let move_in_x_2 = config.initial_position_in_x_2 - counter_2;
                        // ======================================================================================
                        // Se toma en cuenta la figura rotada 270 grados
                        // ======================================================================================
                        let object_270 = &config.figure_2_270[character];
                        // ======================================================================================
                        // Se imprime las indicaciones anteriores
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y_2,
                                  move_in_x_2,
                                  object_270);
                        get_mutex().my_mutex_unlock();
                    }
                    config.initial_position_in_x_2 -= counter_2;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);
                }
                // ABAJO
                if config.initial_position_in_y < config.final_position_in_y {

                    // --------------- Figura 2 ---------------

                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura 
                    // ======================================================================================
                    for character in 0..len_ascii_2 {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // Se suma el contador para que la figura baje
                        // ======================================================================================
                        let move_in_y_2 = config.initial_position_in_y_2 + counter_2 + character as i32;
                        // ======================================================================================
                        // No modificaciones en X, porque solo se mueve en Y
                        // ======================================================================================
                        let move_in_x_2 = config.initial_position_in_x_2;
                        // ======================================================================================
                        // Se toma en cuenta la figura rotada 120 grados
                        // ======================================================================================
                        let object_120 = &config.figure_2_120[character];
                        // ======================================================================================
                        // Se imprime la figura con las indicaciones anteriores 
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y_2,
                                  move_in_x_2,
                                  object_120);
                        get_mutex().my_mutex_unlock();
                    }
                    config.initial_position_in_y_2 += counter_2;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);

                } else {
                    // ARRIBA
                    // --------------- Figura 2 ---------------
                    
                    // ======================================================================================
                    // Se borra la figura para actualizarla con el movimiento respectivo a continuación
                    // ======================================================================================
                    unsafe{werase(window);
                    // ======================================================================================
                    // Se imprime el canva para que este durante toda la ejecución
                    // ======================================================================================
                    wborder(window, column, column, row, row, corner, corner, corner, corner)};
                    // ======================================================================================
                    // Se recorre toda la figura
                    // ======================================================================================
                    for character in 0..len_ascii_2 {
                        get_mutex().my_mutex_lock();
                        // ======================================================================================
                        // Solo se suma caracter para que se vea contamplada toda la figura
                        // ======================================================================================
                        let move_in_y_2 = config.initial_position_in_y_2 + character as i32;
                        // ======================================================================================
                        // En el eje X no hay modificaciones
                        // ======================================================================================
                        let move_in_x_2 = config.initial_position_in_x_2;
                        // ======================================================================================
                        // Se toma en cuenta la figura en estado normal
                        // ======================================================================================
                        let object_normal = &config.figure_2[character];
                        // ======================================================================================
                        // Se imprime las indicaciones anteriores
                        // ======================================================================================
                        mvwprintw(window,
                                  move_in_y_2,
                                  move_in_x_2,
                                  object_normal);
                        get_mutex().my_mutex_unlock();
                    }
                    config.initial_position_in_y_2 -= counter_2;
                    // ======================================================================================
                    // Tiempo de espera para la transición de cada movimiento
                    // ======================================================================================
                    unsafe{sleep(1)};
                    // ======================================================================================
                    // Se actualzia la figura
                    // ======================================================================================
                    wrefresh(window);
                }
            }
        }
        // ======================================================================================
        // Se defina la posicion final que vaya a tener la figura
        // ======================================================================================
        let move_final_y_2 = config.final_position_in_y_2 - 1;
        let move_final_x_2 = config.final_position_in_x_2;
        // ======================================================================================
        // Se imprime la figura final
        // ======================================================================================
        mvwprintw(window, move_final_y_2, move_final_x_2, "");
    }
    // ======================================================================================
    // Se muestra los cambios 
    // ======================================================================================
    wrefresh(window);
    // ======================================================================================
    // Se termina la ventana de la figura 1
    // ======================================================================================
    endwin();
}

unsafe fn get_mutex() -> &'static mut MutexStruct {
    match MY_MUTEX {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}