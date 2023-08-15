use reqwest;
use json::{self, JsonValue};
use std::io;
use std::option::Option;
use std::collections::HashMap;
fn main() -> Result<(), Box<dyn std::error::Error>>{
	const COD_JAEN : i32 = 3543;
	const _COD_LINARES :i32 = 3549;
	const _COD_TGN :i32 = 6494;
	//sacamos los json
	let url_municipios = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes/Listados/Municipios/";

	//municipios
	let result = reqwest::blocking::get(url_municipios)?.text()?;
	let json_municipios = json::parse(&result).unwrap();
	let mut municipios = HashMap::new();
	for mun in json_municipios.members() {
		let muni = get_Municipio(&mun);
		municipios.insert(muni.municipio.clone(), muni);
	}
	println!("{}",municipios.len());
	//sacamos las gasolineras
	println!("Escriba el municipio");
	let mut query = String::new();
	io::stdin().read_line(&mut query).unwrap();
	//json
	let mut url_precios = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes/EstacionesTerrestres/FiltroMunicipio/".to_string();
	let query = query.to_lowercase().trim().to_string();
	let mun = municipios.get(&query).unwrap();
	url_precios.push_str(&mun.id.to_string().as_str());
	//precios
	println!("Precios de carburantes en {}",mun.municipio);
	let mut result = reqwest::blocking::get(url_precios)?.text()?;
	//sacamos el objeto json
	result = result.replace("Precio Gasolina 95 E5", "Precio95");
	result = result.replace("Precio Gasoleo A", "PrecioGasoil");
	let json_out = json::parse(&result).unwrap();
	//precios
	let gasolineras = get_precios(&json_out).unwrap();
	let mut resultados : Vec<Gasolinera> = Vec::new();
	for gasolinera in gasolineras.members() {
		resultados.push(get_gasolinera(&gasolinera));
		println!("{:?}", resultados.last().unwrap());
	}

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
fn get_Municipio(json_val:&JsonValue) -> Municipio {
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