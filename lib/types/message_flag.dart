// ignore_for_file: constant_identifier_names

enum MessageFlag {
  Seen,
  Answered,
  Flagged,
  Deleted,
  Recent,
  Draft,
}

messageFlagsFromJsonList(List<dynamic> data) {
  return data.map((e) {
    final str = e as String;

    return MessageFlag.values
        .firstWhere((e) => e.toString() == 'MessageFlag.$str');
  }).toList();
}
