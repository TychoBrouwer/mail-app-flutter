import requests
import logging

import http.client
http.client.HTTPConnection.debuglevel = 1

# You must initialize logging, otherwise you'll not see debug output.
logging.basicConfig()
logging.getLogger().setLevel(logging.DEBUG)
requests_log = logging.getLogger("requests.packages.urllib3")
requests_log.setLevel(logging.DEBUG)
requests_log.propagate = True

def main():
    base_url = 'http://localhost:9001'
    
    # Login request
    login_params = {
        'username': 'test1928346534@gmail.com',
        'password': 'aydogbzktvjtvqsa',
        'address': 'imap.gmail.com',
        'port': '993'
    }
    response = requests.get(f'{base_url}/login', params=login_params)
    print(f"received: {response.text}")
    print(response.elapsed.total_seconds())

    # Messages request
    # messages_params = {
    #     'session_id': '0',
    #     'mailbox': 'INBOX',
    #     'start': '1',
    #     'end': '10'
    # }
    # response = requests.get(f'{base_url}/messages', params=messages_params)
    # print(f"received: {response.text}")

    # Uncomment and modify the following requests as needed

    # mailboxes_params = {'session_id': '0'}
    # response = requests.get(f'{base_url}/mailboxes', params=mailboxes_params)
    # print(f"received: {response.text}")

    # sessions_params = {}
    # response = requests.get(f'{base_url}/sessions', params=sessions_params)
    # print(f"received: {response.text}")

    # messages_params_2 = {
    #     'session_id': '0',
    #     'mailbox': 'INBOX',
    #     'start': '10',
    #     'end': '20'
    # }
    # response = requests.get(f'{base_url}/messages', params=messages_params_2)
    # print(f"received: {response.text}")

    # message_ids_params = {'session_id': '0', 'mailbox': 'INBOX'}
    # response = requests.get(f'{base_url}/message_ids', params=message_ids_params)
    # print(f"received: {response.text}")

    # message_params = {'session_id': '0', 'mailbox': 'INBOX', 'message_uid': '25'}
    # response = requests.get(f'{base_url}/message', params=message_params)
    # print(f"received: {response.text}")

    # modify_flags_params = {
    #     'session_id': '0',
    #     'mailbox': 'INBOX',
    #     'message_uid': '22',
    #     'flags': 'Seen',
    #     'add': 'true'
    # }
    # response = requests.get(f'{base_url}/modify_flags', params=modify_flags_params)
    # print(f"received: {response.text}")

    # message_params_2 = {'session_id': '0', 'mailbox': 'INBOX', 'message_uid': '41'}
    # response = requests.get(f'{base_url}/message', params=message_params_2)
    # print(f"received: {response.text}")

    # message_params_3 = {'session_id': '0', 'mailbox': 'INBOX', 'message_uid': '30'}
    # response = requests.get(f'{base_url}/message', params=message_params_3)
    # print(f"received: {response.text}")

    # message_params_4 = {'session_id': '0', 'mailbox': 'INBOX', 'message_uid': '22'}
    # response = requests.get(f'{base_url}/message', params=message_params_4)
    # print(f"received: {response.text}")

if __name__ == "__main__":
    main()