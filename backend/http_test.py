import requests

from mysecrets import password, username

def main():
    base_url = 'http://localhost:9001'
    
    # Login
    login_params = { 'username': username, 'password': password, 'address': 'imap.gmail.com', 'port': '993' }
    response = requests.get(f'{base_url}/login', params=login_params)
    print(f"received: {response.text}")

    # Get sessions
    sessions_params = {}
    response = requests.get(f'{base_url}/get_sessions', params=sessions_params)
    print(f"received: {response.text}")

    # Get mailboxes
    mailboxes_params = {'session_id': '0'}
    response = requests.get(f'{base_url}/get_mailboxes', params=mailboxes_params)
    print(f"received: {response.text}")

    # Update mailbox
    update_mailbox_params = { 'session_id': '0', 'mailbox_path': 'INBOX' }
    response = requests.get(f'{base_url}/update_mailbox', params=update_mailbox_params)
    print(f"received: {response.text}")

    # Get messages
    messages_params = { 'session_id': '0', 'mailbox_path': 'INBOX', 'message_uids': '91' }
    response = requests.get(f'{base_url}/get_messages_with_uids', params=messages_params)
    print(f"received: {response.text}")

    # Get messages sorted
    messages_params_sorted = { 'session_id': '0', 'mailbox_path': 'INBOX', 'start': '0', 'end': '10' }
    response = requests.get(f'{base_url}/get_messages_sorted', params=messages_params_sorted)
    print(f"received: {response.text}")

    # # Modify flags
    # modify_flags_params = { 'session_id': '0', 'mailbox_path': 'INBOX', 'message_uid': '1', 'flags': 'Seen,Flagged', 'add': 'true' }
    # response = requests.get(f'{base_url}/modify_flags', params=modify_flags_params)
    # print(f"received: {response.text}")

    # # Move message
    # move_message_params = { 'session_id': '0', 'mailbox_path': 'INBOX', 'message_uid': '1', 'mailbox_path_dest': '[Gmail]/All Mail' }
    # response = requests.get(f'{base_url}/move_message', params=move_message_params)
    # print(f"received: {response.text}")

    # # Logout
    # logout_params = { 'session_id': '0' }
    # response = requests.get(f'{base_url}/logout', params=logout_params)
    # print(f"received: {response.text}")


if __name__ == "__main__":
    main()