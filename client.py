import streamlit as st
import requests
import json


API_URL = "http://0.0.0.0:8000"

#TODO: Change it to make it a dynamically infered information with a form
if "username" not in st.session_state:
    st.session_state["username"] = "Alex"
if "conv_cache" not in st.session_state:
    st.session_state["conv_cache"] = dict()
if "current_conv_id" not in st.session_state:
    st.session_state["current_conv_id"] = None
if "uuid" not in st.session_state:
    st.session_state["uuid"] = None
if "history" not in st.session_state:
    st.session_state["history"] = []


def login():
    if "uuid" not in st.session_state or st.session_state.uuid is None:
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = {"targeted_ads": "true"}
        payload={"username": f"{st.session_state.username}"}
        try:
            resp = requests.post(f"{API_URL}/login", data=json.dumps(payload), headers=headers, cookies=cookies)
            if resp.status_code == 200:
                st.session_state.uuid = resp.json().get("uuid")
                # Automatically fetch the history from the user if we log in
                get_history()
                print(f"Got assigned uuid: {st.session_state.uuid}")
                return st.toast(f"Welcome back {st.session_state.username}!")
            else:
                return st.toast("Failed to login: Unrecognized username")
        except:
            return st.toast("Server unreachable...")
    else:
        return st.toast(f"Already logged in as {st.session_state.username}")

def logout():
    st.session_state.history=[]
    st.session_state.uuid = None
    st.session_state.current_conv_id = None
    st.username = None
    st.session_state.messages = []
    st.session_state.conv_cache = dict()

def get_history():
    if "uuid" in st.session_state:
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = {"no_storage": "false", "ads": "true", "image_gen": "false", "targeted_ads": "true"}
        resp = requests.get(API_URL+f"/history/{st.session_state.uuid}", headers=headers, cookies=cookies) 
        if resp.status_code == 200:
            history = resp.json().get("history_list")
            if len(history) > 0:
                st.session_state.history = history

def load_conversation(conv_id):
    #Save current conversation locally if we have a current conversation
    print("We are trying to load a conversation")
    if "current_conv_id" in st.session_state and st.session_state.current_conv_id is not None:
        print("Saving current state")
        st.session_state.conv_cache[st.session_state.current_conv_id] = st.session_state.messages
    #If not cached, fetch it
    if conv_id not in st.session_state.conv_cache.keys():
        print("Fetching remote conversation")
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = {"targeted_ads": "true"}
        print(f"User id : {st.session_state.uuid}")
        resp = requests.get(API_URL+f"/c/{st.session_state.uuid}/{conv_id}", headers=headers, cookies=cookies) 
        if resp.status_code == 200:
            conversation = resp.json().get("conv")
            if conversation is not None:
                st.session_state.messages = conversation
    #Else, pull from local
    else:
        st.session_state.messages = st.session_state.conv_cache[conv_id]
    #Context switch on current conv id
    st.session_state.current_conv_id = conv_id

def new_chat():
    #If we were in a chat session that had some content and state, save it, then reset the state
    if "current_conv_id" in st.session_state and "messages" in st.session_state:
        st.session_state.conv_cache[st.session_state.current_conv_id] = st.session_state.messages
        st.session_state.current_conv_id = None
        st.session_state.messages = []
    #We were on an already new chat
    else:
        st.session_state["current_conv_id"] = None
        st.session_state["messages"] = []


def converse():
    payload = {
        'user': st.session_state.username if st.session_state.uuid is not None else "anonymous",
        'uuid': st.session_state.uuid,
        'conv_id': st.session_state.current_conv_id,
        'conversation': st.session_state.messages,
        'nb_token': 20
    }
    headers={'Content-type': 'application/json',
             # 'Accept': 'text/event-stream', 
             'Connection': 'keep-alive',
             'X-Accel-Buffering': 'no'}

    cookies = {"no_storage": "false" if st.session_state.uuid is not None else "true", "ads": "true", "image_gen": "false", "targeted_ads": "false"}
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


with st.sidebar:
    st.title("etosLM")
    st.button("Login", icon=":material/login:",on_click=login, use_container_width=True)
    st.button("Logout", icon=":material/logout:",on_click=logout, use_container_width=True)
    # st.button("Signup",icon=":material/app_registration:", use_container_width = True)
    st.header("Chat management")
    st.button("New chat", icon=":material/rocket_launch:", on_click=new_chat, use_container_width=True)
    if "history" in st.session_state and len(st.session_state.history)> 0:
        for conv in st.session_state.history:
            st.button(conv, on_click=load_conversation, args=(conv,), type="tertiary", use_container_width=True)
    else:
        st.button("Empty chat history!", type="tertiary", disabled=True, use_container_width=True)

st.title("Welcome to etosLM!")

if "messages" not in st.session_state or len(st.session_state.messages) == 0:
    st.session_state.messages = []
    st.chat_message("assistant").write("How can I help you today?")

if "messages" in st.session_state:
    for msg in st.session_state.messages:
        st.chat_message(msg["role"]).write(msg["content"])

if prompt := st.chat_input():
    st.session_state.messages.append({"role": "user", "content": prompt})
    st.chat_message("user").write(prompt)
    response = converse()
    st.session_state.messages.append({"role": "assistant", "content": response})
    st.chat_message("assistant").write(response)
