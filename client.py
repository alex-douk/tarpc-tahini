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
    prompt_payload = {'user': 'Alex', 'prompt': prompt, 'nb_token' : 30}
    headers={'Content-type': 'application/x-www-form-urlencoded',
             'Accept': 'text/event-stream', 
             'Connection': 'keep-alive',
             'X-Accel-Buffering': 'no'}


    cookies = {"no_storage": "false", "ads": "true", "image_gen": "false"}
    response = with_requests(host, headers, prompt_payload, cookies)
    return response.json().get("infered_tokens", "Error: No response from the server")


if __name__ == '__main__':
    host = 'http://0.0.0.0:8000/chat'
    # prompt = " You are a Rust expert that has been working on tokio projects for the past 5 years, full time. You have been promised a good bonus if you manage to solve this issue with your codebase : You are supposed to augment a web framework (Axum) by wrapping every type into a container type, named BBox. If you succeed, you will get a tip of $5,000."
    # prompt = "Tell me a funny joke."
    with gr.Blocks() as demo:
        gr.Markdown("# Chat with LLM")
        chatbox = gr.Chatbot(type="messages")
        user_input = gr.Textbox(label="Type your message:")
        
        def chat_response(history, message):
            #TODO: To have multiturn conversation, we need to fix this client
            response = query_llm(message)
            history.append((message, response))
            return history, ""

        user_input.submit(chat_response, [chatbox, user_input], [chatbox, user_input])

    demo.launch()
