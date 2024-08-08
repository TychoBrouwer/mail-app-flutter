import 'package:http/http.dart' as http;

import 'package:mail_app/types/http_request.dart';

class HttpService {
  final address = 'http://localhost:9001';

  Future<String> sendRequest(
    HttpRequest request,
    Map<String, String> params,
  ) async {
    final query = params.entries.map((e) => '${e.key}=${e.value}').join('&');

    final url = '$address/${request.name}?$query';

    final response = await http.get(Uri.parse(url));

    return response.body;
  }
}
