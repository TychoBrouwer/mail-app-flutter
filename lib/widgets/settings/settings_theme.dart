import 'package:flutter/material.dart';
import 'package:mail_app/widgets/custom_button.dart';

import '../../services/global_configuration.dart';
import '../../types/project_colors.dart';
import '../../types/project_sizes.dart';
import '../../utils/hex_color.dart';

class SettingsTheme extends StatefulWidget {
  const SettingsTheme({super.key});

  @override
  SettingsThemeState createState() => SettingsThemeState();
}

class SettingsThemeState extends State<SettingsTheme> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  String rgbaToHex(int r, int g, int b, double a) {
    final aHex = (a * 255).round().toRadixString(16).padLeft(2, '0');
    final rHex = r.toRadixString(16).padLeft(2, '0');
    final gHex = g.toRadixString(16).padLeft(2, '0');
    final bHex = b.toRadixString(16).padLeft(2, '0');

    return "#$aHex$rHex$gHex$bHex";
  }

  updateColor(String key, int r, int g, int b, double a) {
    Configuration().updateValue(key, "$r, $g, $b, $a");
  }

  Widget colorForm(
    Color color,
    String labelText,
    void Function(Color) onChanged,
  ) {
    final initialValue = color.toHex();
    final colorString = ValueNotifier<String>(initialValue);

    final controller = TextEditingController(text: colorString.value);

    return Padding(
      padding: const EdgeInsets.only(bottom: 10),
      child: SizedBox(
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
                          color: Colors.blue,
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
                  width: 60,
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
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  colorForm(
                    ProjectColors.accent(true),
                    "Accent Color",
                    (color) {
                      print("Accent Color: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.accent(false),
                    "Accent Color Secondary",
                    (color) {
                      print("Accent Color Secondary: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.background(true),
                    "Background Color",
                    (color) {
                      print("Background Color: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.background(false),
                    "Background Color Secondary",
                    (color) {
                      print("Background Color Secondary: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.text(true),
                    "Text Color",
                    (color) {
                      print("Text Color: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.text(false),
                    "Text Color Secondary",
                    (color) {
                      print("Text Color Secondary: ${color.toHex()}");
                    },
                  ),
                ],
              ),
              const SizedBox(width: 80),
              Column(
                children: [
                  colorForm(
                    ProjectColors.border(true),
                    "Border Color",
                    (color) {
                      print("Border Color: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.border(false),
                    "Border Color Secondary",
                    (color) {
                      print("Border Color Secondary: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.button(true),
                    "Button Color",
                    (color) {
                      print("Button Color: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.button(false),
                    "Button Color Secondary",
                    (color) {
                      print("Button Color Secondary: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.header(true),
                    "Header Color",
                    (color) {
                      print("Header Color: ${color.toHex()}");
                    },
                  ),
                  colorForm(
                    ProjectColors.header(false),
                    "Header Color Secondary",
                    (color) {
                      print("Header Color Secondary: ${color.toHex()}");
                    },
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
