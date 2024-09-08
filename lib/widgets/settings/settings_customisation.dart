import 'package:flutter/material.dart';

import '../../types/project_colors.dart';
import '../../types/project_sizes.dart';
import '../custom_button.dart';
import '../custom_icon_button.dart';

class SettingsCustomisation extends StatefulWidget {
  const SettingsCustomisation({super.key});

  @override
  SettingsCustomisationState createState() => SettingsCustomisationState();
}

class SettingsCustomisationState extends State<SettingsCustomisation> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  Widget sizeForm(
    double size,
    String labelText,
    void Function(double) onChanged,
  ) {
    final currentSize = ValueNotifier<double>(size);

    return Container(
      padding: const EdgeInsets.only(bottom: 10),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            labelText,
            style: TextStyle(
              color: ProjectColors.text(true),
              fontSize: ProjectSizes.fontSize,
            ),
          ),
          const SizedBox(height: 4),
          Row(
            children: [
              CustomIconButton(
                  onTap: () {
                    if (currentSize.value > 1) {
                      currentSize.value -= 1;
                      onChanged(currentSize.value);
                    }
                  },
                  icon: "minus"),
              const SizedBox(width: 10),
              SizedBox(
                width: 40,
                child: ValueListenableBuilder<double>(
                  valueListenable: currentSize,
                  builder: (context, value, child) {
                    return Text(
                      value.toString(),
                      textAlign: TextAlign.center,
                      style: TextStyle(
                        color: ProjectColors.text(true),
                        fontSize: ProjectSizes.fontSizeLarge,
                      ),
                    );
                  },
                ),
              ),
              const SizedBox(width: 10),
              CustomIconButton(
                  onTap: () {
                    if (currentSize.value < 30) {
                      currentSize.value++;
                      onChanged(currentSize.value);
                    }
                  },
                  icon: "plus"),
              Container(
                width: 60,
                padding: const EdgeInsets.only(left: 10),
                child: CustomButton(
                  onTap: () {
                    currentSize.value = size;
                  },
                  child: Container(
                    padding: const EdgeInsets.only(
                        top: 3, bottom: 5, left: 5, right: 5),
                    child: Text(
                      "Reset",
                      textAlign: TextAlign.center,
                      style: TextStyle(
                        fontSize: ProjectSizes.fontSizeLarge,
                        color: ProjectColors.text(true),
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Form(
        key: _formKey,
        child: Padding(
          padding: const EdgeInsets.only(top: 20),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  sizeForm(
                    ProjectSizes.fontSizeSmall,
                    "Font Size Small",
                    (size) {
                      print("Font size small: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.fontSize,
                    "Font Size",
                    (size) {
                      print("Font size: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.fontSizeLarge,
                    "Font Size Large",
                    (size) {
                      print("Font size Large: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.fontSizeExtraLarge,
                    "Font Size Extra Large",
                    (size) {
                      print("Font size Extra Large: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.iconSize,
                    "Icon Size",
                    (size) {
                      print("Icon size: $size");
                    },
                  ),
                ],
              ),
              const SizedBox(width: 80),
              Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  sizeForm(
                    ProjectSizes.radiusExtraSmall,
                    "Border Radius Extra Small",
                    (size) {
                      print("Border Radius Extra Small: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.radiusSmall,
                    "Border Radius Small",
                    (size) {
                      print("Border Radius Small: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.radius,
                    "Border Radius",
                    (size) {
                      print("Border Radius: $size");
                    },
                  ),
                  sizeForm(
                    ProjectSizes.radiusLarge,
                    "Border Radius Large",
                    (size) {
                      print("Border Radius Large: $size");
                    },
                  ),
                  const SizedBox(height: 60),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
