use std::fs::File;
use std::io::{BufReader, stdin};
use std::path::Path;

fn main() {

    let  tamanho = 12;
    let mut soma = 0;
    let mut contador = 0;
    let mut linha = 1;
    let mut coluna = 11;
    let mut o: String = String::new();
    let path = Path::new("./data/matriz.json");
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let matriz: Vec<Vec<i32>> = serde_json::from_reader(reader).unwrap();
    let mut desenho: [[char; 12]; 12] = [['.'; 12]; 12];
    println!("Matriz: {:?}", matriz);


    println!("M = Média | S = Soma");
    println!("Digite a operação desejada: ");
    stdin().read_line(&mut o).unwrap();

    let o = o.trim();
    let mut conta=0;
    let mut conta2=1;
    let mut resto=0;
    let mut coluna_minha=0;
    
    while linha < (tamanho-1)  {
    println!("linha: {}, Coluna {}", linha, coluna);
    coluna_minha=coluna;    
    resto=12 - coluna_minha;
    conta=0;
    

    while conta<resto{
        soma += matriz[linha][coluna_minha];
        desenho[linha][coluna_minha] = 'X';
        contador += 1;
        conta +=1;
        coluna_minha+=1;
    }
        linha += 1;
        conta2 +=1;
        match linha {
            1..=5 => coluna -= 1,
            6=> coluna += 0,
            _ => coluna += 1
        }
    }
    
    if o == "M" || o == "m" {
        println!("Média: {}", soma / contador);
    } else {
        println!("Soma: {}", soma);
    }
    for linha in desenho.iter() {
        for &caractere in linha.iter() {
            print!("{} ", caractere);
        }
        println!();
    }
}
