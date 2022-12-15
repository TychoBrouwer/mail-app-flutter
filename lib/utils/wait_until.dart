import 'dart:async';

Future<bool> waitUntil(bool Function() test,
    {final int maxIterations = 200,
    final Duration step = const Duration(milliseconds: 50)}) async {
  int iterations = 0;
  for (; iterations < maxIterations; iterations++) {
    await Future.delayed(step);
    if (test()) {
      break;
    }
  }

  if (iterations >= maxIterations) {
    throw TimeoutException(
        "Condition not reached within ${iterations * step.inMilliseconds}ms");
  }

  return true;
}
