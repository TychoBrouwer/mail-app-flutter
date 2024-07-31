import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/services/websocket_service.dart';
import 'package:mail_app/screens/home.dart';
import 'package:mail_app/types/project_colors.dart';

class SplashPage extends StatefulWidget {
  const SplashPage({super.key});

  @override
  SplashPageState createState() => SplashPageState();
}

class SplashPageState extends State<SplashPage> {
  double _turns = 0;
  String _status = '';

  @override
  void initState() {
    super.initState();

    _loadHomePage();
  }

  void _loadHomePage() async {
    setState(() => _turns += 100);
    final inboxService = await _loadInboxService();
    setState(() => _status = 'Loading inboxes');
    setState(() => _turns += 100);

    if (!mounted) return;
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(
        builder: (context) => HomePage(
          inboxService: inboxService,
        ),
      ),
    );
  }

  Future<InboxService> _loadInboxService() async {
    final WebsocketService websocketService = WebsocketService();

    await websocketService.connect();

    final inboxService = InboxService(websocketService);
    final sessions = await inboxService.getSessions();

    if (sessions.isNotEmpty) {
      inboxService.setActiveSessionId(sessions[0].sessionId);
    }

    return inboxService;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.secondary(true),
        ),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Padding(
                padding: const EdgeInsets.only(bottom: 30),
                child: Text(
                  _status,
                  style: TextStyle(
                    fontSize: 24,
                    color: ProjectColors.main(false),
                  ),
                ),
              ),
              AnimatedRotation(
                alignment: Alignment.center,
                turns: _turns,
                duration: const Duration(seconds: 100),
                child: SvgPicture.asset(
                  'assets/icons/arrows-rotate.svg',
                  color: ProjectColors.main(false),
                  width: 60,
                  height: 60,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
