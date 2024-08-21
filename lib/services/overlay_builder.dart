import 'package:flutter/material.dart';

class OverlayBuilder {
  static final OverlayBuilder _singleton = OverlayBuilder._internal();

  late BuildContext _context;
  final Map<int, OverlayEntry?> _overlayEntries = {};

  factory OverlayBuilder() {
    return _singleton;
  }

  OverlayBuilder._internal();

  loadContext(BuildContext context) {
    _context = context;
  }

  void removeOverlay(idx) {
    _overlayEntries[idx]?.remove();
    _overlayEntries[idx]?.dispose();
    _overlayEntries[idx] = null;
  }

  void insertOverlay(Widget overlay, int idx) {
    _overlayEntries[idx] = OverlayEntry(
      builder: (context) => overlay,
    );

    Overlay.of(_context).insert(_overlayEntries[idx]!);
  }
}
