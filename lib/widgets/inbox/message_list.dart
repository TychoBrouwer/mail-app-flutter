import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';
import 'package:mail_app/widgets/inbox/message_preview.dart';

class MessageList extends StatefulWidget {
  final List<Message> messages;
  // final MessageSequence unseenMessages;
  final String mailboxTitle;
  final int activeID;
  final Function updateActiveID;
  final Future<void> Function() refreshAll;
  final Function updatePage;
  final double listPosition;

  const MessageList({
    super.key,
    required this.updateActiveID,
    required this.mailboxTitle,
    required this.messages,
    // required this.unseenMessages,
    required this.activeID,
    required this.refreshAll,
    required this.updatePage,
    required this.listPosition,
  });

  @override
  MessageListState createState() => MessageListState();
}

class MessageListState extends State<MessageList> {
  late List<Message> _messages;
  // late MessageSequence _unseenMessages;
  late String _mailboxTitle;
  late int _activeID;
  late Function _updateActiveID;
  late Future<void> Function() _refreshAll;
  late Function _updatePage;
  late ScrollController _listController;

  double turns = 0;
  bool rotatingFinished = true;
  bool refreshFinished = false;

  @override
  void initState() {
    super.initState();

    _messages = widget.messages;
    // _unseenMessages = widget.unseenMessages;
    _mailboxTitle = widget.mailboxTitle;
    _activeID = widget.activeID;
    _updateActiveID = widget.updateActiveID;
    _refreshAll = widget.refreshAll;
    _updatePage = widget.updatePage;
    _listController =
        ScrollController(initialScrollOffset: widget.listPosition);

    _listController.addListener(_loadMore);
  }

  void _refreshRotate() async {
    if (!rotatingFinished) return;

    rotatingFinished = false;

    setState(() {
      turns += 1;
    });

    await Future.delayed(const Duration(seconds: 1), () {});

    rotatingFinished = true;

    if (!refreshFinished) _refreshRotate();
  }

  @override
  void dispose() {
    _listController.dispose();
    super.dispose();
  }

  void _loadMore() {
    if (_listController.position.pixels ==
        _listController.position.maxScrollExtent) {
      _updatePage(_listController.position.pixels);
    }
  }

  void _updateActiveIDSaveScroll(int idx) {
    _updateActiveID(idx, _listController.position.pixels);
  }

  bool _getActive(int idx) {
    return _activeID == idx;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.only(left: 15, right: 15, top: 10),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  RegExp(r'.*@.*\.').hasMatch(_mailboxTitle)
                      ? 'INBOX'
                      : _mailboxTitle.toUpperCase(),
                  textAlign: TextAlign.left,
                  style: TextStyle(
                    fontSize: 16,
                    fontWeight: FontWeight.bold,
                    color: ProjectColors.main(false),
                  ),
                ),
              ),
              CustomButton(
                onTap: () async => {
                  refreshFinished = false,
                  _refreshRotate(),
                  await _refreshAll(),
                  refreshFinished = true,
                },
                child: AnimatedRotation(
                  alignment: Alignment.center,
                  turns: turns,
                  duration: const Duration(seconds: 1),
                  child: Padding(
                    padding: const EdgeInsets.all(5),
                    child: SvgPicture.asset(
                      'assets/icons/arrows-rotate.svg',
                      colorFilter: ColorFilter.mode(
                          ProjectColors.main(false), BlendMode.srcIn),
                      width: 20,
                      height: 20,
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 15),
            child: ListView.builder(
              controller: _listController,
              padding: const EdgeInsets.only(bottom: 200),
              itemBuilder: (_, idx) {
                return MailPreview(
                  message: _messages[idx],
                  idx: idx,
                  unseen: false,
                  // unseen: _unseenMessages.toList().contains(
                  //     MessageSequence.fromMessage(_messages[idx])
                  //         .toList()
                  //         .last),
                  getActive: _getActive,
                  updateMessageID: _updateActiveIDSaveScroll,
                  key: UniqueKey(),
                );
              },
              itemCount: _messages.length,
            ),
          ),
        ),
      ],
    );
  }
}
