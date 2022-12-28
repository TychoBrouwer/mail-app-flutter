import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';

class MailboxHeader extends StatefulWidget {
  final Function composeMessage;
  final Function addMailAccount;

  const MailboxHeader({
    super.key,
    required this.composeMessage,
    required this.addMailAccount,
  });

  @override
  MailboxHeaderState createState() => MailboxHeaderState();
}

class MailboxHeaderState extends State<MailboxHeader> {
  late Function _composeMessage;
  late Function _addMailAccount;

  @override
  void initState() {
    super.initState();

    _composeMessage = widget.composeMessage;
    _addMailAccount = widget.addMailAccount;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(bottom: 15),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          CustomButton(
            onTap: () => _addMailAccount(),
            child: Padding(
              padding: const EdgeInsets.all(10),
              child: SvgPicture.asset(
                'assets/icons/square-plus.svg',
                color: ProjectColors.main(false),
                height: 20,
                width: 20,
              ),
            ),
          ),
          CustomButton(
            onTap: () => _composeMessage(),
            child: Padding(
              padding: const EdgeInsets.all(10),
              child: SvgPicture.asset(
                'assets/icons/paper-plane.svg',
                color: ProjectColors.main(false),
                height: 20,
                width: 20,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
