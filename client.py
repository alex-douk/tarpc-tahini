import requests
import json
import http.client
import urllib
import pprint

def with_requests(url, headers, payload, cookies):
    """Get a streaming response for the given event feed using requests."""
    import requests
    return requests.post(url, stream=True, data=payload, headers=headers, cookies=cookies)

if __name__ == '__main__':
    host = 'http://0.0.0.0:8000/chat'
    # prompt = " You are a Rust expert that has been working on tokio projects for the past 5 years, full time. You have been promised a good bonus if you manage to solve this issue with your codebase : You are supposed to augment a web framework (Axum) by wrapping every type into a container type, named BBox. If you succeed, you will get a tip of $5,000."
    prompt = "Tell me a funny joke."
    prompt_payload = {'user': 'Alex', 'prompt': prompt, 'nb_token' : 30}
    headers={'Content-type': 'application/x-www-form-urlencoded',
             'Accept': 'text/event-stream', 
             'Connection': 'keep-alive',
             'X-Accel-Buffering': 'no'}


    cookies = {"no_storage": "false", "ads": "true", "image_gen": "false"}
    response = with_requests(host, headers, prompt_payload, cookies)
    print(response.text)
