import 'dart:async' show Completer, Future;

import 'package:flutter/material.dart';

import '../services/inbox_service.dart';
import '../types/mail_account.dart';
import '../types/mailbox_info.dart';
import '../types/message.dart';
import '../types/message_flag.dart';
import '../types/notification_info.dart';
import '../widgets/add_account.dart';
import '../widgets/custom_notification.dart';
import '../widgets/message_list/message_list.dart';
import '../widgets/message_list/message_list_header.dart';
import '../widgets/mailbox/mailbox_list_header.dart';
import '../widgets/mailbox/mailbox_list.dart';
import '../widgets/message/message_content.dart';
import '../widgets/message/message_control_bar.dart';
import '../services/overlay_builder.dart';
import '../types/project_colors.dart';
import 'settings.dart';
import '../widgets/vertical_split_view.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  HomePageState createState() => HomePageState();
}

class HomePageState extends State<HomePage> {
  String? _activeMailbox;
  int? _activeSession;
  int? _activeID;
  int _currentPage = 0;

  final _messageLoadCount = 25;

  List<Message> _messages = [];
  Map<int, List<MailboxInfo>> _mailboxTree = {};

  int _messageListKeyIndex = 0;

  final Map<int, NotificationInfo> _notifications = {};

  @override
  void initState() {
    super.initState();

    _initMessages();
  }

  void _initMessages() async {
    if (InboxService().getActiveSessionId() == null) {
      return;
    }

    final inboxes = await InboxService().getMailboxes();

    await _changeMailbox(0, inboxes[0].path);

    _mailboxTree = await InboxService().getMailboxTree();

    setState(() {
      _mailboxTree = _mailboxTree;
    });
  }

  void _showNotification(String message, bool showLoader, Future? callback) {
    final idx = _notifications.keys.length + 1;
    final notification = NotificationInfo(
      idx: idx,
      message: message,
      showLoader: showLoader,
    );

    _notifications[idx] = notification;
    OverlayBuilder().insertOverlay(
      CustomNotification(
        notification: notification,
        callback: callback,
      ),
      idx,
    );
  }

  Future<void> _changeMailbox(int sessionId, String mailboxPath) async {
    if (sessionId == InboxService().getActiveSessionId() &&
        mailboxPath == InboxService().getActiveMailbox()) {
      return;
    }

    InboxService().setActiveSessionId(sessionId);
    InboxService().setActiveMailbox(mailboxPath);

    _messages =
        await InboxService().getMessages(start: 1, end: _messageLoadCount);

    setState(() {
      _messageListKeyIndex += 1;

      _activeMailbox = InboxService().getActiveMailbox();
      _activeSession = InboxService().getActiveSessionId();

      _currentPage = 0;
      _messages = _messages;
    });

    _updateActiveID(0);
  }

  void _updateActiveID(int idx) {
    if (_activeID == idx) return;

    _readMessage();

    setState(() {
      _activeID = idx;
    });
  }

  void _loadMoreMessages() async {
    _currentPage++;

    final completer = Completer();
    _showNotification("Loading more messages", true, completer.future);

    final newMessages = await InboxService().getMessages(
      start: 1 + _currentPage * _messageLoadCount,
      end: _messageLoadCount + _currentPage * _messageLoadCount,
    );

    setState(() {
      _messages.addAll(newMessages);
    });

    await Future.delayed(const Duration(milliseconds: 500), () {});
    completer.complete();

    print('loaded more messages');
  }

  void _readMessage() async {
    await Future.delayed(const Duration(seconds: 2), () {});
  }

  Future<void> _refreshAll() async {
    final updatedMessageUids = await InboxService().updateInbox();

    if (updatedMessageUids.isEmpty) {
      return;
    }

    if (_messages.length < _messageLoadCount) {
      _messages = await InboxService().getMessages(
        start: 1,
        end: _messageLoadCount,
      );
    } else {
      // final loadedUpdatedMessageUids = updatedMessageUids
      //     .where((element) => _messages.any((m) => m.uid == element))
      //     .toList();

      // final updatedMessages = await InboxService().getMessagesWithUids(
      //   uids: loadedUpdatedMessageUids[0],
      // );

      // for (var message in updatedMessages) {
      //   final idx = _messages.indexWhere((m) => m.uid == message.uid);
      //   if (idx != -1) {
      //     _messages[idx] = message;
      //   }
      // }
    }

    setState(() {
      _messages = _messages;
    });
  }

  void _addMailAccount() {
    OverlayBuilder().insertOverlay(const AddAccount(), 0);
  }

  Future<void> _composeMessage() async {
    print('composing a message');
  }

  void _flagMessage(MessageFlag flag) async {
    if (_activeID == null) return;

    final message = _messages[_activeID!];

    final add = !message.flags.contains(flag);
    final messageUid = message.uid;

    await InboxService()
        .modifyFlags(messageUid: messageUid, flags: [flag], add: add);

    setState(() {
      if (add) {
        message.flags.add(flag);
      } else {
        message.flags.remove(flag);
      }
    });
  }

  Future<void> _moveMessage(SpecialMailboxType mailbox) async {
    if (_activeID == null) return;

    final message = _messages[_activeID!];

    final mailboxDest = InboxService().getSpecialMailbox(mailbox);
    final messageUid = message.uid;

    final mailboxNew = await InboxService()
        .moveMessage(messageUid: messageUid, mailboxDest: mailboxDest);

    if (mailboxNew == '') {
      print('failed to move message');
      return;
    }

    setState(() {
      _messages.removeAt(_activeID!);
    });
  }

  Future<void> _reply() async {
    print('reply to message');
  }

  Future<void> _replyAll() async {
    print('reply to all message');
  }

  Future<void> _share() async {
    print('share message');
  }

  void _openSettings() async {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => const SettingsPage(),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    OverlayBuilder().loadContext(context);

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.background(true),
        ),
        child: Center(
          child: VerticalSplitView(
            left: Column(
              children: [
                MailboxHeader(
                  addMailAccount: _addMailAccount,
                  composeMessage: _composeMessage,
                ),
                Expanded(
                  child: MailboxList(
                    key: UniqueKey(),
                    mailboxTree: _mailboxTree,
                    updateMessageList: _changeMailbox,
                    activeMailbox: _activeMailbox ?? '',
                    activeSession: _activeSession ?? 0,
                  ),
                ),
              ],
            ),
            middle: Column(
              children: [
                MessageListHeader(
                  mailboxTitle: InboxService().getActiveMailboxDisplay() ?? '',
                  refreshAll: _refreshAll,
                ),
                Expanded(
                  child: MessageList(
                    key: UniqueKey(),
                    messages: _messages,
                    activeID: _activeID ?? 0,
                    updateActiveID: _updateActiveID,
                    loadMoreMessages: _loadMoreMessages,
                    messageListKey: PageStorageKey<int>(_messageListKeyIndex),
                  ),
                ),
              ],
            ),
            right: Column(
              children: [
                MessageControlBar(
                  flagMessage: _flagMessage,
                  moveMessage: _moveMessage,
                  reply: _reply,
                  replyAll: _replyAll,
                  share: _share,
                  openSettings: _openSettings,
                ),
                Expanded(
                  child: MessageContent(
                    key: ValueKey(
                        "${_activeID != null && _messages.isNotEmpty ? _messages[_activeID!].uid : 0}${_activeID != null && _messages.isNotEmpty ? _messages[_activeID!].messageId : ''}"),
                    message: _activeID != null && _messages.isNotEmpty
                        ? _messages[_activeID!]
                        : null,
                  ),
                ),
              ],
            ),
            ratio2: 0.25,
            minRatio2: 0.1,
            maxRatio2: 0.45,
            ratio1: 0.2,
            minRatio1: 0.1,
            maxRatio1: 0.25,
          ),
        ),
      ),
    );
  }
}
