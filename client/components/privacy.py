import streamlit as st
import json

def configure_privacy_parameters(key_prefix):
    st.header("Prompt policy")
    st.toggle("Save conversation to database", key= key_prefix + "db",value=st.session_state.privacy_parameters["storage"], on_change=switch_boolean_parameters, args=("storage",))
    st.toggle("Use your data to improve services", key= key_prefix + "ads",value=st.session_state.privacy_parameters["ads"], on_change=switch_boolean_parameters, args=("ads",))
    st.toggle("Agree to use third-party un-Tahini'd services", key = key_prefix + "unprot", value=st.session_state.privacy_parameters["image_gen"], on_change=switch_boolean_parameters, args=("image_gen",))
    st.header("Username policy")
    st.toggle("Consent to targeted ads", key = key_prefix + "targeted", value=st.session_state.privacy_parameters["targeted_ads"], on_change=switch_boolean_parameters, args=("targeted_ads",))
    st.header("Third party data vendors")
    st.write("For each of the vendor below, you consent to sending your data for processing")
    for vendor in st.session_state.third_party_data_vendors:
        st.toggle(parse_vendor_string(vendor), key = key_prefix+vendor, value = st.session_state.privacy_parameters["third_party_data_vendors"][vendor], on_change=switch_vendor_consent, args=(vendor,))

    # st.toggle("Use your data to improve services", key= key_prefix + "ads",value=st.session_state.privacy_parameters["ads"], on_change=switch_boolean_parameters, args=("ads",))


def switch_boolean_parameters(key):
    st.session_state.privacy_parameters[key] =  not st.session_state.privacy_parameters[key]

def switch_vendor_consent(key):
    st.session_state.privacy_parameters["third_party_data_vendors"][key] = not st.session_state.privacy_parameters["third_party_data_vendors"][key]


def construct_cookies(policies):
    cookies = dict()
    if "PromptPolicy" in policies:
        cookies["storage"] = parse(st.session_state.privacy_parameters["storage"])
        cookies["ads"] = parse(st.session_state.privacy_parameters["ads"])
        cookies["image_gen"] = parse(st.session_state.privacy_parameters["image_gen"])
    if "UsernamePolicy" in policies:
        cookies["targeted_ads"] = parse(st.session_state.privacy_parameters["targeted_ads"])
    for vendor in st.session_state.third_party_data_vendors:
        cookies[vendor] = parse(st.session_state.privacy_parameters["third_party_data_vendors"][vendor])
    cookies["user_id"] = st.session_state.uuid
    return cookies

def parse(boolean):
    return "true" if boolean else "false"

def parse_vendor_string(vendor):
    return vendor.replace("_", " ")
