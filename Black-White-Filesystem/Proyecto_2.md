# Proyecto #2
**Instituto Tecnológico de Costa Rica**

**Carrera:** 
Bachillerato en Ingeniería en Computación

**Curso:** 
Principios de Sistemas Operativos

**Profesor:** 
Kevin Moraga García

**Alumnos:** 
Alberto Zumbado Abarca
Jonathan Quesada Salas

**Proyecto 2:** 
Black & White Filesystem
****
### Introducción
Principalmente con lo que respecta a este segundo proyecto de principios de
sistemas operativos consiste en desarrollar un sistema de archivos tiene como
objetivo utilizarlos imágenes blanco y negro para almacenar archivos. Para BWFS el
espacio físico donde se almacena su información y toda su estructura se encuentra
en pixels de color blanco y negro, definidos en la creación del FS.
La cual dicho filesystem se necesita hacer una reimplementación de las siguientes
funciones utilizando la biblioteca FUSE:
* getattr
* create
* open
* read
* write
* rename
* mkdir
* readdir
* opendir
* rmdir
* statfs
* fsync
* access
* unlink
* flush
* lseek


Adicionalmente en este proyecto deberá estar presente mkfs.bwfs, el cual este es
un binario que consiste en la creación de un nuevo sistema de archivos tipo BWFS.
Obviamente teniendo en cuenta aspectos generales como:

* Cada bloque de BWFS podrá tener un máximo de 1000 px por 1000 px.
* Se le solicitará al usuario introducir un nuevo “passphrase”.
* Toda la información relevante a la organización del FS
* El sistema de archivos deberá de utilizar i-nodos como estructura de
indexación de bloques.
* El sistema de archivos deberá permitir una estructura jerárquica de
directorios.

Por otra parte se encuentra fsck.bwfs, el cual se encarga de realizar un chequeo de consistencia de BWFS.

Y por último se encuentra mount.bwfs este se encargará de montar el BWFS en
algún punto de montaje perteneciente al FS del Sistema Operativo.


***
### Ambiente de desarrollo
Con lo que respecta al ambiente de desarrollo se usará el sistema operativo Ubuntu
por parte de Alberto Zumbado Abarca y Jonathan Quesada Salas, específicamente
la versión:
~~~
Ubuntu 20.04.2 LTS
~~~
En cuanto a los IDLE se que usarán serán el Visual Studio Code para el desarrollo
de la tarea y adicionalmente Eclipse para la funcionalidad del debugger que tiene
dicho IDLE para el lenguaje de programación Rust.

Se usará el lenguaje de programación Rust para la resolución de dicha asignación
de proyecto número uno de principios de sistemas operativos.
Adicionalmente se contará con un repositorio en Github el cual contendrá todo los entregables requeridos en este proyecto, el link del repositorio es el siguiente:
~~~
https://github.com/Nuwidra/IC-6600_Black-White-Filesystem_Proyecto-2
~~~

***
### Estructuras de datos usadas y funciones


### Instrucciones para ejecutar el programa
1. Primeramente ingrese a la carpeta del proyecto
2. Ingrese el siguiente comando para la ejecución del programa animar:
    ~~~
    ./animar -c config.txt 
    ~~~


***


### Actividades realizadas por estudiante
**ALBERTO ZUMBADO ABARCA**


|Fecha|Inicio|Fin|Avance Realizado|
|---|------|---|---| 
|||||

**JONATHAN QUESADA SALAS**


|Fecha|Inicio|Fin|Avance Realizado|
|---|------|---|---| 
|||||

***


### Autoevaluación

#### Estado final
Se vieron comtemplado la totalidad de los requerimientos obligatorios de este proyecto.

#### Problemas y limitaciones

#### Reporte de commits

~~~


~~~


#### Calificación
|Rubro|Porcentaje|
|-----|----------|
|mkfs.bwfs|14%|
|fsck.bwfs|5%|
|mount.bwfs|10%|
|Funciones de la biblioteca|26%|
|Documentación utilizando Markdown o Latex-PDF|20%|
|Persistencia en Disco|25%|
|Otros opcionales|5%|
|Kick-off|5%|
|TOTAL|110%|


#### Autoevaluación
**Auto-Evaluación de Jonathan**


|Rubro|Escala|
|-----|----------|
|Aprendizaje de mkfs|5|
|Aprendizaje de fsck|5|
|Aprendizaje de mount|5|
|Aprendizaje de implementacion de funciones|5|
|Aprendizaje de Diseño de Filesystem|5|


**Auto-Evaluación de Alberto**


|Rubro|Escala|
|-----|----------|
|Aprendizaje de mkfs|5|
|Aprendizaje de fsck|5|
|Aprendizaje de mount|5|
|Aprendizaje de implementacion de funciones|5|
|Aprendizaje de Diseño de Filesystem|5|

***


### Lecciones Aprendidas
Primeramente se puede destacar el conocimiento que se obtuvo sobre ¿Qué es un sistema de archivos? Es cual su funcionamiento se puede describir de manera de una base de datos indexada en donde se encuentra la ubicación física de cada archivo adentro del disco duro de la computadora o cualquier otro dispositivo de almacenamiento, como pueden ser memorias USB, teléfonos inteligentes, un servidor de red e incluso hasta cajeros automáticos y el ordenador del auto.

Por regla general, sea cual sea el dispositivo de almacenamiento, la información guardada en él se organiza en directorios o carpetas, mismas que pueden contener subcarpetas y otros archivos.

Por otra parte una leccón importante para futuros estudiantes que puedan llegar a cursar este curso en un futuro es que empiecen a investigar desde el principio que se asigna el proyecto, ya que términos como pueden ser i-nodo pueden llegar a ser confusos, por otra parte algo que se aprendió a profundidad fue este mismo concepto el cual se puede describir como las características de un archivo regular, directorio, o cualquier otro objeto que pueda contener el sistema de ficheros.

De igual manera conceptos como la consistencia y el montaje se vieron aclarados en este proyecto, ya que la consistencia se determina como que cada bloque o está libre o está en uso sólo una vez. Por otra parte con lo que se refiere a montaje se puede ver como que cuando uno tiene su sistema de archivos de su computadora, este proyecto tiene que "ponerse encima" en el espacio de usuario y que este mismo pueda tener un mismo comportamiento de un sistemas de archivos, en cuanto a las funcionalidades que puede verse contemplada está asignación.
***


### Bibliografía



