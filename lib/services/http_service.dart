import 'package:http/http.dart' as http;
import 'dart:convert' as convert show jsonDecode;

import 'package:mail_app/types/http_request_path.dart';
import 'package:mail_app/types/message_request.dart';

class HttpService {
  final address = 'http://localhost:9001';

  Future<MessageResponse> sendRequest(
    HttpRequestPath request,
    Map<String, String> params,
  ) async {
    String query = params.entries.map((e) => '${e.key}=${e.value}').join('&');
    if (query.isNotEmpty) query = '?$query';

    final url = '$address/${request.name}$query';

    try {
      final response = await http.get(Uri.parse(url));

      return MessageResponse.fromJson(convert.jsonDecode(response.body));
    } catch (e) {
      print(e);

      return MessageResponse(false, null, "Failed to connect to server");
    }
  }
}
