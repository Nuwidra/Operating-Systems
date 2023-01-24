use crate::arguments_summary::ArgumentsSummary;
// ======================================================================================
// t_argument es una interfaz que solo define 2 métodos get_text y summarize_argument
//      get_text: Retornará un string de los argumentos
//      summarize_argument: Hace un resumen a sí mismo en el objeto summarize_argument con los argumentos
// ======================================================================================
pub trait TArgument {
    fn get_text(&self) -> &String;
    fn summarize_argument(&self, summary:&mut ArgumentsSummary);
}