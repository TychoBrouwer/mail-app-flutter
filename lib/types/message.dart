import 'package:mail_app/types/message_address.dart';
import 'package:mail_app/types/message_flag.dart';

class Message {
  late int uid;
  late String messageId;
  late String subject;
  late List<MessageAddress> from;
  late List<MessageAddress> sender;
  late List<MessageAddress> to;
  late List<MessageAddress> cc;
  late List<MessageAddress> bcc;
  late List<MessageAddress> replyTo;
  late String inReplyTo;
  late String deliveredTo;
  late int date;
  late int received;
  late List<MessageFlag> flags;
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
    this.flags,
    this.text,
    this.html,
  );

  factory Message.fromJson(Map<String, dynamic> data) {
    final uid = data['uid'] as int;
    final messageId = data['message_id'] as String;
    final subject = data['subject'] as String;
    final from = addressesFromJsonList(data['from']);
    final sender = addressesFromJsonList(data['sender']);
    final to = addressesFromJsonList(data['to']);
    final cc = addressesFromJsonList(data['cc']);
    final bcc = addressesFromJsonList(data['bcc']);
    final replyTo = addressesFromJsonList(data['reply_to']);
    final inReplyTo = data['in_reply_to'];
    final deliveredTo = data['delivered_to'];
    final date = data['date'] as int;
    final received = data['received'] as int;
    final flags = messageFlagsFromJsonList(data['flags']);
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
      flags,
      text,
      html,
    );
  }
}
