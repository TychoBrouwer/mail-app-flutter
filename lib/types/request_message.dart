class RequestMessage {
  final bool success;
  final dynamic data;
  final String message;

  RequestMessage(this.success, this.data, this.message);

  factory RequestMessage.fromJson(Map<String, dynamic> jsonData) {
    final success = jsonData['success'] as bool;
    final data = jsonData['data'] as dynamic;
    final message = jsonData['message'] as String;

    return RequestMessage(success, data, message);
  }
}
