import 'package:flutter/material.dart';

import '../types/project_colors.dart';
import '../types/project_sizes.dart';

class CustomButton extends StatelessWidget {
  final void Function() onTap;
  final Widget child;
  final BorderRadius? borderRadius;
  final Color backgroundColor;
  final bool active;

  const CustomButton({
    super.key,
    required this.onTap,
    required this.child,
    this.borderRadius = BorderRadius.zero,
    this.backgroundColor = Colors.transparent,
    this.active = false,
  });

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.transparent,
      child: InkWell(
        borderRadius: borderRadius == BorderRadius.zero
            ? ProjectSizes.borderRadiusSmall
            : borderRadius,
        hoverColor: active ? Colors.transparent : ProjectColors.button(true),
        splashColor: Colors.transparent,
        highlightColor:
            active ? Colors.transparent : ProjectColors.button(false),
        onTap: onTap,
        child: DecoratedBox(
          decoration: BoxDecoration(
            borderRadius: borderRadius == BorderRadius.zero
                ? ProjectSizes.borderRadiusSmall
                : borderRadius,
            color: active ? ProjectColors.button(true) : Colors.transparent,
          ),
          child: child,
        ),
      ),
    );
  }
}
