package tcpsocket;

import java.io.*;
import java.net.*;

public class TCPClient {

	public static void main(String args[]) throws Exception {

		
		// conecta ao processo do server rodando no host
		var socket = new Socket("127.0.0.1", 9000);

		
		// proximas duas linhas criam uma stream de saida que podemos escrever para o server
		OutputStream outputStream = socket.getOutputStream();
		var serverWriter = new DataOutputStream(outputStream);


		// proximas duas linahs criam um buffer de leitura que le da standard input (teclado) para ler a stream do server
		var isrServer = new InputStreamReader(socket.getInputStream());
		var serverReader = new BufferedReader(isrServer);

		// cria um buffered reader para ler o input do usuario e adiciona -o à variavel sentence
		var inputFromUser = new BufferedReader(new InputStreamReader(System.in));
		String sentence = inputFromUser.readLine();

		// manda o input do usuario ao server
		serverWriter.writeBytes(sentence + "\n");


		// le a responsta do server e printa na tela
		String response = serverReader.readLine();
		System.out.println(response); 		// Server tem que converter para caixa alta e responder
		socket.close();
	}
}
