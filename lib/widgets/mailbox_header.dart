import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mail_app/types/project_colors.dart';

class MailboxHeader extends StatefulWidget {
  final Function composeMessage;

  const MailboxHeader({
    super.key,
    required this.composeMessage,
  });

  @override
  _MailboxHeader createState() => _MailboxHeader();
}

class _MailboxHeader extends State<MailboxHeader> {
  late Function _composeMessage;

  @override
  void initState() {
    super.initState();

    _composeMessage = widget.composeMessage;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(top: 10, bottom: 25),
      padding: const EdgeInsets.symmetric(horizontal: 5),
      child: GestureDetector(
        onTap: () => _composeMessage(),
        child: SvgPicture.asset(
          'assets/icons/paper-plane.svg',
          color: ProjectColors.main(false),
          alignment: Alignment.centerRight,
          height: 20,
          width: 20,
        ),
      ),
    );
  }
}
