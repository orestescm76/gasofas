use reqwest;
use json::{self, JsonValue};
use std::io;
use std::option::Option;
use std::collections::HashMap;
use std::fs::{self, exists};
use std::env;

fn save_initial_file(url: &str) {
	let req = reqwest::blocking::get(url.to_string() + "/Listados/Municipios/");
	let out = req.unwrap().text().unwrap();
	let _ = fs::write("cities.json", out);
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
	//sacamos los json
	let url_base = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes";
	match exists("cities.json") {
		Ok(false) => save_initial_file(url_base),
		Ok(true) => (),
		Err(_) => (),
	}
	//municipios
	let file = std::fs::read_to_string("cities.json")?;
	let json_municipios = json::parse(&file).unwrap();
	let mut municipios = HashMap::new();
	for mun in json_municipios.members() {
		let muni = get_municipio(&mun);
	 	municipios.insert(muni.municipio.clone(), muni);
	}
	let mut city = String::new();
	//sacamos las gasolineras
	let args : Vec<String> = env::args().collect();
	let mut iter_args = args.iter().skip(1);
	while let Some(arg) = iter_args.next() {
		println!("{}",arg.to_string());
		match arg.as_str() {
			"-c" => {
				if let Some(val) = iter_args.next() {
					city = val.to_string();
				} else {
					println!("-c requiere un valor, como una ciudad");
				}
			}
			"-a" | "--ayuda" => {println!("uso: carburantes [OPCIÓN] [ARGUMENTO]\n\
				opciones:\n\
				carburantes {{-a --ayuda}}\n\
				carburantes {{-c [ciudad]}}");
			return Ok(())
			}
			"" => {
				println!("debes escribir argumentos, -a o --ayuda para imprimir ayuda");
			}
			_ => {
				println!("debes escribir argumentos, -a o --ayuda para imprimir ayuda");
			}
		}
	}
	println!("{}", city);
	// let mut query = String::new();
	// io::stdin().read_line(&mut query).unwrap();
	// //json
	// let mut url_precios = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes/EstacionesTerrestres/FiltroMunicipio/".to_string();
	// let query = query.to_lowercase().trim().to_string();
	// let mun = municipios.get(&query).unwrap();
	// url_precios.push_str(&mun.id.to_string().as_str());
	// //precios
	// println!("Precios de carburantes en {}",mun.municipio);
	// let mut result = reqwest::blocking::get(url_precios)?.text()?;
	// //sacamos el objeto json
	// result = result.replace("Precio Gasolina 95 E5", "Precio95");
	// result = result.replace("Precio Gasoleo A", "PrecioGasoil");
	// let json_out = json::parse(&result).unwrap();
	// //precios
	// let gasolineras = get_precios(&json_out).unwrap();
	// let mut resultados : Vec<Gasolinera> = Vec::new();
	// for gasolinera in gasolineras.members() {
	// 	resultados.push(get_gasolinera(&gasolinera));
	// 	println!("{:?}", resultados.last().unwrap());
	// }

	Ok(())
	
}
// resultados.push(get_gasolinera(&resultados));
// println!("{:?}", get_gasolinera(&gasolinera));
fn get_precios(raiz : &JsonValue) -> Option<JsonValue> {
	let mut lista_precios=None;
	for val in raiz.entries() {
		if val.0 == "ListaEESSPrecio" {
			lista_precios = Some(val.1.clone());
		}
	}

	lista_precios
}

fn get_gasolinera(json_val : &JsonValue) -> Gasolinera {
	let mut gas = Gasolinera {

		direccion : "".to_string(),
		horario : "".to_string(),
		nombre : "".to_string(),
		precio_gasoil : 0.0,
		precio_gasolina : 0.0,
	};

	for entry in json_val.entries() {
		match entry.0 {
			"Dirección" => gas.direccion = entry.1.as_str().unwrap().to_string(),
			"Horario" => gas.horario = entry.1.as_str().unwrap().to_string(),
			"PrecioGasoil" => gas.precio_gasoil = entry.1.as_str().unwrap().replace(",",".").parse::<f32>().unwrap_or(0.0),
			"Precio95" => gas.precio_gasolina = entry.1.as_str().unwrap().replace(",",".").parse::<f32>().unwrap_or(0.0),
			"Rótulo" => gas.nombre = entry.1.as_str().unwrap().to_string(),
			_ => ()
		}
	}
	gas
}
fn get_municipio(json_val:&JsonValue) -> Municipio {
	let mut mun = Municipio {
		ccaa : "".to_string(),
		municipio : "".to_string(),
		provincia : "".to_string(),
		id:0
	};
	for entry in json_val.entries() {
		match entry.0 {
			"CCAA" => mun.ccaa = entry.1.as_str().unwrap().to_string(),
			"Municipio" => mun.municipio = entry.1.as_str().unwrap().to_string().to_lowercase(),
			"Provincia" => mun.provincia = entry.1.as_str().unwrap().to_string(),
			"IDMunicipio" => mun.id = entry.1.as_str().unwrap().parse::<i32>().unwrap_or(0),
			_ => ()
		}
	}
	mun
}
#[derive(Debug)]
struct Gasolinera {
	nombre : String,
	precio_gasolina : f32,
	precio_gasoil: f32, 
	direccion: String,
	horario: String,
}
#[derive(Debug)]
struct Municipio {
	ccaa:String,
	municipio:String,
	provincia:String,
	id:i32,
}
