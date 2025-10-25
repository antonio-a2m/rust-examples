use std::io;

// Trait para cualquier tipo de empleado
trait Empleado {
    fn salario_mensual(&self) -> f64;
    fn nombre(&self) -> String;
    fn tipo_empleado(&self) -> String;

    // Métodos con implementación por defecto
    fn salario_anual(&self) -> f64 {
        self.salario_mensual() * 12.0
    }

    fn aguinaldo(&self) -> f64 {
        self.salario_mensual() * 0.5
    }

    fn total_anual(&self) -> f64 {
        self.salario_anual() + self.aguinaldo()
    }

    fn mostrar_resumen(&self) {
        println!("\n{}", "=".repeat(50));
        println!("        RESUMEN DE SALARIO - {}", self.tipo_empleado().to_uppercase());
        println!("{}", "=".repeat(50));
        println!("Nombre:           {}", self.nombre());
        println!("Tipo:             {}", self.tipo_empleado());
        println!("Salario mensual:  {}", formatear_dinero(self.salario_mensual()));
        println!("Salario anual:    {}", formatear_dinero(self.salario_anual()));
        println!("Aguinaldo:        {}", formatear_dinero(self.aguinaldo()));
        println!("{}", "-".repeat(50));
        println!("TOTAL ANUAL:      {}", formatear_dinero(self.total_anual()));
        println!("{}", "=".repeat(50));
    }
}

// Empleado por horas
struct EmpleadoPorHoras {
    nombre: String,
    tarifa_por_hora: f64,
    horas_por_mes: f64,
}

impl Empleado for EmpleadoPorHoras {
    fn salario_mensual(&self) -> f64 {
        self.tarifa_por_hora * self.horas_por_mes
    }

    fn nombre(&self) -> String {
        self.nombre.clone()
    }

    fn tipo_empleado(&self) -> String {
        "Empleado por Horas".to_string()
    }
}

impl EmpleadoPorHoras {
    fn new(nombre: String, tarifa_por_hora: f64, horas_por_mes: f64) -> Self {
        EmpleadoPorHoras {
            nombre,
            tarifa_por_hora,
            horas_por_mes,
        }
    }
}

// Empleado asalariado
struct EmpleadoAsalariado {
    nombre: String,
    salario: f64,
}

impl Empleado for EmpleadoAsalariado {
    fn salario_mensual(&self) -> f64 {
        self.salario
    }

    fn nombre(&self) -> String {
        self.nombre.clone()
    }

    fn tipo_empleado(&self) -> String {
        "Empleado Asalariado".to_string()
    }
}

impl EmpleadoAsalariado {
    fn new(nombre: String, salario: f64) -> Self {
        EmpleadoAsalariado { nombre, salario }
    }
}

// Empleado con comisiones
struct EmpleadoConComision {
    nombre: String,
    salario_base: f64,
    ventas_mensuales: f64,
    porcentaje_comision: f64,
}

impl Empleado for EmpleadoConComision {
    fn salario_mensual(&self) -> f64 {
        self.salario_base + (self.ventas_mensuales * self.porcentaje_comision / 100.0)
    }

    fn nombre(&self) -> String {
        self.nombre.clone()
    }

    fn tipo_empleado(&self) -> String {
        "Empleado con Comisión".to_string()
    }
}

impl EmpleadoConComision {
    fn new(nombre: String, salario_base: f64, ventas_mensuales: f64, porcentaje_comision: f64) -> Self {
        EmpleadoConComision {
            nombre,
            salario_base,
            ventas_mensuales,
            porcentaje_comision,
        }
    }
}

// Función auxiliar para formatear dinero
fn formatear_dinero(amount: f64) -> String {
    let formatted = format!("{:.2}", amount);
    let parts: Vec<&str> = formatted.split('.').collect();
    let entero = parts[0];
    let decimal = parts[1];

    let mut resultado = String::new();
    let chars: Vec<char> = entero.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            resultado.push(',');
        }
        resultado.push(*c);
    }

    format!("${}.{}", resultado, decimal)
}

fn main() {
    println!("=== SISTEMA DE NÓMINA ===\n");

    // Vector de empleados usando trait objects
    let mut empleados: Vec<Box<dyn Empleado>> = Vec::new();

    loop {
        println!("\n¿Qué tipo de empleado deseas agregar?");
        println!("1. Empleado Asalariado");
        println!("2. Empleado por Horas");
        println!("3. Empleado con Comisión");
        println!("4. Mostrar resumen de todos los empleados");
        println!("5. Salir");
        println!("Elige una opción (1-5):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer");

        match input.trim() {
            "1" => {
                let nombre = obtener_string("Ingresa el nombre del empleado:");
                let salario = obtener_numero("Ingresa el salario mensual:");
                empleados.push(Box::new(EmpleadoAsalariado::new(nombre, salario)));
                println!("Empleado asalariado agregado exitosamente!");
            }
            "2" => {
                let nombre = obtener_string("Ingresa el nombre del empleado:");
                let tarifa = obtener_numero("Ingresa la tarifa por hora:");
                let horas = obtener_numero("Ingresa las horas trabajadas por mes:");
                empleados.push(Box::new(EmpleadoPorHoras::new(nombre, tarifa, horas)));
                println!("Empleado por horas agregado exitosamente!");
            }
            "3" => {
                let nombre = obtener_string("Ingresa el nombre del empleado:");
                let salario_base = obtener_numero("Ingresa el salario base mensual:");
                let ventas = obtener_numero("Ingresa las ventas mensuales:");
                let comision = obtener_numero("Ingresa el porcentaje de comisión:");
                empleados.push(Box::new(EmpleadoConComision::new(nombre, salario_base, ventas, comision)));
                println!("Empleado con comisión agregado exitosamente!");
            }
            "4" => {
                if empleados.is_empty() {
                    println!("No hay empleados registrados.");
                } else {
                    mostrar_resumen_general(&empleados);
                }
            }
            "5" => {
                println!("¡Hasta luego!");
                break;
            }
            _ => println!("Opción inválida. Intenta de nuevo."),
        }
    }
}

fn obtener_string(mensaje: &str) -> String {
    loop {
        println!("{}", mensaje);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer");
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
        println!("El nombre no puede estar vacío. Intenta de nuevo.");
    }
}

fn obtener_numero(mensaje: &str) -> f64 {
    loop {
        println!("{}", mensaje);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer");

        match input.trim().parse::<f64>() {
            Ok(numero) if numero >= 0.0 => return numero,
            _ => println!("Ingresa un número válido mayor o igual a 0\n"),
        }
    }
}

fn mostrar_resumen_general(empleados: &[Box<dyn Empleado>]) {
    println!("\n{}", "=".repeat(60));
    println!("                  RESUMEN GENERAL DE NÓMINA");
    println!("{}", "=".repeat(60));

    let mut total_nomina = 0.0;

    for empleado in empleados {
        empleado.mostrar_resumen();
        total_nomina += empleado.total_anual();
    }

    println!("\n{}", "=".repeat(60));
    println!("TOTAL NÓMINA ANUAL: {}", formatear_dinero(total_nomina));
    println!("{}", "=".repeat(60));
}