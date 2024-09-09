import 'package:flutter/material.dart';

import '../../services/global_configuration.dart';
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
  final ValueKey _key = const ValueKey("SettingsCustomisation");

  _updateSize(String key, double size) {
    Configuration().updateValue(key, size);

    setState(() {});
  }

  Widget _sizeForm(
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
                padding: const EdgeInsets.only(left: 10),
                child: CustomButton(
                  onTap: () {
                    currentSize.value = size;
                    onChanged(currentSize.value);
                  },
                  child: Container(
                    padding: const EdgeInsets.only(
                        top: 3, bottom: 5, left: 8, right: 8),
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
      key: _key,
      child: Padding(
        padding: const EdgeInsets.only(top: 20),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                _sizeForm(
                  ProjectSizes.fontSizeSmall,
                  "Font Size Small",
                  (size) => _updateSize("appSizes:fontSizeSmall", size),
                ),
                _sizeForm(
                  ProjectSizes.fontSize,
                  "Font Size",
                  (size) => _updateSize("appSizes:fontSize", size),
                ),
                _sizeForm(
                  ProjectSizes.fontSizeLarge,
                  "Font Size Large",
                  (size) => _updateSize("appSizes:fontSizeLarge", size),
                ),
                _sizeForm(
                  ProjectSizes.fontSizeExtraLarge,
                  "Font Size Extra Large",
                  (size) => _updateSize("appSizes:fontSizeExtraLarge", size),
                ),
                _sizeForm(
                  ProjectSizes.iconSize,
                  "Icon Size",
                  (size) => _updateSize("appSizes:iconSize", size),
                ),
              ],
            ),
            const SizedBox(width: 80),
            Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                _sizeForm(
                  ProjectSizes.radiusExtraSmall,
                  "Border Radius Extra Small",
                  (size) => _updateSize("appSizes:radiusExtraSmall", size),
                ),
                _sizeForm(
                  ProjectSizes.radiusSmall,
                  "Border Radius Small",
                  (size) => _updateSize("appSizes:radiusSmall", size),
                ),
                _sizeForm(
                  ProjectSizes.radius,
                  "Border Radius",
                  (size) => _updateSize("appSizes:radius", size),
                ),
                _sizeForm(
                  ProjectSizes.radiusLarge,
                  "Border Radius Large",
                  (size) => _updateSize("appSizes:radiusLarge", size),
                ),
                const SizedBox(height: 60),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
