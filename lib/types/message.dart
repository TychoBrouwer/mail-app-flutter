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

class Message {
  late int uid;
  late String messageId;
  late String subject;
  late List<Address> from;
  late List<Address> sender;
  late List<Address> to;
  late List<Address> cc;
  late List<Address> bcc;
  late List<Address> replyTo;
  late String inReplyTo;
  late String deliveredTo;
  late int date;
  late int received;
  late String text;
  late String html;

  Message(
    this.uid,
    this.messageId,
    this.subject,
    this.from,
    this.sender,
    this.to,
    this.cc,
    this.bcc,
    this.replyTo,
    this.inReplyTo,
    this.deliveredTo,
    this.date,
    this.received,
    this.text,
    this.html,
  );

  factory Message.fromJson(Map<String, dynamic> data) {
    fromJsonList(List<dynamic> data) {
      return data.map((e) => Address.fromJson(e)).toList();
    }

    final uid = data['uid'] as int;
    final messageId = data['message_id'] as String;
    final subject = data['subject'] as String;
    final from = fromJsonList(data['from']);
    final sender = fromJsonList(data['sender']);
    final to = fromJsonList(data['to']);
    final cc = fromJsonList(data['cc']);
    final bcc = fromJsonList(data['bcc']);
    final replyTo = fromJsonList(data['reply_to']);
    final inReplyTo = data['in_reply_to'];
    final deliveredTo = data['delivered_to'];
    final date = data['date'] as int;
    final received = data['received'] as int;
    final text = data['text'] as String;
    final html = data['html'] as String;

    return Message(
      uid,
      messageId,
      subject,
      from,
      sender,
      to,
      cc,
      bcc,
      replyTo,
      inReplyTo,
      deliveredTo,
      date,
      received,
      text,
      html,
    );
  }
}
