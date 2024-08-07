class SocketMessage {
  final bool success;
  final dynamic data;
  final String message;

  SocketMessage(this.success, this.data, this.message);

  factory SocketMessage.fromJson(Map<String, dynamic> jsonData) {
    final success = jsonData['success'] as bool;
    final data = jsonData['data'] as dynamic;
    final message = jsonData['message'] as String;

    return SocketMessage(success, data, message);
  }
}
