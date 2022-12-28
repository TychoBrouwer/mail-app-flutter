import 'package:flutter/material.dart';
import 'package:mail_app/types/project_colors.dart';

class CustomButton extends StatelessWidget {
  final void Function() onTap;
  final Widget child;

  const CustomButton({
    super.key,
    required this.onTap,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        borderRadius: BorderRadius.circular(2),
        hoverColor: ProjectColors.secondary(true),
        highlightColor: ProjectColors.secondary(true),
        splashColor: ProjectColors.secondary(true),
        onTap: onTap,
        child: child,
      ),
    );
  }
}
