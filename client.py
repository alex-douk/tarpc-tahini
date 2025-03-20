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

def query_llm(prompt):
    prompt_payload = {'user': 'Alex', 'conversation': prompt, 'nb_token' : 30}
    headers={'Content-type': 'application/json',
             # 'Accept': 'text/event-stream', 
             'Connection': 'keep-alive',
             'X-Accel-Buffering': 'no'}


    cookies = {"no_storage": "false", "ads": "true", "image_gen": "false", "targeted_ads": "false"}
    response = with_requests(host, headers, json.dumps(prompt_payload), cookies)
    if response.status_code == 200:
        return response.json().get("infered_tokens", "Error: No response from the server")
    else:
        print(response.status_code)
        return "Error has occured"

def append_to_send_history(history, message, response=""):
    history = history + [{"user": message, "assistant": response}]
    return history

def append_pre_send(history, message):
    return append_to_send_history(history, message)

def append_post_send(history, message, response):
    return append_to_send_history(history, message, response)


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
        
        def chat_response(history, message):
            #TODO: To have multiturn conversation, we need to fix this client
            history = history + [{"role": "user", "content": message}]
            response = query_llm(history)

            history = history + [response]
            return history, ""

        user_input.submit(chat_response, [chatbox, user_input], [chatbox, user_input])

    demo.launch()
