//Strings
//Para gestao de memoria
fn ownership(){
    // Cada valor em Rust possui uma variavel que é chamada seu owner
    // Só pode existir um owner por vez para um valor
    // quando o owner sai do escopo, o valor é descartado da heap.

    let uma_string = String::from("vinicius"); // texto a partir de um stack str
    //variavel uma_string nao possui a string, mas um ponteiro para um endereço de memoria (na heap) q contem a string "vinicius"

    //strings dinamicas sao alocadas na heap

    // ownership é um conceito diferente de otimização de memória
    // cada valor na heap só pode ter 'um dono'
    // quando o valor é usado (por uma função por exemplo), a variavel que continha esse valor deixa de ser dona do valor, deixando de existir
    // no caso de uma utilização em função, se recebemos o valor como parametro, o owner do valor passa ser a variavel do parametro.
    rouba(uma_string);

    //portanto, a linha a seguir dara um erro, visto que  nao existe valor alocado na posição da variavel uma_string na heap
    // se eu ainda assim quiser usar o valor, posso usar o metodo .clone
   
    // soluções: Posso retornar uma string na funcao e utilizar o retorno, ou  emprestar o valor passando o endereço de memoria como parametro
    // caso eu queira mudar o valor dessa referencia, posso mandar uma referencia mutavel, ou seja:
    rouba2(& mut uma_string); // para isso porem, a string enviada tem que ser declarada como mutavel também
    println!("{}", uma_string);
    
}

//sendo assim, quando a função acaba, e o parametro deixa de existir, o valor some da heap
fn rouba(string:String){
    println!("{}", string)
}

fn rouba2(string: & mut String){
    string.push_str("JOAO");
    println!("{}", string)
}




fn main(){
    //let contador:u8 = 0;

    for contador in 1..11{
        println!("iteracao numero {}", contador);
    }
    //ownership();
    pattern_matching();
}

fn pattern_matching(){
    // se eu tiver um valor que queira casar com algum tipo de expressao ou condição, posso usar a sintaxe abaixo
    for x in 1..21{
        println!("{}:{}", x, match x {1 => "Pouco",2|3 => "Um pouquinho",4..=10 => "Um bocado", _ if x % 2 == 0 => "Uma boa quantidade" , _=>"Muito"});
    }
}


// LIDANDO COM ERROS
/*
rust nao usa excessoes
os erros sao irrecuperaveis

usamos a macro panic! para reportar o erro
caso queiramos usar um erro recuperavel utilizamos  result
*/

fn erros(){
    match resultado (){
        Ok(s) => println!("String de sucesso: {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero)
    }
}

fn resultado () -> Result <String, u8> {
    Ok(String::from("Tudo deu certo"));
}