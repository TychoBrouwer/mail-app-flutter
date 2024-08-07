class Address {
  late String name;
  late String mailbox;
  late String host;

  Address(this.name, this.mailbox, this.host);

  factory Address.fromJson(Map<String, dynamic> data) {
    final name = data['name'] as String;
    final mailbox = data['mailbox'] as String;
    final host = data['host'] as String;

    return Address(name, mailbox, host);
  }
}

addressesFromJsonList(List<dynamic> data) {
  return data.map((e) => Address.fromJson(e)).toList();
}
