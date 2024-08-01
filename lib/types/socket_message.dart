class MessageData {
  final bool success;
  final dynamic data;
  final String message;

  MessageData(this.success, this.data, this.message);

  factory MessageData.fromJson(Map<String, dynamic> jsonData) {
    final success = jsonData['success'] as bool;
    final data = jsonData['data'] as dynamic;
    final message = jsonData['message'] as String;

    return MessageData(success, data, message);
  }
}
