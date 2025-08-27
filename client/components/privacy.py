import streamlit as st
import json

def configure_privacy_parameters(key_prefix):
    st.header("Information regarding your conversations:")
    st.toggle("Save conversation to database", key= key_prefix + "db",value=st.session_state.privacy_parameters["storage_consent"], on_change=switch_boolean_parameters, args=("storage_consent",))
    st.toggle("Use your conversations for related advertisements", key= key_prefix + "ads",value=st.session_state.privacy_parameters["ad_consent"], on_change=switch_boolean_parameters, args=("ad_consent",))
    st.toggle("Agree to use third-party un-Tahini'd services", key = key_prefix + "unprot", value=st.session_state.privacy_parameters["image_gen"], on_change=switch_boolean_parameters, args=("image_gen",))
    st.header("Information regarding you as a user:")
    st.toggle("Consent to targeted ads", key = key_prefix + "targeted", value=st.session_state.privacy_parameters["targeted_ads_consent"], on_change=switch_boolean_parameters, args=("targeted_ads_consent",), disabled = not st.session_state.is_authenticated)
    st.header("Third party data vendors")
    st.write("For each of the vendor below, you consent to sending your data for processing")
    for vendor in st.session_state.third_party_data_vendors:
        st.toggle(parse_vendor_string(vendor), key = key_prefix+vendor, value = st.session_state.privacy_parameters["allowed_third_party_data_vendors"][vendor], on_change=switch_vendor_consent, args=(vendor,))


def switch_boolean_parameters(key):
    st.session_state.privacy_parameters[key] =  not st.session_state.privacy_parameters[key]

def switch_vendor_consent(key):
    st.session_state.privacy_parameters["allowed_third_party_data_vendors"][key] = not st.session_state.privacy_parameters["allowed_third_party_data_vendors"][key]


def construct_cookies(policies):
    cookies = dict()
    if (st.session_state.privacy_parameters["storage_consent"]):
        cookies["storage_consent"] = "true"
    if (st.session_state.privacy_parameters["ad_consent"]):
        cookies["ad_consent"] = "true"
    if (st.session_state.privacy_parameters["image_gen"]):
        cookies["image_gen"] = "true"
    if (st.session_state.privacy_parameters["targeted_ads_consent"]):
        cookies["targeted_ads_consent"] = "true"

    cookies["user_id"] = st.session_state.uuid
    cookies["allowed_third_party_data_vendors"] = str([vendor for vendor, consent in st.session_state.privacy_parameters["allowed_third_party_data_vendors"].items() if consent]).replace("'", "\"")
    return cookies

def parse(boolean):
    return "true" if boolean else "false"

def parse_vendor_string(vendor):
    return vendor.replace("_", " ")
