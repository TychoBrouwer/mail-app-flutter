import 'package:flutter/material.dart';

import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';

class CustomButton extends StatelessWidget {
  final void Function() onTap;
  final Widget child;
  final BorderRadius borderRadius;

  const CustomButton({
    super.key,
    required this.onTap,
    required this.child,
    this.borderRadius = ProjectSizes.borderRadiusSmall,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        borderRadius: borderRadius,
        hoverColor: ProjectColors.secondary(true),
        highlightColor: ProjectColors.secondary(true),
        splashColor: ProjectColors.secondary(true),
        onTap: onTap,
        child: child,
      ),
    );
  }
}
