fn main() {
    condicionais();
    repeticoes();
}

fn condicionais(){
    let idade:u8 = 16;
    let autorizacao = true;
    if idade >= 18 {
        println!("Pode entrar na balada!");
    }
    else if idade > 16 && autorizacao {
        print!("Pode entrar com assinatura ");
    }
    else {
        print!("Nao pode, sinto muito ");
    }

    // rust trata tudo como expressao, portanto:

    let condicao = if idade > 18 {"maior"} else {"menor"};
    println!("É {} de idade", condicao);
}

fn repeticoes(){
    let multiplicador:u8 = 5;
    let mut contador:u8 = 1;
    while contador < 11 {
        //println!("{} x {} = {}", contador, multiplicador, contador * multiplicador);
        contador += 1;
    }
    
    // loop infinito:
    contador = 1;
    loop{
        println!("{} x {} = {}", contador, multiplicador, contador * multiplicador);
        contador += 1;
        if contador == 5 {
            continue;
        }
        else if contador == 11 {
            break;            
        }
    }

    //for loop range é 1-inclusivo e 11 exclusivo
    for i in 1..11 {
        println!("{} x {} = {}", i, multiplicador, i * multiplicador);
    }

    //match statement é tipo um case switch
    let linguagem = "";
    let proposito = match linguagem {
        "PHP" => "web",
        "Python" => "Data Science",
        _=>"Desconhecido" //valor default
    }

}