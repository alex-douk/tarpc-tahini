import streamlit as st

def configure_privacy_parameters(key_prefix):
    st.header("Prompt policy")
    st.toggle("Save conversation to database", key= key_prefix + "db",value=st.session_state.privacy_parameters["storage"], on_change=switch_boolean_parameters, args=("storage",))
    st.toggle("Use your data to improve services", key= key_prefix + "ads",value=st.session_state.privacy_parameters["ads"], on_change=switch_boolean_parameters, args=("ads",))
    st.toggle("Agree to use third-party un-Tahini'd services", key = key_prefix + "unprot", value=st.session_state.privacy_parameters["image_gen"], on_change=switch_boolean_parameters, args=("image_gen",))
    st.header("Username policy")
    st.toggle("Consent to targeted ads", key = key_prefix + "targeted", value=st.session_state.privacy_parameters["targeted_ads"], on_change=switch_boolean_parameters, args=("targeted_ads",))

def switch_boolean_parameters(key):
    st.session_state.privacy_parameters[key] =  not st.session_state.privacy_parameters[key]

def construct_cookies(policies):
    cookies = dict()
    if "PromptPolicy" in policies:
        cookies["storage"] = parse(st.session_state.privacy_parameters["storage"])
        cookies["ads"] = parse(st.session_state.privacy_parameters["ads"])
        cookies["image_gen"] = parse(st.session_state.privacy_parameters["image_gen"])
    if "UsernamePolicy" in policies:
        cookies["targeted_ads"] = parse(st.session_state.privacy_parameters["targeted_ads"])

    cookies["user_id"] = st.session_state.uuid
    return cookies

def parse(boolean):
    return "true" if boolean else "false"
