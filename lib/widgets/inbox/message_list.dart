import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/widgets/custom_button.dart';
import 'package:mail_app/widgets/inbox/message_preview.dart';

class MessageList extends StatefulWidget {
  final List<Message> messages;
  // final MessageSequence unseenMessages;
  final String mailboxTitle;
  final int activeID;
  final void Function(int, double) updateActiveID;
  final Future<void> Function() refreshAll;
  final void Function(double) updateMessagePage;
  final double listPosition;

  const MessageList({
    super.key,
    required this.updateActiveID,
    required this.mailboxTitle,
    required this.messages,
    // required this.unseenMessages,
    required this.activeID,
    required this.refreshAll,
    required this.updateMessagePage,
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
  late void Function(int, double) _updateActiveIDSaveScroll;
  late Future<void> Function() _refreshAll;
  late void Function(double) _updateMessagePage;
  late ScrollController _listController;

  double turns = 0;
  bool rotatingFinished = true;
  bool refreshFinished = false;

  @override
  void initState() {
    super.initState();

    _messages = widget.messages;
    _mailboxTitle = widget.mailboxTitle;
    _activeID = widget.activeID;
    _updateActiveIDSaveScroll = widget.updateActiveID;
    _refreshAll = widget.refreshAll;
    _updateMessagePage = widget.updateMessagePage;
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
    final position = _listController.position;

    if (position.pixels == position.maxScrollExtent) {
      _updateMessagePage(position.pixels);
    }
  }

  void Function(int) _updateActiveID(int idx) {
    return (int idx) {
      _updateActiveIDSaveScroll(idx, _listController.position.pixels);
    };
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
                    fontSize: ProjectSizes.fontSizeLarge,
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
                  getActive: _getActive,
                  updateMessageID: _updateActiveID(idx),
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