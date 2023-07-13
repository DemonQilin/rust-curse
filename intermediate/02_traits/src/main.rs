/*
    TRAITS

    Los traits son la forma en que se definen funcionalidades compartidas en tipos. Se trata de definir comportamiento de una forma abstracta, similar a las interfaces en otros lenguajes.

    Para crear un trait se usa la palabra reservada "trait" seguida del nombre que se le va a dar y dentro de llaves "{}" se declaran la firma de los metodos que agrupa donde:

        - Pueden terminar en punto y coma ";", obligando al tipo que implementa el trait a definir el comportamiento
        - Tener una implementación, que se conoce como Implementación Por Defecto, y cualquier tipo que implemente el Trait tendra esta implementación a menos que la sobreescriba con su propia definición.

    Los Traits solo permiten compartir/abstraer funcionalidad, es decir, métodos.

    IMPLEMENTACIÓN

    Implementar un Trait es muy similar a implementar metodos normales, la diferencia es que despues de "impl" se pone el nombre del Trait seguido de la palabra reservada "for" y finalmente el nombre del tipo al que se le va hacer la implementación. Luego dentro de llaves usando las firmas de los metodos del Trait definimos los comportamientos que se requieran y/o se quieran sobreescribir.

    Los metodos de un Trait pueden ser invocados de la misma forma que los normales, pero antes se debe traer el Trair al scope.


    TRAITS EN PARAMETROS
    Los Traits se pueden usar para definir funciones que utilizan y retornan distintos tipos que tiene comportamientos en comun, es decir, utilizar tipos asegurando que se implementen cierto(s) Trait(s).

    Existen tres formas para indicar esto en la firma de la función:
        - Trait Bound: se declara un tipo generico seguido de dos puntos ":" y el nombre del Trait que se requiere implemente el tipo
        - Shorthand Trait Bound: El Trait Bound tiene una forma corta que no utiliza genericos y se logra dando como tipo al parametro "impl <TraitName>".
        - Where clause: Permite definir lo trait despues la firma de la función, se logra usando la palabra reservada "where" y cada generico que necesite Traits separado por coma.

    Se pueden especificar multiples Traits Bound con la sintaxis "+".

    TRAITS EN RETORNOS
    Se puede usar la sintaxis "impl <TraitName>" para indicar que el retorno de una función implementa cierto(s) Trait(s) pero sin que el codigo que llama conozca el tipo concreto que se retorna.

    Es importante tener en cuenta que la función debe unicamente retornar un tipo, es decir, no puedo tener un condicional que retorne un tipo o el otro a pesar de que implementen el trait.
*/

// Definimos un trait con el proposito de que cualquier que lo implemente tenga la capacidad de parquear
trait Park {
    fn park(&self);
}

// Definimos un trait que busca dar la funcionalidad de pintar y ademas tiene una implementación por defecto, que mas adelante los tipos que lo implementen podran elegir sobreescribir
trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {color}");
    }
}

// Para abstraer campos e información usamos un struct porque los Trait solo pueden abstraer metodos
struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

// Implementamos el Trait Park y definimos el comportamiento deseado al metodo "park"
impl Park for Car {
    fn park(&self) {
        print!("Parking car");
    }
}

// Implementa el trait Paint y utilizara el comportamiento por defecto
impl Paint for Car {}

struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unload(&self) {
        print!("unloading truck");
    }
}

// Al igual que Car implementa pintar con su comportamiento por defecto
impl Paint for Truck {}

struct House {}

// Implementa el Trait de pintar y define su propia forma de hacerlo
impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting House with color: {color}");
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Ford".to_owned(),
            model: "Fiesta".to_owned(),
            year: 1980,
        },
    };
    let house = House {};
    // El codigo cliente no conoce el tipo concreto pero sabe que puede usar todos los metodos del Trati Paint
    let paintable_object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&paintable_object);

    // Solo el tipo Car implementa los trait Paint y Park
    paint_vehicle_red(&car);
    // paint_vehicle_red(&house);
    // paint_vehicle_red(&paintable_object);
}

// Define una función que recibe cualquier tipo que tenga la funcionalidad de pintarse usando la sintaxis Trait Bound
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

// Define una función que recibe cualquier tipo que implemente Paint usando la azucar sintactica de Trait Bound
// fn paint_red2(object: &impl Paint) {
//     object.paint("red".to_owned());
// }

// Define la funcionalidad de pintar de rojo cualquier tipo pueda ser pintando y parqueado la clausula "where".
fn paint_vehicle_red<T>(object: &T)
where
    T: Paint + Park,
{
    object.paint("red".to_owned())
}

// Define una función que crea objetos que pueden ser pintados
fn create_paintable_object() -> impl Paint {
    House {}
}
