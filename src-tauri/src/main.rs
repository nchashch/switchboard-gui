#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use anyhow::Result;
use clap::Parser;
use futures::executor::block_on;
use serde_json::Value;
use std::path::{Path, PathBuf};
use switchboard::{config::Config, launcher};
use tauri::{RunEvent, WindowEvent};
use web3::Transport;

struct Main(ureq_jsonrpc::Client);
struct Testchain(ureq_jsonrpc::Client);
struct BitAssets(ureq_jsonrpc::Client);
struct BitNames(ureq_jsonrpc::Client);
struct Zcash(ureq_jsonrpc::Client);
struct Web3(web3::transports::Http);

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct FullConfig {
    config: Config,
    datadir: PathBuf,
}

#[tauri::command]
fn get_config(config: tauri::State<'_, FullConfig>) -> FullConfig {
    (*config).clone()
}

#[tauri::command]
fn get_geth_console_command(command: tauri::State<'_, GethConsole>) -> String {
    command.0.clone()
}

#[tauri::command]
async fn mainchain(
    client: tauri::State<'_, Main>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn testchain(
    client: tauri::State<'_, Testchain>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn bitassets(
    client: tauri::State<'_, BitAssets>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn bitnames(
    client: tauri::State<'_, BitNames>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| format!("{}", err))
}

#[tauri::command]
async fn zcash(
    client: tauri::State<'_, Zcash>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .send_request(method, &params)
        .map_err(|err| match err {
            ureq_jsonrpc::Error::Ureq(ureq::Error::Transport(transport)) => {
                format!("{}", transport.message().unwrap())
            }
            ureq_jsonrpc::Error::Rpc(ureq_jsonrpc::RpcError { message, .. }) => {
                format!("{}", message)
            }
            err @ _ => format!("{:?}", err),
        })
}

#[tauri::command]
async fn web3(
    client: tauri::State<'_, Web3>,
    method: &str,
    params: Vec<ureq_jsonrpc::Value>,
) -> Result<Value, String> {
    client
        .0
        .execute(method, params)
        .await
        .map_err(|err| format!("{}", err))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let datadir = {
        let home_dir = dirs::home_dir().unwrap();
        let sb_dir = home_dir.join(".switchboard");
        args.datadir.unwrap_or(sb_dir)
    };
    let config: Config = confy::load_path(datadir.join("config.toml"))?;
    if !datadir.join("data").exists() {
        std::fs::create_dir_all(datadir.join("data/main"))?;
        std::fs::create_dir_all(datadir.join("data/testchain"))?;
        std::fs::create_dir_all(datadir.join("data/bitassets"))?;
        std::fs::create_dir_all(datadir.join("data/bitnames"))?;
        std::fs::create_dir_all(datadir.join("data/ethereum"))?;
        std::fs::create_dir_all(datadir.join("data/zcash"))?;
        ethereum_regtest_setup(&datadir)?;
        zcash_setup(&datadir)?;
    }
    let geth_console = {
        let ipc_file = datadir.join("data/ethereum/geth.ipc");
        let geth_bin = datadir.join("bin/geth");
        format!("{} attach {}", geth_bin.display(), ipc_file.display())
    };
    let main_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.main.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let testchain_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.testchain.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let bitassets_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.bitassets.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let bitnames_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.bitnames.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let zcash_client = ureq_jsonrpc::Client {
        host: "localhost".to_string(),
        port: config.zcash.port,
        user: config.switchboard.rpcuser.clone(),
        password: config.switchboard.rpcpassword.clone(),
        id: "switchboard-gui".to_string(),
    };
    let web3_client =
        web3::transports::Http::new(&format!("http://localhost:{}", config.ethereum.port))?;
    // let mut daemons = Daemons::start(&url, &datadir, &config)?;
    let app = tauri::Builder::default()
        .manage(FullConfig { config, datadir })
        .manage(Main(main_client.clone()))
        .manage(Testchain(testchain_client.clone()))
        .manage(BitAssets(bitassets_client.clone()))
        .manage(BitNames(bitnames_client.clone()))
        .manage(Zcash(zcash_client.clone()))
        .manage(Web3(web3_client))
        .manage(GethConsole(geth_console))
        .invoke_handler(tauri::generate_handler![
            get_config,
            get_geth_console_command,
            mainchain,
            testchain,
            bitassets,
            bitnames,
            zcash,
            web3
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::Exit => {
            kill_ethereum();
            testchain_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            bitassets_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            bitnames_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            zcash_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
            main_client.send_request::<ureq_jsonrpc::Value>("stop", &[]);
        }
        _ => {}
    });
    Ok(())
}

struct GethConsole(String);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    datadir: Option<PathBuf>,
    #[arg(short, long)]
    bin_download_url: Option<String>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("ureq jsonrpc error")]
    Ureq(#[from] ureq_jsonrpc::Error),
    #[error("web3 error")]
    Web3(#[from] web3::Error),
}

fn kill_ethereum() -> Result<(), String> {
    println!("killing ethereum process");
    let pid = std::process::Command::new("pgrep")
        .args(["--oldest", "geth"])
        .output()
        .map_err(|err| format!("{}", err))?;
    let pid = std::str::from_utf8(&pid.stdout)
        .map_err(|err| format!("{}", err))?
        .trim();
    let pid: u32 = pid.parse().map_err(|err| format!("{}", err))?;
    std::process::Command::new("kill")
        .args(["-s", "INT", &pid.to_string()])
        .spawn()
        .map_err(|err| format!("{}", err))?
        .wait()
        .map_err(|err| format!("{}", err))?;
    Ok(())
}

pub fn ethereum_regtest_setup(datadir: &Path) -> Result<()> {
    const GENESIS_JSON: &str = r#"
{
"config": {
    "chainId": 5123,
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "constantinopleBlock": 0,
    "petersburgBlock": 0,
    "istanbulBlock": 0,
    "berlinBlock": 0
},
"difficulty": "0",
"gasLimit": "21000000",
"alloc": {
    "0xc96aaa54e2d44c299564da76e1cd3184a2386b8d":
    { "balance": "21000000000000000000000000"}
}
}
"#;

    let ethereum_datadir = datadir.join("data/ethereum");
    std::fs::create_dir_all(&ethereum_datadir)?;
    let genesis_json_path = ethereum_datadir.join("genesis.json");
    std::fs::write(&genesis_json_path, GENESIS_JSON)?;
    // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
    let (mut rx, mut child) = tauri::api::process::Command::new_sidecar("geth")?
        .args(&[
            format!("--datadir={}", ethereum_datadir.display()),
            "init".into(),
            format!("{}", genesis_json_path.display()),
        ])
        .spawn()?;
    Ok(())
}

pub fn zcash_setup(datadir: &Path) -> Result<()> {
    let zcash_datadir = datadir.join("data/zcash");
    std::fs::create_dir_all(datadir.join(&zcash_datadir))?;
    let zcash_conf_path = zcash_datadir.join("zcash.conf");
    let zcash_conf = "nuparams=5ba81b19:1
nuparams=76b809bb:1";
    dbg!(&zcash_conf_path);
    std::fs::write(zcash_conf_path, zcash_conf)?;
    tauri::api::process::Command::new_sidecar("fetch-params.sh")?.spawn()?;
    Ok(())
}
