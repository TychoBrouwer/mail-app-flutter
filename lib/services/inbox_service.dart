import 'dart:convert';

import 'package:mail_app/services/http_service.dart';
import 'package:mail_app/types/http_request.dart';
import 'package:mail_app/types/mail_account.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/message_flag.dart';
import 'package:mail_app/types/request_message.dart';

class InboxService {
  int? _activeSession;
  String? _activeMailbox;

  final HttpService httpService = HttpService();

  final List<MailAccount> _sessions = [];
  final List<MailboxInfo> _mailboxes = [];

  void setActiveSessionId(int session) {
    _activeSession = session;
  }

  int? getActiveSessionId() {
    return _activeSession;
  }

  void setActiveMailbox(String mailbox) {
    _activeMailbox = mailbox;
  }

  String? getActiveSessionDisplay() {
    final sessions =
        _sessions.where((element) => element.sessionId == _activeSession);

    if (sessions.isEmpty) return '';

    return sessions.first.username;
  }

  String? getActiveMailbox() {
    return _activeMailbox;
  }

  String? getActiveMailboxDisplay() {
    final mailboxes =
        _mailboxes.where((element) => element.path == _activeMailbox);

    if (mailboxes.isEmpty) return '';

    return mailboxes.first.display;
  }

  Future<Map<int, List<MailboxInfo>>> getMailboxTree() async {
    Map<int, List<MailboxInfo>> mailboxTree = {};

    for (var session in _sessions) {
      final mailboxes = await getMailboxes(session: session.sessionId);

      mailboxTree[session.sessionId] = mailboxes;
    }

    return mailboxTree;
  }

  Future<int> newSession(
    String username,
    String password,
    String address,
    int port,
  ) async {
    final body = {
      'email': username,
      'password': password,
      'address': address,
      'port': port.toString(),
    };

    final response = await HttpService().sendRequest(HttpRequest.login, body);

    final decode = jsonDecode(response);
    final messageData = RequestMessage.fromJson(decode);

    if (!messageData.success) return -1;

    final session = messageData.data['id'] as int;

    _sessions.add(MailAccount(
      session,
      username,
      address,
      port,
    ));

    return session;
  }

  Future<List<MailAccount>> getSessions() async {
    final response = await httpService.sendRequest(HttpRequest.sessions, {});

    final decode = jsonDecode(response);

    final messageData = RequestMessage.fromJson(decode);

    if (!messageData.success) return [];

    for (var session in (messageData.data as List)) {
      _sessions.add(MailAccount.fromJson(session));
    }

    return _sessions;
  }

  Future<List<MailboxInfo>> getMailboxes({int? session}) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }

    final body = {
      'session_id': session.toString(),
    };

    final response =
        await HttpService().sendRequest(HttpRequest.mailboxes, body);

    final messageData = RequestMessage.fromJson(jsonDecode(response));

    if (!messageData.success) return [];

    _mailboxes.clear();

    for (var mailbox in (messageData.data as List)) {
      if ((mailbox as String).endsWith(']')) continue;
      _mailboxes
          .add(MailboxInfo.fromJson(mailbox, getActiveSessionDisplay() ?? ''));
    }

    return _mailboxes;
  }

  Future<List<Message>> getMessages({
    int? session,
    String? mailbox,
    required int start,
    required int end,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    final body = {
      'session_id': session.toString(),
      'mailbox': mailbox!,
      'start': start.toString(),
      'end': end.toString(),
    };

    final response =
        await HttpService().sendRequest(HttpRequest.messages, body);

    final messageData = RequestMessage.fromJson(jsonDecode(response));

    if (!messageData.success) return [];

    final List<Message> messages = [];
    for (var message in (messageData.data as List)) {
      messages.add(Message.fromJson(message));
    }

    return messages;
  }

  Future<List<MessageFlag>> updateFlags({
    int? session,
    String? mailbox,
    required int messageUid,
    required List<MessageFlag> flags,
    required bool add,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    final flagsString = flags.map((e) => e.name).join(',');
    final addString = add.toString();

    final body = {
      'session_id': session.toString(),
      'mailbox': mailbox!,
      'message_uid': messageUid.toString(),
      'flags': flagsString,
      'add': addString,
    };

    final response =
        await HttpService().sendRequest(HttpRequest.modify_flags, body);

    final messageData = RequestMessage.fromJson(jsonDecode(response));

    if (!messageData.success) return [];

    return messageFlagsFromJsonList(messageData.data);
  }

  // Future<String> getMessage(int messageUid) async {
  //   String request =
  //       'message\r\nsession_id=$_activeSession\nmailbox=$_activeMailbox\nmessage_uid=$messageUid';

  //   final response = await _websocketService.sendMessage(request);
  //   return response;
  // }
}
