use std::collections::HashMap;
use sha2::{Digest, Sha256};

// Definición de la estructura Empleado
struct Empleado {
    nombre: String,
    numero_empleado: String, // Se almacena el número de empleado en formato encriptado
    departamento: String,
}

impl Empleado {
    // Constructor de la estructura Empleado
    fn new(nombre: &str, numero_empleado: &str, departamento: &str) -> Empleado {
        Empleado {
            nombre: nombre.to_string(),
            // Encriptar el número de empleado antes de almacenarlo
            numero_empleado: encriptar_sha256(numero_empleado),
            departamento: departamento.to_string(),
        }
    }
}

// Función para encriptar con SHA-256
fn encriptar_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

// Definición de la estructura de la tabla hash para almacenar empleados
struct GestorEmpleados {
    empleados: HashMap<String, Empleado>,
}

impl GestorEmpleados {
    // Constructor de la estructura GestorEmpleados
    fn new() -> GestorEmpleados {
        GestorEmpleados {
            empleados: HashMap::new(),
        }
    }

    // Función para agregar un empleado a la tabla hash
    fn agregar_empleado(&mut self, empleado: Empleado) {
        self.empleados.insert(empleado.numero_empleado.clone(), empleado);
    }

    // Función para buscar un empleado por su número de empleado
    fn buscar_empleado(&self, numero_empleado: &str) -> Option<&Empleado> {
        self.empleados.get(numero_empleado)
    }

    // Función para mostrar la información de todos los empleados
    fn mostrar_empleados(&self) {
        for (_, empleado) in &self.empleados {
            println!("Nombre: {}, Número de Empleado: {}, Departamento: {}", empleado.nombre, empleado.numero_empleado, empleado.departamento);
        }
    }
}

fn main() {
    // Crear una instancia del gestor de empleados
    let mut gestor_empleados = GestorEmpleados::new();

    // Agregar empleados al sistema
    gestor_empleados.agregar_empleado(Empleado::new("Juan", "101", "Ventas"));
    gestor_empleados.agregar_empleado(Empleado::new("María", "102", "Finanzas"));
    gestor_empleados.agregar_empleado(Empleado::new("Pedro", "103", "TI"));

    // Encriptar el número de empleado a buscar
    let numero_empleado_a_buscar = encriptar_sha256("102");

    // Buscar la información del empleado con número de empleado 102 en su formato encriptado
    if let Some(empleado) = gestor_empleados.buscar_empleado(&numero_empleado_a_buscar) {
        println!("Información del Empleado:");
        println!("Nombre: {}, Número de Empleado: {}, Departamento: {}", empleado.nombre, empleado.numero_empleado, empleado.departamento);
    } else {
        println!("Empleado no encontrado.");
    }

    // Mostrar la información de todos los empleados
    println!("Información de todos los empleados:");
    gestor_empleados.mostrar_empleados();
}
