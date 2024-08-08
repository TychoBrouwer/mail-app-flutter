class MessageAddress {
  late String name;
  late String mailbox;
  late String host;

  MessageAddress(this.name, this.mailbox, this.host);

  factory MessageAddress.fromJson(Map<String, dynamic> data) {
    final name = data['name'] as String;
    final mailbox = data['mailbox'] as String;
    final host = data['host'] as String;

    return MessageAddress(name, mailbox, host);
  }
}

addressesFromJsonList(List<dynamic> data) {
  return data.map((e) => MessageAddress.fromJson(e)).toList();
}
