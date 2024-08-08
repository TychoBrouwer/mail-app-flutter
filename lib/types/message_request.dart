class MessageResponse {
  final bool success;
  final dynamic data;
  final String message;

  MessageResponse(this.success, this.data, this.message);

  factory MessageResponse.fromJson(Map<String, dynamic> jsonData) {
    final success = jsonData['success'] as bool;
    final data = jsonData['data'] as dynamic;
    final message = jsonData['message'] as String;

    return MessageResponse(success, data, message);
  }
}
