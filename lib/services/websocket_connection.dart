import 'dart:js_interop';

import 'package:web_socket_client/web_socket_client.dart';

class WebsocketConnection {
  late WebSocket _socket;

  // Connect to the websocket server
  WebsocketConnection() {
    // Connect to the websocket server
    final uri = Uri.parse('ws://localhost:9001');
    const backoff = ConstantBackoff(Duration(seconds: 1));

    _socket = WebSocket(uri, backoff: backoff);
  }

  // Disconnect from the websocket server
  void disconnect() {
    // Disconnect from the websocket server
    _socket.close();
  }

  // Send a message to the websocket server
  String sendMessage(String message) {
    // Send a message to the websocket server
    _socket.send(message);

    final response = _socket.connection.listen((event) {
      event;
    });

    return response.toString();
  }

  String getConnections() {
    String request = '/imap/sessions';

    final response = sendMessage(request);
    return response;
  }

  String getMessages(String connectionId, String mailbox, int start, int end) {
    String request =
        '/imap/message_ids\r\nsession_id$connectionId\nmailbox=$mailbox\nstart=$start\nend=$end';

    final response = sendMessage(request);
    return response;
  }

  String getMessage(String connectionId, String mailbox, String messageId) {
    String request =
        '/imap/message\r\nsession_id$connectionId\nmailbox=$mailbox\nmessage_id=$messageId';

    final response = sendMessage(request);
    return response;
  }

  String getMailboxes(String connectionId) {
    String request = '/imap/mailboxes\r\nsession_id$connectionId';

    final response = sendMessage(request);
    return response;
  }
}
