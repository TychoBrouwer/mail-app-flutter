import 'package:web_socket_client/web_socket_client.dart';

class WebsocketService {
  late WebSocket _socket;

  bool lock = false;

  Future<void> connect() async {
    // Connect to the websocket server
    final uri = Uri.parse('ws://localhost:9001');
    const backoff = ConstantBackoff(Duration(seconds: 1));

    _socket = WebSocket(uri, backoff: backoff);

    await Future.doWhile(() async {
      await Future.delayed(const Duration(milliseconds: 100));

      return _socket.connection.state is! Connected;
    });

    return;
  }

  // Disconnect from the websocket server
  void disconnect() {
    // Disconnect from the websocket server
    _socket.close();
  }

  // Send a message to the websocket server
  Future<String> sendMessage(String message) async {
    lock = true;

    _socket.send(message);

    await for (final String message in _socket.messages) {
      return message;
    }

    lock = false;

    throw Exception('No message received');
  }
}
