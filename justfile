alias b:= build

build:
  cargo build --release

setup_database:
  mkdir -p {{justfile_dir()}}/runtime/db_server/
  cp {{justfile_dir()}}/target/release/db_server {{justfile_dir()}}/runtime/db_server
  cp -r {{justfile_dir()}}/db_server/resources/ {{justfile_dir()}}/runtime/db_server

setup_llm:
  mkdir -p {{justfile_dir()}}/runtime/llm_server/
  cp {{justfile_dir()}}/target/release/llm_server {{justfile_dir()}}/runtime/llm_server

setup_ads:
  mkdir -p {{justfile_dir()}}/runtime/ad_server/
  cp {{justfile_dir()}}/target/release/ad_server {{justfile_dir()}}/runtime/ad_server

export_certificates CERTIFICATE_DIR:
  mkdir -p {{CERTIFICATE_DIR}}
  cp -r {{justfile_dir()}}/certificates/* {{CERTIFICATE_DIR}}

#Setup runtime folders for all services
setup_services:
  mkdir -p {{justfile_dir()}}/runtime/keys
  @just setup_database
  @just setup_llm
  @just setup_ads
  @just setup_webserver

setup_webserver:
  #Important line, as we need to rebuild webservers without the "server" feature on service crates
  #or else we require a FIFO and whatnot...
  cd {{justfile_dir()}}/webserver/ && cargo build --release
  mkdir -p {{justfile_dir()}}/runtime/webserver/
  cp {{justfile_dir()}}/target/release/webserver {{justfile_dir()}}/runtime/webserver
  cp {{justfile_dir()}}/webserver/client_attestation_config.toml {{justfile_dir()}}/runtime/webserver


run_client:
  cd {{justfile_dir()}}/client/ && streamlit run client.py

run_webserver:
  cd {{justfile_dir()}}/runtime/webserver/ && ./webserver


gen_sidecar_metadata:
  touch project_metadata.toml
  echo [binaries] > project_metadata.toml
  echo llm_server = {bin_path = \"{{justfile_dir()}}/runtime/llm_server/llm_server\", run_path = \"{{justfile_dir()}}/runtime/llm_server/\"} >> project_metadata.toml
  echo db_server = {bin_path = \"{{justfile_dir()}}/runtime/db_server/db_server\", run_path = \"{{justfile_dir()}}/runtime/db_server/\"} >> project_metadata.toml
  echo ad_server = {bin_path = \"{{justfile_dir()}}/runtime/ad_server/ad_server\", run_path = \"{{justfile_dir()}}/runtime/ad_server/\"} >> project_metadata.toml

  echo >> project_metadata.toml

  echo [service_mapping] >> project_metadata.toml
  echo llm_server = \"Inference\" >> project_metadata.toml
  echo db_server = \"Database\" >> project_metadata.toml
  echo ad_server = \"Advertisement\" >> project_metadata.toml


setup_part1:
  @just build
  @just setup_services
  @just gen_sidecar_metadata
  mkdir -p ./certificates/

setup_part2:
  @just export_certificates {{justfile_dir()}}/runtime/certificates/

clean:
  rm -r ./runtime
  rm -r ./target
  rm -r ./certificates
  rm -r ./policy_hashes
  rm ./project_metadata.toml
  rm ./attest.log


