import 'package:flutter/material.dart';

import 'package:mail_app/screens/splash.dart';
import 'package:mail_app/services/global_configuration.dart';
import 'package:mail_app/types/project_colors.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  await Configuration().loadFromLocal();

  runApp(const MailApp());
}

class MailApp extends StatelessWidget {
  const MailApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Mail App',
      theme: ThemeData(
        scrollbarTheme: ScrollbarThemeData(
          thumbColor: WidgetStateProperty.all(ProjectColors.background(false)),
          thumbVisibility: WidgetStateProperty.all(true),
          thickness: WidgetStateProperty.all(5),
          crossAxisMargin: 0,
        ),
      ),
      home: const SplashPage(),
    );
  }
}
