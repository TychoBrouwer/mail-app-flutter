import 'package:http/http.dart' as http;

import 'package:mail_app/types/http_request.dart';

class HttpService {
  final address = 'http://localhost:9001';
  final client = http.Client();

  Future<String> sendRequest(
    HttpRequest request,
    Map<String, String> params,
  ) async {
    String query = params.entries.map((e) => '${e.key}=${e.value}').join('&');
    if (query.isNotEmpty) query = '?$query';

    final url = '$address/${request.name}$query';

    print(url);

    final response = await client.get(Uri.parse(url));

    return response.body;
  }
}
