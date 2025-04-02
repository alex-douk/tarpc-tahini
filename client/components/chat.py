import streamlit as st
from .api import delete, load_conversation, converse


def chatbox():
    st.markdown(
        """
        <style>
        [data-testid="stChatMessageContent"] p{
            font-size: 1.5rem;
        }
        .stChatMessage:has(.chat-user) {
                flex-direction: row-reverse;
                text-align: right;
            }
        </style>
        """, unsafe_allow_html=True
    )

    if "messages" not in st.session_state or len(st.session_state.messages) == 0:
        st.session_state.messages = []
        st.chat_message("assistant", avatar="./chat_avatar.jpg").write("How can I help you today?")

    if "messages" in st.session_state:
        for msg in st.session_state.messages:
            with st.chat_message(parse_roles(msg["role"])):
                st.html(f"<span class='chat-{parse_roles(msg['role'])}'></span>")
                st.write(msg["content"])


    st.markdown(
        """
        <style>
        textarea {
            font-size: 1.5rem !important;
        }
        input {
            font-size: 1.5rem !important;
        }
        </style>
        """,
        unsafe_allow_html=True,
    )
    if prompt := st.chat_input():
        st.markdown(
            """
            <style>
            [data-testid="stChatMessageContent"] p{
                font-size: 1.5rem;
            }
            .stChatMessage:has(.chat-user) {
                    flex-direction: row-reverse;
                    text-align: right;
                }
            </style>
            """, unsafe_allow_html=True
        )
        st.session_state.messages.append({"role": "user", "content": prompt})
        st.chat_message("user").write(prompt)
        response = converse()
        st.session_state.messages.append({"role": "model", "content": response["content"]})
        with st.chat_message("assistant"):
            st.write(response["content"])
            if response["ad"] is not None:
                st.write(response["ad"])
        # st.rerun()

def reset_chat_state():
    #If we were in a chat session that had some content and state, save it, then reset the state
    if "current_conv_id" in st.session_state and "messages" in st.session_state:
        st.session_state.conv_cache[st.session_state.current_conv_id] = st.session_state.messages
        st.session_state.current_conv_id = None
        st.session_state.messages = []
    #We were on an already new chat
    else:
        st.session_state["current_conv_id"] = None
        st.session_state["messages"] = []
    
def parse_roles(role):
    if role == "model":
        return "assistant"
    return role


def history_list(new_chat_callback):
    if "history" in st.session_state and len(st.session_state.history)> 0:
        for conv in st.session_state.history:
            main, delete_col = st.columns([0.98, 0.02])
            with main:
                st.button(conv, key=str(conv), on_click=load_conversation, args=(conv,), type="tertiary", use_container_width=True)
            with delete_col:
                st.button(":material/delete:", key="delete_"+str(conv), on_click=delete_comp, args=(conv, new_chat_callback), type="tertiary")
    else:
        st.button("Empty chat history!", type="tertiary", disabled=True, use_container_width=True)

def delete_comp(conv_id, new_chat):
    resp = delete(conv_id)
    if resp.status_code == 200:
        st.session_state.history.remove(conv_id)
        new_chat()
        if conv_id in st.session_state.conv_cache.keys():
            del st.session_state.conv_cache[conv_id]
