import streamlit as st
import requests
import json

from .privacy import construct_cookies

API_URL = "http://0.0.0.0:8000"

def converse():
    payload = {
        'user': st.session_state.username if st.session_state.uuid is not None else "anonymous",
        # 'uuid': st.session_state.uuid,
        'conv_id': st.session_state.current_conv_id,
        'conversation': st.session_state.messages,
        'nb_token': 300
    }

    headers={'Content-type': 'application/json',
             # 'Accept': 'text/event-stream', 
             'Connection': 'keep-alive',
             'X-Accel-Buffering': 'no'}

    cookies = construct_cookies(["PromptPolicy", "UsernamePolicy"])
    resp = requests.post(f"{API_URL}/chat", stream=True, data= json.dumps(payload), headers=headers, cookies = cookies)
    if resp.status_code == 200:
        resp_json = resp.json()
        st.session_state.current_conv_id = resp_json.get("db_uuid")
        print(f"Got conversation ID {st.session_state.current_conv_id}")
        if st.session_state.current_conv_id is not None:
            st.session_state.history.append(st.session_state.current_conv_id)
        return resp_json.get("infered_tokens")["content"]
    else:
        return "An error has occured"

def authenticate(login_type, username):
    headers={'Content-type': 'application/json',
             # 'Accept': 'text/event-stream', 
             'Connection': 'keep-alive',
             'X-Accel-Buffering': 'no'}

    cookies = construct_cookies(["UsernamePolicy"])
    payload={"username": f"{username}"}
    return requests.post(f"{API_URL}/account/{login_type}", data=json.dumps(payload), headers=headers, cookies=cookies)

def load_conversation(conv_id):
    #Save current conversation locally if we have a current conversation
    if "current_conv_id" in st.session_state and st.session_state.current_conv_id is not None:
        st.session_state.conv_cache[st.session_state.current_conv_id] = st.session_state.messages
    #If not cached, fetch it
    if conv_id not in st.session_state.conv_cache.keys():
        print("Fetching remote conversation")
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = construct_cookies(["UsernamePolicy"])
        resp = requests.get(API_URL+f"/c/{conv_id}", headers=headers, cookies=cookies) 
        if resp.status_code == 200:
            conversation = resp.json().get("conv")
            if conversation is not None:
                st.session_state.messages = conversation
                st.session_state.current_conv_id = conv_id
            else:
                st.warning(f"Couldn't load conversation with ID {conv_id}")
        else:
            st.warning(f"Couldn't load conversation with ID {conv_id}")
    #Else, pull from local
    else:
        st.session_state.messages = st.session_state.conv_cache[conv_id]
        #Context switch on current conv id
        st.session_state.current_conv_id = conv_id

def get_history():
    if "uuid" in st.session_state:
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = construct_cookies(["UsernamePolicy"])
        resp = requests.get(API_URL+f"/history/{st.session_state.uuid}", headers=headers, cookies=cookies) 
        if resp.status_code == 200:
            history = resp.json().get("history_list")
            if len(history) > 0:
                st.session_state.history = history

def delete(conv_id):
    cookies = construct_cookies([])
    return requests.get(API_URL+f"/history/delete/{conv_id}",cookies=cookies) 

