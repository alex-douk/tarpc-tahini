#!/usr/bin/python
# What are the flows we are trying to test here?
# Anonymous user and authenticated ones
# Login -> Chat -> 

#Start
#Signup + privacy config

#CycleLogin (logout, login) (clear conversation history table, and checks if there is at least one available, if so choose at random a new conversation)
#Change to a  random existing conversation from history
#Start new chat
#Update chat privacy configuration
#Delete a conversation in history
#Chat with the LLM



import random
import json
import string
from locust import FastHttpUser, TaskSet, between, task
from faker import Faker
import datetime
fake = Faker()

VENDORS = ["Meta_Ads", "Google_Ads"]

def parse(b):
    "true" if b else "false"

class CookieConfig():
    def __init__(self, anonymous=False):
        #Conversation based
        self.storage_consent = bool(random.getrandbits(1))
        self.ad_consent = bool(random.getrandbits(1))
        self.image_gen = bool(random.getrandbits(1))

        #User based
        self.targeted_ad_consent = bool(random.getrandbits(1))
        sample_size = random.randint(0, len(VENDORS))
        self.third_party_ad_vendors = random.sample(VENDORS, k=sample_size)
        self.user_id = None


    def construct(self):
        cookies =  dict()
        cookies["storage_consent"] = parse(self.storage_consent)
        cookies["ad_consent"] = parse(self.ad_consent)
        cookies["image_gen"] = parse(self.image_gen)
        cookies["targeted_ads_consent"] = parse(self.targeted_ad_consent)
        cookies["user_id"] = self.user_id
        cookies["allowed_third_party_data_vendors"] = str(self.third_party_ad_vendors).replace("'", "\"")
        return cookies


    def shuffle_conv_params(self):
        self.storage_consent = bool(random.getrandbits(1))
        self.ad_consent = bool(random.getrandbits(1))
        self.image_gen = bool(random.getrandbits(1))
        sample_size = random.randint(0, len(VENDORS))
        self.third_party_ad_vendors = random.sample(VENDORS, k=sample_size)

    def set_uid(self, uid):
        self.user_id = uid


class AnonymousUser(FastHttpUser):
    fixed_count = 1
    def on_start(self):
        self.current_chat = []
        self.cookies = CookieConfig()
        self.current_conv_id = None

    @task(10)
    def chat_llm(self):
        headers={'Content-type': 'application/json',
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        payload = {
            'user': 'anonymous',
            'conv_id': self.current_conv_id,
            'conversation': self.current_chat,
            'nb_token': 300
        }

        cookies = self.cookies.construct()
        resp = self.client.post(f"/chat", data= json.dumps(payload), headers=headers, cookies = cookies)
        resp.raise_for_status()
        resp_json = resp.json()
        self.current_conv_id = resp_json.get("db_uuid")

        # if st.session_state.current_conv_id is not None and st.session_state.current_conv_id not in st.session_state.history and st.session_state.is_authenticated:
        #     st.session_state.history.append(st.session_state.current_conv_id)
        self.current_chat.append(resp_json.get("infered_tokens"))


    @task(3)
    def start_new_chat(self):
        self.current_chat = []
        self.current_conv_id = None
        self.update_cookie_config()


    @task(3)
    def update_cookie_config(self):
        self.cookies.shuffle_conv_params()


class AuthenticatedUser(FastHttpUser):
    
    def on_start(self):
        self.current_chat = []
        self.cookies = CookieConfig()
        self.current_conv_id = None
        self.history = []
        self.__login("signup")



    def __login(self, login_type):
        characters = string.ascii_uppercase + string.digits
        random_string = ''.join(random.choice(characters) for i in range(8))
        self.username = random_string
        headers={'Content-type': 'application/json',
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = self.cookies.construct()
        payload={"username": f"{self.username}"}
        resp = self.client.post(f"/account/{login_type}", data=json.dumps(payload), headers=headers, cookies=cookies)
        cookies = self.cookies.construct()
        resp.raise_for_status()
        resp_json = resp.json()
        self.user_id = resp.json().get("uuid")
        self.cookies.set_uid(self.user_id)


    @task(2)
    def cycle_log(self):
        self.history = []
        self.current_chat = []
        self.current_conv_id = None
        self.__login("login")
        self.get_history()


    @task(3)
    def update_cookie_config(self):
        self.cookies.shuffle_conv_params()

    @task(5)
    def start_new_chat(self):
        self.current_chat = []
        self.current_conv_id = None
        self.update_cookie_config()
        self.chat_llm()



    @task(1)
    def get_history(self):
        headers={'Content-type': 'application/json',
                 # 'Accept': 'text/event-stream', 
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        cookies = self.cookies.construct()
        resp = self.client.get(f"/history/{self.user_id}", headers=headers, cookies=cookies) 
        resp.raise_for_status()
        history = resp.json().get("history_list")
        if history is not None:
            self.history = history


    @task(3)
    def load_conversation(self):
        if len(self.history) > 0:
            target_conv = random.choice(self.history)
            headers={'Content-type': 'application/json',
                     'Connection': 'keep-alive',
                     'X-Accel-Buffering': 'no'}

            cookies = self.cookies.construct()
            cookies = self.cookies.construct()
            resp = self.client.get(f"/c/{target_conv}", data= json.dumps(payload), headers=headers, cookies = cookies)
            resp.raise_for_status()
            conversation = resp.json().get("conv")
            if conversation is not None:
                self.current_chat = conversation
                self.current_conv_id = target_conv


    @task(2)
    def delete_conversation(self):
        if len(self.history) > 0:
            target_conv = random.choice(self.history)
            headers={'Content-type': 'application/json',
                     'Connection': 'keep-alive',
                     'X-Accel-Buffering': 'no'}

            cookies = self.cookies.construct()
            self.client.post(f"/history/delete/{target_conv}",cookies=cookies) 
            self.history.remove(target_conv)

    

    @task(10)
    def chat_llm(self):
        headers={'Content-type': 'application/json',
                 'Connection': 'keep-alive',
                 'X-Accel-Buffering': 'no'}

        self.current_chat.append({"role": "user", "content": "This a test prompt from the user"})
        payload = {
            'user': self.username,
            'conv_id': self.current_conv_id,
            'conversation': self.current_chat,
            'nb_token': 300
        }

        cookies = self.cookies.construct()
        resp = self.client.post(f"/chat", data= json.dumps(payload), headers=headers, cookies = cookies)
        resp.raise_for_status()
        resp_json = resp.json()
        self.current_conv_id = resp_json.get("db_uuid")

        if self.current_conv_id is not None and self.current_conv_id not in self.history:
            self.history.append(self.current_conv_id)
        self.current_chat.append(resp_json.get("infered_tokens"))

