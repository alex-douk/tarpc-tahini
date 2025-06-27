# tarpc tahini
This branch explores a prototype for 
a Tahini wrapper around a tarpc service.


# Examples

## LLM example

This example is made of five total components :

- `llm_server` : Simple LLM inference server. 

- `db_server`: Database server that stores conversations, user privacy preferences and user login information.

- `ad_server`: A service outside of the main "company" that serves ads based on prompt content.

- `webserver`: Webserver handling coordination between services and the end-client

- `client`: Python webapp running locally. Provides an interface to the webserver. Requires `streamlit`.

## Run

Use the justfile provided to setup the project to work with [Tahini](https://github.com/alex-douk/tahini_lib).

Usage:
```just
#Compiles all services
#Puts them in a common runtime directory
#Generate project metadata for sidecar
#This can be adapted for separate deployment configurations
just setup_part1


#========================================
#You should call tahini_lib's justfile toolchain at this moment
#========================================

# Make certificates available to runtime
just setup_part2

#Launch webserver
just run_webserver

#Launch python client 
just run_client
```
