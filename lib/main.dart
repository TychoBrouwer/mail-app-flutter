import 'package:flutter/material.dart';
import 'package:mail_app/screens/splash.dart';
import 'package:mail_app/types/project_colors.dart';
// import 'package:mail_app/screens/home.dart';

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
        primarySwatch: ProjectColors.accent,
        scrollbarTheme: ScrollbarThemeData(
          thumbColor: MaterialStateProperty.all(ProjectColors.main(false)),
          thickness: MaterialStateProperty.all(6),
        ),
      ),
      home: const SplashPage(),
    );
  }
}
