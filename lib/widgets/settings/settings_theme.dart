import 'package:flutter/material.dart';

import '../../services/global_configuration.dart';
import '../../types/project_colors.dart';
import '../../types/project_sizes.dart';
import '../../utils/hex_color.dart';
import '../custom_button.dart';

class SettingsTheme extends StatefulWidget {
  const SettingsTheme({super.key});

  @override
  SettingsThemeState createState() => SettingsThemeState();
}

class SettingsThemeState extends State<SettingsTheme> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  _updateColor(String key, Color color) {
    Configuration().updateValue(key, color);

    setState(() {});
  }

  Widget colorForm(
    Color color,
    String labelText,
    void Function(Color) onChanged,
  ) {
    final initialValue = color.toHex();
    final colorString = ValueNotifier<String>(initialValue);

    final controller = TextEditingController(text: colorString.value);

    return Container(
      padding: const EdgeInsets.only(bottom: 10),
      width: 300,
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
          Row(
            children: [
              ValueListenableBuilder<String>(
                valueListenable: colorString,
                builder: (context, value, child) {
                  return Container(
                    width: 30,
                    height: 30,
                    decoration: BoxDecoration(
                      border: Border.all(
                        color: ProjectColors.accent(true),
                        width: 1,
                      ),
                      color: HexColor.fromHex(value),
                    ),
                    margin: const EdgeInsets.only(right: 10),
                  );
                },
              ),
              Expanded(
                child: TextFormField(
                  controller: controller,
                  cursorColor: ProjectColors.accent(true),
                  cursorWidth: 0.6,
                  onChanged: (val) {
                    if (!(_formKey.currentState?.validate() ?? false)) {
                      return;
                    }

                    final color = HexColor.fromHex(val);
                    colorString.value = val;

                    onChanged(color);
                  },
                  validator: (color) {
                    if (color == null) {
                      return "Please enter a color";
                    }

                    if (color.isEmpty) {
                      return "Please enter a color";
                    }

                    if (!RegExp(r'^#[0-9a-fA-F]{8}$').hasMatch(color)) {
                      return "Please enter a valid hex color";
                    }

                    return null;
                  },
                  decoration: InputDecoration(
                    isDense: true,
                    errorText: '',
                    contentPadding: const EdgeInsets.symmetric(
                      vertical: 5,
                      horizontal: 5,
                    ),
                    enabledBorder: UnderlineInputBorder(
                      borderSide: BorderSide(
                          width: 2, color: ProjectColors.accent(false)),
                    ),
                    focusedBorder: UnderlineInputBorder(
                      borderSide: BorderSide(
                          width: 2, color: ProjectColors.accent(true)),
                    ),
                    focusedErrorBorder: UnderlineInputBorder(
                      borderSide: BorderSide(
                          width: 2, color: ProjectColors.accent(true)),
                    ),
                    errorBorder: UnderlineInputBorder(
                      borderSide: BorderSide(
                          width: 2, color: ProjectColors.accent(false)),
                    ),
                    labelStyle: TextStyle(color: ProjectColors.text(true)),
                    hintStyle: TextStyle(color: ProjectColors.text(true)),
                    errorStyle: const TextStyle(fontWeight: FontWeight.w500),
                  ),
                  style: TextStyle(
                    color: ProjectColors.text(true),
                    fontSize: ProjectSizes.fontSizeLarge,
                  ),
                ),
              ),
              Container(
                padding: const EdgeInsets.only(left: 10),
                child: CustomButton(
                  onTap: () {
                    colorString.value = initialValue;
                    controller.value = TextEditingValue(
                      text: colorString.value,
                      selection: TextSelection.collapsed(
                          offset: colorString.value.length),
                    );

                    _formKey.currentState?.validate();
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
                  colorForm(
                    ProjectColors.accent(true),
                    "Accent Color",
                    (color) => _updateColor("appColors:accent", color),
                  ),
                  colorForm(
                    ProjectColors.accent(false),
                    "Accent Color Secondary",
                    (color) => _updateColor("appColors:accentSecondary", color),
                  ),
                  colorForm(
                    ProjectColors.background(true),
                    "Background Color",
                    (color) => _updateColor("appColors:background", color),
                  ),
                  colorForm(
                    ProjectColors.background(false),
                    "Background Color Secondary",
                    (color) =>
                        _updateColor("appColors:backgroundSecondary", color),
                  ),
                  colorForm(
                    ProjectColors.text(true),
                    "Text Color",
                    (color) => _updateColor("appColors:text", color),
                  ),
                  colorForm(
                    ProjectColors.text(false),
                    "Text Color Secondary",
                    (color) => _updateColor("appColors:textSecondary", color),
                  ),
                ],
              ),
              const SizedBox(width: 80),
              Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  colorForm(
                    ProjectColors.border(true),
                    "Border Color",
                    (color) => _updateColor("appColors:border", color),
                  ),
                  colorForm(
                    ProjectColors.border(false),
                    "Border Color Secondary",
                    (color) => _updateColor("appColors:borderSecondary", color),
                  ),
                  colorForm(
                    ProjectColors.button(true),
                    "Button Color",
                    (color) => _updateColor("appColors:button", color),
                  ),
                  colorForm(
                    ProjectColors.button(false),
                    "Button Color Secondary",
                    (color) => _updateColor("appColors:buttonSecondary", color),
                  ),
                  colorForm(
                    ProjectColors.header(true),
                    "Header Color",
                    (color) => _updateColor("appColors:header", color),
                  ),
                  colorForm(
                    ProjectColors.header(false),
                    "Header Color Secondary",
                    (color) => _updateColor("appColors:headerSecondary", color),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
