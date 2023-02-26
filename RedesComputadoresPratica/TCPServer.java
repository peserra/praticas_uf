package tcpsocket;

import java.io.*;
import java.net.*;

class TCPServer {

	public static void main(String args[]) throws Exception {

		var serverSocket = new ServerSocket(9000); // cria um socket na porta 9000 para conectar
		// waits for a new connection. Accepts connetion from multiple clients
		while (true) {
			System.out.println("Esperando conexao na porta 9000");
			Socket s = serverSocket.accept(); // aceita a conexão feita no seocket
			System.out.println("Conexao estabelecida de " + s.getInetAddress());

			
			// cria um  objeto BufferedReader para ler as strings do socket (que vem do client)
			var bufferedReader = new BufferedReader(new InputStreamReader(s.getInputStream()));
			var input = bufferedReader.readLine(); // adiciona o que foi lido do bufferedReader à variavel input

			
			// cria uma stream de saida para escrever (enviar) para o cliente
			var output = new DataOutputStream(s.getOutputStream());

			output.writeBytes(input.toUpperCase() + "\n"); // escreve no output stream o valor do input em caixa alta e pula uma linha
			// close current connection
			s.close();
		}
	}
}
