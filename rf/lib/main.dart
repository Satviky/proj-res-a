import 'package:flutter/material.dart';
import 'screens/chat_screen.dart';

void main() {
  runApp(RescueApp());
}

class RescueApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rescue Game',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: ChatScreen(),
    );
  }
}
