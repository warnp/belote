fn creer_des_cartes() -> Vec<String>{
    let mut resultat = vec![];

    for i in 0..32 {

        match i {
            0 ... 8 => resultat.push("carreaux".to_string()),
            9 ... 16 => resultat.push("trefle".to_string()),
            17 ... 24 => resultat.push("pique".to_string()),
            25 ... 32 => resultat.push("coeur".to_string()),
            _ => (),
        }
    }

    resultat
}

fn main() {
    let jeu = creer_des_cartes();

    for i in jeu {
        println!("{:#?}", i);
    }
}
