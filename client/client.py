import streamlit as st
import requests
import json
from components.chat import chatbox, reset_chat_state, history_list
from components.api import converse, authenticate, load_conversation, delete, get_vendors
from components.account import login_dialog, logout, account_buttons
from components.privacy import configure_privacy_parameters


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
if "third_party_data_vendors" not in st.session_state:
    st.session_state["third_party_data_vendors"] = get_vendors()

if "is_authenticated" not in st.session_state:
    st.session_state["is_authenticated"] = st.session_state.uuid is not None

if "privacy_parameters" not in st.session_state:
    st.session_state["privacy_parameters"] = {"storage": st.session_state.is_authenticated, "ads": False, "image_gen": False, "targeted_ads": False, "third_party_data_vendors" : dict([(vendor, False) for vendor in st.session_state["third_party_data_vendors"]])}



def new_chat():
    reset_chat_state()
    privacy_parameters()
    
@st.dialog("Select your privacy settings for this conversation", width="large")
def privacy_parameters():
    configure_privacy_parameters("dialog")
    if st.button("Validate settings"):
        st.rerun()


if __name__ == "__main__":
    st.set_page_config(
        page_title="etosLM",
        page_icon="./etosLM_nobg.jpg",
        layout="wide"
    )
    st.title("Welcome to etosLM!")
    chatbox()

    with st.sidebar:
        st.title("etosLM")
        history, privacy_settings = st.tabs(["History", "Privacy settings"])
        with history:
            account_buttons()
            # st.button("Signup",icon=":material/app_registration:", use_container_width = True)
            st.header("Chat management")
            st.button("New chat", icon=":material/rocket_launch:", on_click=new_chat, use_container_width=True)
            history_list(new_chat)
        with privacy_settings:
            configure_privacy_parameters("sidebar")
