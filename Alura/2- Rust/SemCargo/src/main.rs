// VARIAVEIS X CONSTANTES
// variavel -> aloca um endereço de memoria e poe um valor
//constante -> nao aloca memoria usando só o valor, sem armazenar a constante em memoria, ou seja
//sempre que  ler o nome da variavel, em tempo de compilação, ele troca nome -> valor
// globais -> precisam de seus tipos explicitos
const PI:f32 = 3.14; // pode ser declarada fora da funcao (constante global)
static variavel_global:u8 = 1; // variavel global (tem endereço de memoria)) 
//pode ser mutavel, mas nao é seguro
static mut UNSAFE_VARIABLE:u8 = 1;

fn sombra(){
	let a = 123;
	// pode criar escopos "do nada" -> shadowing
	{
		let dentro = 13;
		println!("Dentro do Escopo, b = {}", dentro);
		let a = 321;
		println!("Dentro do Escopo,a = {}", a);

	}
	println!("Fora do escopo, a = {}" ,a);
}

fn main(){
	// tipos de variavel podem ser (u)unsingned, (i) int, (f) float, etc
	// sao colocados depois de : apos o nome da variavel
	// podem ter o tamanho de bits associados, por ex u32, f32, i16

	let variavel = 128; //por padrao, é um inteiro de 32 bits
	println!("variavel = {},tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

	let decimal:f32 = 2.5;
	println!("valor = {}, tamanho = {}", decimal, std::mem::size_of_val(&decimal));

	// erro, pois variaveis sao imutaveis em rust
	// para mudar, declarar let mut <nomev_variavel>...
	//criando assim uma variavel mutavel
	let mut booleana = false;
	booleana = true;
	println!("Booleana = {}, Tamanho booleana = {}",booleana, std::mem::size_of_val(&booleana));

	let letra:char = 'C';
	println!("Tamanho do char = {}" ,std::mem::size_of_val(&letra));

	
	println!("Valor de pi é {}", PI);

	//lidando com variavel insegura: indica que eu assumo a responsabilidade de lidar com o bloco
	unsafe {
		println!("variavel insegura = {}", UNSAFE_VARIABLE);
	}

	sombra();
	println!("Soma = {}", soma(2,2));	
	
}

//funcoes
// em rust, tudo é uma expressao, e o ; indica "ignore o resultado dessa expressao", ou seja, nao utilizarei isso depois
// se eu quiser retornar o valor, preciso NAO COLOCAR o ; ai a função é tratada como valor | posso usar return como em qualquer outra linguagem
fn soma (a:i32, b:i32) -> i32{
	println!("{} + {} = {}", a, b, a + b);
	return a + b;
}