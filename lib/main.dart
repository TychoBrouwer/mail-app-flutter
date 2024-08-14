import 'package:flutter/material.dart';

import 'package:mail_app/screens/splash.dart';
import 'package:mail_app/types/project_colors.dart';

void main() {
  runApp(const MailApp());
}

class MailApp extends StatelessWidget {
  const MailApp({super.key});

  // This widget is the root of your application.
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
