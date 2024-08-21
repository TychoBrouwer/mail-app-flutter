import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart' show SvgPicture;

import '../types/project_colors.dart';
import '../types/project_sizes.dart';
import 'custom_button.dart';

class CustomIconButton extends StatelessWidget {
  final void Function() onTap;
  final String icon;
  final String? text;

  const CustomIconButton({
    super.key,
    required this.onTap,
    required this.icon,
    this.text,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.only(right: 10),
      child: CustomButton(
        onTap: () => onTap(),
        child: Padding(
          padding: const EdgeInsets.all(5),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              SizedBox(
                width: ProjectSizes.iconSize,
                height: ProjectSizes.iconSize,
                child: Center(
                  child: SvgPicture.asset(
                    'assets/icons/$icon.svg',
                    colorFilter: ColorFilter.mode(
                        ProjectColors.text(true), BlendMode.srcIn),
                    alignment: Alignment.centerRight,
                    height: ProjectSizes.iconSize,
                    width: ProjectSizes.iconSize,
                  ),
                ),
              ),
              if (text != null)
                Container(
                  padding: const EdgeInsets.only(right: 5, left: 5, bottom: 1),
                  child: Text(
                    text!,
                    style: TextStyle(
                      color: ProjectColors.text(true),
                      fontSize: ProjectSizes.fontSizeSmall,
                    ),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}
