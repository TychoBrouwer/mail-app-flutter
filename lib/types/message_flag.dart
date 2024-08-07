enum MessageFlag {
  seen,
  answered,
  flagged,
  deleted,
  recent,
  draft,
}

messageFlagsFromJsonList(List<dynamic> data) {
  return data.map((e) {
    final str = (e as String).toLowerCase();
    return MessageFlag.values
        .firstWhere((e) => e.toString() == 'MessageFlag.$str');
  }).toList();
}
