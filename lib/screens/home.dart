import 'package:flutter/material.dart';

import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/message.dart';
import 'package:mail_app/widgets/add_account.dart';
import 'package:mail_app/widgets/inbox/message_list.dart';
import 'package:mail_app/widgets/mailbox/mailbox_header.dart';
import 'package:mail_app/widgets/mailbox/mailbox_list.dart';
import 'package:mail_app/widgets/message/message_content.dart';
import 'package:mail_app/widgets/message/message_control_bar.dart';
import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/vertical_split.dart';

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

  List<Message> _messages = [];
  Map<int, List<MailboxInfo>> _mailboxTree = {};
  double _messageListPosition = 0;

  @override
  void initState() {
    super.initState();

    _inboxService = widget.inboxService;

    _initMessages();
  }

  void _initMessages() async {
    final inboxes = await _inboxService.getMailboxes();
    _inboxService.setActiveMailbox(inboxes[0].path);

    _messages = await _inboxService.getMessages(start: 1, end: 10);

    setState(() {
      _messages = _messages;
    });

    _mailboxTree = await _inboxService.getMailboxTree();

    setState(() {
      _activeID = 0;
      _mailboxTree = _mailboxTree;
      _activeMailbox = _inboxService.getActiveMailbox();
      _activeSession = _inboxService.getActiveSessionId();
    });
  }

  void _updateActiveID(int idx, double messageListPosition) {
    if (_activeID == idx) return;

    setState(() {
      _activeID = idx;
      _messageListPosition = messageListPosition;
      // message = _inboxService.getMessages().isNotEmpty
      //     ? _inboxService.getMessages()[_activeID]
      //     : MimeMessage();

      // if (!message.isSeen) {
      //   _readMessage();
      // }
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
  //     await _inboxService.currentClient().markMessage(
  //         _inboxService.getMessages()[_activeID], MessageUpdate.seen);
  //   }

  //   setState(() {
  //     message = _inboxService.getMessages()[_activeID];
  //   });
  // }

  void _updateMessagePage(double messageListPosition) async {
    _currentPage++;

    final newMessages = await _inboxService.getMessages(
        start: 1 + _currentPage * 10, end: 10 + _currentPage * 10);

    setState(() {
      _messageListPosition = messageListPosition;
      _messages.addAll(newMessages);
    });
  }

  Future<void> _updateMessageList(
      int sessionId, String mailboxPath, String mailboxTitle) async {
    if (sessionId == _inboxService.getActiveSessionId() &&
        mailboxPath == _inboxService.getActiveMailbox()) {
      return;
    }

    _inboxService.setActiveSessionId(sessionId);
    _inboxService.setActiveMailbox(mailboxPath);

    setState(() {
      _activeMailbox = _inboxService.getActiveMailbox();
      _activeSession = _inboxService.getActiveSessionId();
    });

    _messages = await _inboxService.getMessages(start: 1, end: 10);

    setState(() {
      _activeID = 0;
      _currentPage = 0;
      _messageListPosition = 0;
      _messages = _messages;
    });
  }

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

  void _markMessage() async {
    print('mark message');
    // await _inboxService
    //     .currentClient()
    //     .markMessage(_inboxService.getMessages()[_activeID], messageUpdate);

    // setState(() {
    //   message = _inboxService.getMessages()[_activeID];
    // });
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

  @override
  Widget build(BuildContext context) {
    _overlayBuilder = OverlayBuilder(context);

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.secondary(true),
        ),
        child: Center(
          child: VerticalSplitView(
            left: Container(
              decoration: BoxDecoration(
                border: Border(
                    right: BorderSide(color: ProjectColors.secondary(false))),
              ),
              height: double.infinity,
              child: MailboxList(
                mailboxTree: _mailboxTree,
                updateMessageList: _updateMessageList,
                activeMailbox: _activeMailbox ?? '',
                activeSession: _activeSession ?? 0,
                header: MailboxHeader(
                  addMailAccount: _addMailAccount,
                  composeMessage: _composeMessage,
                  key: UniqueKey(),
                ),
                key: UniqueKey(),
              ),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: MessageList(
                messages: _messages,
                // unseenMessages:
                //     _inboxService.currentClient().getUnseenMessages(),
                mailboxTitle: _inboxService.getActiveMailboxDisplay() ?? '',
                activeID: _activeID ?? 0,
                updateActiveID: _updateActiveID,
                refreshAll: _refreshAll,
                updatePage: _updateMessagePage,
                listPosition: _messageListPosition,
                key: UniqueKey(),
              ),
            ),
            right: Container(
              decoration: BoxDecoration(
                border: Border(
                    left: BorderSide(color: ProjectColors.secondary(false))),
              ),
              height: double.infinity,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  ControlBar(
                    markMessage: _markMessage,
                    reply: _reply,
                    replyAll: _replyAll,
                    share: _share,
                    key: UniqueKey(),
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
