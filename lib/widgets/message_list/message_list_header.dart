import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart' show SvgPicture;

import '../../types/project_colors.dart';
import '../../types/project_sizes.dart';
import '../custom_button.dart';

class MessageListHeader extends StatefulWidget {
  final String mailboxTitle;
  final Future<void> Function() refreshAll;

  const MessageListHeader({
    super.key,
    required this.refreshAll,
    required this.mailboxTitle,
  });

  @override
  MessageListHeaderState createState() => MessageListHeaderState();
}

class MessageListHeaderState extends State<MessageListHeader> {
  late String _mailboxTitle;
  late Future<void> Function() _refreshAll;

  double turns = 0;
  bool rotatingFinished = true;
  bool refreshFinished = false;

  @override
  void initState() {
    super.initState();

    _mailboxTitle = widget.mailboxTitle;
    _refreshAll = widget.refreshAll;
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
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: ProjectColors.header(true),
      ),
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
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
                color: ProjectColors.text(true),
              ),
            ),
          ),
          CustomButton(
            onTap: () async {
              refreshFinished = false;
              _refreshRotate();
              await _refreshAll();
              refreshFinished = true;
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
                      ProjectColors.text(true), BlendMode.srcIn),
                  width: ProjectSizes.iconSize,
                  height: ProjectSizes.iconSize,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
