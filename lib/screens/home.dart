import 'package:flutter/material.dart';

import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/message_flag.dart';
import 'package:mail_app/types/special_mailbox.dart';
import 'package:mail_app/widgets/add_account.dart';
import 'package:mail_app/widgets/inbox/message_list.dart';
import 'package:mail_app/widgets/mailbox/mailbox_header.dart';
import 'package:mail_app/widgets/mailbox/mailbox_list.dart';
import 'package:mail_app/widgets/message/message_content.dart';
import 'package:mail_app/widgets/control_bar.dart';
import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/vertical_split_view.dart';

class HomePage extends StatefulWidget {
  final InboxService inboxService;

  const HomePage({super.key, required this.inboxService});

  @override
  HomePageState createState() => HomePageState();
}

class HomePageState extends State<HomePage> {
  late InboxService _inboxService;
  late OverlayBuilder _overlayBuilder;

  String? _activeMailbox;
  int? _activeSession;
  int? _activeID;
  int _currentPage = 0;

  final _messageLoadCount = 25;

  List<Message> _messages = [];
  Map<int, List<MailboxInfo>> _mailboxTree = {};

  int _messageListKeyIndex = 0;

  @override
  void initState() {
    super.initState();

    _inboxService = widget.inboxService;

    _initMessages();
  }

  void _initMessages() async {
    if (_inboxService.getActiveSessionId() == null) {
      return;
    }

    final inboxes = await _inboxService.getMailboxes();

    await _changeMailbox(0, inboxes[0].path);

    _mailboxTree = await _inboxService.getMailboxTree();

    setState(() {
      _mailboxTree = _mailboxTree;
    });
  }

  Future<void> _changeMailbox(int sessionId, String mailboxPath) async {
    if (sessionId == _inboxService.getActiveSessionId() &&
        mailboxPath == _inboxService.getActiveMailbox()) {
      return;
    }

    _inboxService.setActiveSessionId(sessionId);
    _inboxService.setActiveMailbox(mailboxPath);

    _messages =
        await _inboxService.getMessages(start: 1, end: _messageLoadCount);

    setState(() {
      _messageListKeyIndex += 1;

      _activeMailbox = _inboxService.getActiveMailbox();
      _activeSession = _inboxService.getActiveSessionId();

      _activeID = 0;
      _currentPage = 0;
      _messages = _messages;
    });
  }

  void _updateActiveID(int idx) {
    if (_activeID == idx) return;

    setState(() {
      _activeID = idx;
    });
  }

  void _loadMoreMessages() async {
    _currentPage++;

    final newMessages = await _inboxService.getMessages(
      start: 1 + _currentPage * _messageLoadCount,
      end: _messageLoadCount + _currentPage * _messageLoadCount,
    );

    setState(() {
      _messages.addAll(newMessages);
    });
  }

  // void _readMessage() async {
  //   await Future.delayed(const Duration(seconds: 2), () {});

  //   MimeMessage message = _inboxService.getMessages()[_activeID];

  //   if (_inboxService
  //       .currentClient()
  //       .getUnseenMessages()
  //       .toList()
  //       .contains(MessageSequence.fromMessage(message).toList().first)) {
  //     await _inboxService.currentClient().flagMessage(
  //         _inboxService.getMessages()[_activeID], MessageUpdate.seen);
  //   }

  //   setState(() {
  //     message = _inboxService.getMessages()[_activeID];
  //   });
  // }

  Future<void> _refreshAll() async {
    // await _inboxService.updateInbox();
  }

  void _addMailAccount() {
    _overlayBuilder.insertOverlay(AddAccount(
      overlayBuilder: _overlayBuilder,
      inboxService: _inboxService,
    ));
  }

  Future<void> _composeMessage() async {
    print('composing a message');
  }

  void _flagMessage(MessageFlag flag) async {
    if (_activeID == null) return;

    final message = _messages[_activeID!];

    final add = !message.flags.contains(flag);
    final messageUid = message.uid;

    final flags = await _inboxService.modifyFlags(
        messageUid: messageUid, flags: [flag], add: add);

    setState(() {
      if (flags.contains(flag)) {
        message.flags.add(flag);
      } else {
        message.flags.remove(flag);
      }
    });
  }

  Future<void> _moveMessage(SpecialMailboxType mailbox) async {
    if (_activeID == null) return;

    final message = _messages[_activeID!];

    final mailboxDest = _inboxService.getSpecialMailbox(mailbox);
    final messageUid = message.uid;

    final mailboxNew = await _inboxService.moveMessage(
        messageUid: messageUid, mailboxDest: mailboxDest);

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

  Future<void> _settings() async {
    print('settings');
  }

  @override
  Widget build(BuildContext context) {
    _overlayBuilder = OverlayBuilder(context);

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.main(true),
        ),
        child: Center(
          child: VerticalSplitView(
            left: Container(
              decoration: BoxDecoration(
                border:
                    Border(right: BorderSide(color: ProjectColors.main(false))),
              ),
              height: double.infinity,
              child: MailboxList(
                mailboxTree: _mailboxTree,
                updateMessageList: _changeMailbox,
                activeMailbox: _activeMailbox ?? '',
                activeSession: _activeSession ?? 0,
                header: MailboxHeader(
                  addMailAccount: _addMailAccount,
                  composeMessage: _composeMessage,
                ),
                key: UniqueKey(),
              ),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: MessageList(
                key: UniqueKey(),
                messages: _messages,
                mailboxTitle: _inboxService.getActiveMailboxDisplay() ?? '',
                activeID: _activeID ?? 0,
                updateActiveID: _updateActiveID,
                refreshAll: _refreshAll,
                loadMoreMessages: _loadMoreMessages,
                messageListKey: PageStorageKey<int>(_messageListKeyIndex),
              ),
            ),
            right: Container(
              decoration: BoxDecoration(
                border:
                    Border(left: BorderSide(color: ProjectColors.main(false))),
              ),
              height: double.infinity,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  ControlBar(
                    flagMessage: _flagMessage,
                    moveMessage: _moveMessage,
                    reply: _reply,
                    replyAll: _replyAll,
                    share: _share,
                    settings: _settings,
                  ),
                  Expanded(
                    child: MessageContent(
                      key: UniqueKey(),
                      message: _activeID != null && _messages.isNotEmpty
                          ? _messages[_activeID!]
                          : null,
                    ),
                  ),
                ],
              ),
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
