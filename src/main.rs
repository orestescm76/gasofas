use reqwest;
use json::{self, JsonValue};
use std::option::Option;
fn main() -> Result<(), Box<dyn std::error::Error>>{
	const COD_JAEN : i32 = 3543;
	const COD_LINARES :i32 = 3549;
	const COD_TGN :i32 = 6494;
	//sacamos el json
	let mut url = "https://sedeaplicaciones.minetur.gob.es/ServiciosRESTCarburantes/PreciosCarburantes/EstacionesTerrestres/FiltroMunicipio/".to_string();
	url.push_str(&COD_LINARES.to_string());
	let mut result = reqwest::blocking::get(url)?.text()?;
	//sacamos el objeto json
	result = result.replace("Precio Gasolina 95 E5", "Precio95");
	result = result.replace("Precio Gasoleo A", "PrecioGasoil");
	let json_out = json::parse(&result).unwrap();
	//sacamos las gasolineras
	let gasolineras = get_precios(&json_out).unwrap();
	println!("{}", gasolineras.members().len());
	let mut resultados : Vec<Gasolinera> = Vec::new();
	for gasolinera in gasolineras.members() {
		resultados.push(get_gasolinera(&gasolinera));
		println!("{:?}", resultados.last().unwrap());
	}
	let mut buf = String::new();
	std::io::stdin().read_line(&mut buf).unwrap();
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
fn get_gasolinera(jsonVal : &JsonValue) -> Gasolinera {
	let mut gas = Gasolinera {

		direccion : "".to_string(),
		horario : "".to_string(),
		nombre : "".to_string(),
		precio_gasoil : 0.0,
		precio_gasolina : 0.0,
	};

	for entry in jsonVal.entries() {
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
#[derive(Debug)]
struct Gasolinera {
	nombre : String,
	precio_gasolina : f32,
	precio_gasoil: f32, 
	direccion: String,
	horario: String,
}