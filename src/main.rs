
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate regex;
use regex::Regex;

fn main() {
	
	let mut identificadores = vec![];
	let mut reservadas = vec![];
	let mut numeros = vec![];
	let mut operadores = vec![];
	let mut simbolos = vec![];
	let mut errores = vec![];

	let file = File::open("fuente.txt").unwrap();
  let reader = BufReader::new(file);

  for (index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let words: Vec<&str> = line.split(" ").collect();
    for word in words {
    	/*print!("{:?}", word);*/
			if(esPalabraReservada(word.to_string())) {
				reservadas.push(word.to_string());
				/*println!("->Reservada");*/
			}
			else if(esNumero(word.to_string())) {
				numeros.push(word.to_string());
				/*println!("->Numero");*/
			}
			else if(esOperador(word.to_string())) {
				operadores.push(word.to_string());
				/*println!("->Operador");*/
			}
			else if(esIdentificador(word.to_string())) {
      	identificadores.push(word.to_string());
				/*println!("->Identificador");*/
			}
			else if(esSimbolo(word.to_string())) {
				simbolos.push(word.to_string());
				/*println!("->Simbolo");*/
			}
			else {
				errores.push(word.to_string());
				/*println!("error");*/
			}
    }
  }

  println!("_____________________________________________");
  println!("LISTA DE TOKENS");
  println!("_____________________________________________");
  println!("Identificadores->{:?}", identificadores);
  println!("Reservadas->{:?}", reservadas);
  println!("Numeros->{:?}", numeros);
  println!("Delimitadores->{:?}", simbolos);
  println!("Operadores->{:?}", operadores);
  println!("Tokens indefinidos->{:?}", errores);

}

fn esNumero(token: String) -> bool {
	let re = Regex::new(r"^(\d+|\d*\.\d+)$").unwrap();;
	if(re.is_match(&token)){
		return true;
	}
	return false;
}

fn esOperador(token: String) -> bool {
	let re = Regex::new(r"[*|/|+|-|=]$").unwrap();;
	if(re.is_match(&token)){
		return true;
	}
	return false;
}

fn esIdentificador(token: String) -> bool {
	let re = Regex::new(r"^[A-Za-z]*[%$&#]$").unwrap();;
	if(re.is_match(&token)){
		return true;
	}
	return false;
}

fn esSimbolo(token: String) -> bool {
	let re = Regex::new(r"[/|\|(|)|?|;|{|}]$").unwrap();;
	if(re.is_match(&token)){
		return true;
	}
	return false;
}

fn esPalabraReservada(token: String) -> bool {

	let palabrasReservadas = ["PRINT","LET","LIST","RUN","INPUT","GOTO","IF",
	"THEN","ELSE","STOP","END","EDIT","AUTO","WHILE","FOR","NEXT","TO","LEN", "OR"];

	for reservada in palabrasReservadas.iter() {
		if(reservada.to_lowercase() == token.to_lowercase()){
			return true;
		}
	}
	return false;
}