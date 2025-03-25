import streamlit as st
from .api import authenticate as api_auth, get_history
@st.dialog("Authenticate to etosLM", width="large")
def login_dialog(login_type):
    st.header("Credentials")
    username = st.text_input("Username", value="")
    if username != "":
        resp = api_auth(login_type, username)
        if resp.status_code == 200:
            uuid = resp.json().get("uuid")
            if uuid is not None:
                st.session_state.uuid = uuid
                # Automatically fetch the history from the user if we log in
                if login_type == "login":
                    get_history()
                st.session_state.username = username
                st.session_state.is_authenticated = True
                st.rerun()
                return True
            else:
                st.warning(f"{login_type} failed")
        else:
            return False

def logout():
    st.session_state.history=[]
    st.session_state.uuid = None
    st.session_state.current_conv_id = None
    st.username = None
    st.session_state.messages = []
    st.session_state.conv_cache = dict()
    st.session_state.is_authenticated = False

def authenticate(endpoint):
    if "uuid" not in st.session_state or st.session_state.uuid is None:
        success = login_dialog(endpoint)
        if success is not None:
            if success:
                st.toast(f"Welcome back {st.session_state.username}!")
            else:
                st.toast("Server-side error for login")
    else:
        return st.toast(f"Already logged in as {st.session_state.username}")


def account_buttons():
    st.button("Login", icon=":material/login:",on_click=authenticate, args=("login",), use_container_width=True)
    st.button("Signup", icon=":material/login:",on_click=authenticate, args=("signup",), use_container_width=True)
    st.button("Logout", icon=":material/logout:",on_click=logout, use_container_width=True)
