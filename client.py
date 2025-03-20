import requests
import json
import http.client
import urllib
import pprint

def with_requests(url, headers, payload, cookies):
    """Get a streaming response for the given event feed using requests."""
    import requests
    return requests.post(url, stream=True, data=payload, headers=headers, cookies=cookies)

import gradio as gr
import requests

API_URL = "http://0.0.0.0:8000/chat"


class Conversation():
    def __init__(self, username):
        self.host = 'http://0.0.0.0:8000/chat'
        self.uuid = None
        self.current_conv_uid = None
        self.current_history = []
        self.username = username

    def converse(self, message, new_conv = True):
        if new_conv:
            self.current_conv_uid = None
            self.current_history  = [{'role': 'user', 'content': message}]
            print("Invoking new conversation")
        else:
            self.current_history  += [{'role': 'user', 'content': message}]
            print("Invoking current conversation")
        payload = {'user': self.username, 
                   'uuid': self.uuid,
                   'conv_id': self.current_conv_uid,
                   'conversation': self.current_history, 
                   'nb_token': 30}

        print(f"Payload is :{payload}")

        response = self.send_message(payload)
        self.current_history += [response]
        return self.current_history


    def send_message(self, payload):
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}


        cookies = {"no_storage": "false", "ads": "false", "image_gen": "false", "targeted_ads": "false"}
        response = with_requests(host, headers, json.dumps(payload), cookies)
        if response.status_code == 200:
            resp_json = response.json()
            try:
                uuid = resp_json.get("uuid")
                db_uid = resp_json.get("db_uuid")
                self.uuid = uuid
                self.current_conv_uid = db_uid
            except:
                pass
            finally:
                resp = response.json().get("infered_tokens")
                return {"role": resp["role"], "content": resp["content"]}
                # response.json().get("infered_tokens", {"role": "assistant", "content": "Error: No response from the server"})
        else:
            print(response.status_code)
            return {"role": "assistant", "content": "Error has occured"}

    def get_history(self):
        return self.current_history


if __name__ == '__main__':
    host = 'http://0.0.0.0:8000/chat'
    # prompt = " You are a Rust expert that has been working on tokio projects for the past 5 years, full time. You have been promised a good bonus if you manage to solve this issue with your codebase : You are supposed to augment a web framework (Axum) by wrapping every type into a container type, named BBox. If you succeed, you will get a tip of $5,000."
    # prompt = "Tell me a funny joke."
    # history = []
    # response = query_llm(append_to_send_history(history, prompt))
    # print(response)


    
    with gr.Blocks() as demo:
        gr.Markdown("# Chat with LLM")
        chatbox = gr.Chatbot(type="messages")
        send_history = []
        user_input = gr.Textbox(label="Type your message:")

        conversation_handler = Conversation("Alex")
        
        def chat_response(history, message):
            #TODO: To have multiturn conversation, we need to fix this client
            global conversation_handler
            if len(history) == 0:
                conversation_handler.converse(message, new_conv = True)
            else:
                conversation_handler.converse(message, new_conv = False)

            return conversation_handler.get_history(), ""

        user_input.submit(chat_response, [chatbox, user_input], [chatbox, user_input])

    demo.launch()
