use ethers::prelude::k256::Secp256k1;
use ethers::prelude::{*, k256::pkcs8::spki::DynAssociatedAlgorithmIdentifier};
use ethers::prelude::k256::ecdsa::SigningKey;
use serde_json::{json, Value};
use actix_web::{get,post, web, App, HttpServer, Responder,HttpResponse};
use actix_files as fs;
use std::thread;
use futures::{
    future::FutureExt,
    pin_mut,
    select,
};
use teloxide::prelude::*;
use std::convert::TryFrom;
use std::sync::Arc;
use chrono::prelude::*;
use std::time::{Duration,SystemTime,Instant, UNIX_EPOCH};
use serde::Deserialize;
use rand::rngs::OsRng;
mod ERC20;
mod routeruni;
mod unipoolgest;
mod unifact;
mod unipooladdr;
mod oracle;
static mut SELECSIS:usize = 0;
static mut SELEC:usize = 0;
static mut DIAS:u64 = 0;
static mut WRIR:bool = true;
static mut SOLO:bool = false;
static mut APIS:bool = false;
static mut TKEY:String = String::new();
static mut TID:ChatId = ChatId(0);
static mut KEY:String = String::new();
static REDES:&[&str] = &["etherium.json","polygon.json","arbitrum.json","optimism.json"];
static mut LINKS:Vec<String> = vec![];
async fn sistems(index: usize,timer:u64){
    if REDES.len() == unsafe{LINKS.len()} && unsafe{WRIR == true}{
        unsafe{SELEC = index;}
        let file:&str = unsafe{REDES[SELEC]};
        let (uni_router,unipoolgestaddr,tempo,factory,gwei,_) = contantes(file);
            let mut exclu:Vec<usize> = vec![];
            let provedor0 = provedor(unsafe{&KEY},unsafe{&LINKS[index]});
            let provider = provedor0.await;
            match provider {
                Ok(provider0) =>{
                    let publicaddr = provider0.address();
                    let ba = provider0.get_balance(publicaddr, None).await.unwrap();
                        if ba > U256::zero(){
                            save_addr(&file,&provider0).await;
                            thread::sleep(Duration::from_secs(timer));
                            let mut jsons = {
                                let text = std::fs::read_to_string(&file).unwrap();
                                serde_json::from_str::<Value>(&text).unwrap()
                            };
                            let acoes: Vec<String> = serde_json::from_str(&jsons["agenda"].to_string()).unwrap();
                            let diatual: u32 = serde_json::from_str(&jsons["diatual"].to_string()).unwrap();
                            let dates = Utc::today();
                            let day:u32  = dates.day();
                            let weck:u32  = dates.weekday().num_days_from_monday();
                            if acoes.len() > 0 && unsafe{APIS == true}{
                                let (namesg,pricesg,balancesg,_,_,_,addr,_,_,_,_,_) = prc_name(file,"geral");
                                let (namesa,pricesa,balancesa,_,acao,callput,_,tokenid,token0,token1,dias,modos) = prc_name(file,"agenda");
                                for i in 0..namesa.len(){
                                    let mut name = &namesa[i];
                                    if name == "-"{name = &namesg[0]}
                                    let nun = namesg.iter().position(|s| s == name).unwrap();
                                    if pricesg[nun] >= pricesa[i] && callput[i] == true && modos[i] == "price" ||
                                    pricesg[nun] <= pricesa[i] && callput[i] == false && modos[i] == "price" || 
                                    diatual != day && modos[i] == "dca" || diatual != day && weck == 0 && modos[i] == "dcawk"{
                                        if acao[i] == "swap".to_string(){
                                            let token0nun = namesg.iter().position(|s| s == &token0[i]).unwrap();
                                            if balancesa[i] <= balancesg[token0nun] && balancesg[token0nun] > U256::from(0) && balancesa[i] > U256::from(0){
                                                let token1nun = namesg.iter().position(|s| s == &token1[i]).unwrap();
                                                jsons["operacao"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",i+1).as_str()).unwrap();
                                                unsafe{WRIR = false;}
                                                std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap(),).unwrap();
                                                println!("iniciado troca..");
                                                let swap = simple_swap(addr[token0nun], addr[token1nun], uni_router, balancesa[i], factory, tempo,gwei,&provider0).await;
                                                if swap == true{
                                                    if dias[i]-1 == 0{
                                                        let mut sb = "".to_string();
                                                        if modos[i] == "price"{sb = "preço".to_string()}
                                                        if modos[i] == "dca"{sb = "dia".to_string()}
                                                        if modos[i] == "dcawk"{sb = "Semana".to_string()}
                                                        let status = format!("foi realizada a °{} Operação agendada por {} na rede {} que trocou {} por {}",i+1,sb,&file.replace(".json", ""),token0[i],token1[i]);
                                                        if unsafe{TKEY.clone()} != "" && unsafe{TID} != ChatId(0) && unsafe{SOLO} == true{
                                                            for _ in 0..10{
                                                                let bot = Bot::new(unsafe{TKEY.clone()});
                                                                let mensagem = bot.send_message(unsafe{TID}, &status).await;
                                                                match mensagem {
                                                                    Ok(_) =>{break;},Err(e) =>println!("Mensagem não enviada {e}")
                                                                }
                                                            }
                                                        }
                                                        println!("{}",status);
                                                        exclu.push(i);
                                                    }else{
                                                        let viws: String = serde_json::from_str(&jsons["agenda"][i].to_string()).unwrap();
                                                        let mut newdata: Value = serde_json::from_str(&viws).unwrap();
                                                        newdata["dias"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",dias[i]-1).as_str()).unwrap();
                                                        jsons["agenda"][i] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",newdata.to_string()).as_str()).unwrap();
                                                        std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap()).unwrap();
                                                    }
                                                    jsons["operacao"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",0).as_str()).unwrap();
                                                    std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap(),).unwrap();  
                                                }
                                                unsafe{WRIR = true;}
                                            }else{exclu.push(i);println!("Balanço insuficiente para troca da {} Ação,retirando ação em breve..",i+1)}
                                        }
                                        if acao[i] == "stop".to_string(){
                                            let poolinfo = unipoolgest::unipoolgest::unipoolgest::new(unipoolgestaddr, provider0.clone());
                                            let poolres = poolinfo.positions(tokenid[i]).await;
                                            match poolres {
                                                Ok(pool) =>{
                                                    let lincy:u128 = pool.7;
                                                    if U256::from(lincy) > U256::zero(){
                                                        match poolinfo.owner_of(tokenid[i]).call().await {
                                                            Ok(onerpool) => {
                                                                if onerpool == publicaddr{
                                                                    let token0nun = namesg.iter().position(|s| s == &token0[i]).unwrap();
                                                                    let tokens:Vec<Address> = vec![pool.2,pool.3];
                                                                    jsons["operacao"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",i+1).as_str()).unwrap();
                                                                    unsafe{WRIR = false;}
                                                                    std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap()).unwrap();
                                                                    let mut balancion:U256 = balancesa[i];
                                                                    if U256::from(lincy) < balancion{
                                                                        balancion = U256::from(lincy);
                                                                    }
                                                                    println!("iniciado Stop Pool..");
                                                                    let rmv = simple_rmv_pool(tokenid[i],balancion.as_u128(),tokens,
                                                                    addr[token0nun],unipoolgestaddr,uni_router,factory,tempo,gwei,&provider0).await;
                                                                    if rmv == true{
                                                                        if dias[i]-1 == 0{
                                                                            let mut sb = "".to_string();
                                                                            if modos[i] == "price"{sb = "preço".to_string()}
                                                                            if modos[i] == "dca"{sb = "dia".to_string()}
                                                                            if modos[i] == "dcawk"{sb = "Semana".to_string()}
                                                                            let status = format!("foi realizada a °{} Operação agendada por {} na rede {} em sua piscina de liquidez com id: {}",i+1,sb,&file.replace(".json", ""),tokenid[i]);
                                                                            if unsafe{TKEY.clone()} != "" && unsafe{TID} != ChatId(0) && unsafe{SOLO} == true{
                                                                                for _ in 0..10{
                                                                                    let bot = Bot::new(unsafe{TKEY.clone()});
                                                                                    let mensagem = bot.send_message(unsafe{TID}, &status).await;
                                                                                    match mensagem {
                                                                                        Ok(_) =>{break;},Err(e) =>println!("Mensagem não enviada {e}")
                                                                                    }
                                                                                }
                                                                            }
                                                                            println!("{}",status);
                                                                            exclu.push(i);
                                                                        }else{
                                                                            let viws: String = serde_json::from_str(&jsons["agenda"][i].to_string()).unwrap();
                                                                            let mut newdata: Value = serde_json::from_str(&viws).unwrap();
                                                                            newdata["dias"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",dias[i]-1).as_str()).unwrap();
                                                                            jsons["agenda"][i] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",newdata.to_string()).as_str()).unwrap();
                                                                            std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap()).unwrap();
                                                                        }
                                                                        jsons["operacao"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",0).as_str()).unwrap();
                                                                        std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap(),).unwrap();
                                                                    }
                                                                    unsafe{WRIR = true;}
                                                                }
                                                            },Err(_) =>println!("dono da pool não identificado tentando novamente")
                                                        }
                                                    }else{exclu.push(i);println!("Balanço insuficiente para recolher liquidez na {} Ação,retirando ação em breve..",i+1)}
                                                },Err(_) =>println!("erro ao acessar a pool..")
                                            }
                                        }
                                    }
                                }
                            unsafe{WRIR = false;}
                            jsons["diatual"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",day).as_str()).unwrap();
                            std::fs::write(&file,serde_json::to_string_pretty(&jsons).unwrap(),).unwrap();
                            unsafe{WRIR = true;}
                            remove_acao(&file,"agenda",exclu);
                        }
                    }
                }
                ,Err(_) =>println!("Erro conecção tentando novamente..")
            }
        }
    unsafe{WRIR = true;}
    thread::sleep(Duration::from_secs(timer));
}
async fn sistema(){
    if unsafe{TKEY.clone()} != "" && unsafe{TID} != ChatId(0) && unsafe{SOLO} == true{
        for _ in 0..10{
            let bot = Bot::new(unsafe{TKEY.clone()});
            let mensagem = bot.send_message(unsafe{TID}, "AutoUniV1 iniciado!").await;
            match mensagem {
                Ok(_) =>{println!("iniciado por telegram..");break;}
                ,Err(e) =>println!("Mensagem não enviada {e}")
            }
        }
    }else{println!("telegram desativado.")}
    let mut trocadia = false;
    println!("iniciado em http://localhost:3000/");
    loop{
        sistems(0,70).await;
        sistems(1,70).await;
        sistems(2,70).await;
        sistems(3,70).await;
    }
}
#[derive(Deserialize)]
struct Info {
    acao: String,
    name: String,
    price: f64,
    balance: String,
    quantia: String,
    token0: String,
    token1: String,
    tokenid: u64,
    callput: bool,
    dias: u64,
    modo:String
}
#[derive(Deserialize)]
struct Nuns {nun: u32}

#[derive(Deserialize)]
struct new_cont {cont:String,oracle:String}
#[derive(Deserialize)]
struct Del {posi:usize,rede:usize}

#[derive(Deserialize)]
struct Parvw {token0:String,token1:String}

#[get("/json/{name}")]
async fn jsons_get(name: web::Path<String>) -> impl Responder {
    let namejson = name.to_string()+".json";
    let jsons = {
        let text = std::fs::read_to_string(namejson.clone()).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    unsafe{SELECSIS = REDES.iter().position(|s| s == &namejson).unwrap();}
    format!("{:#?}",jsons.to_string())
}
#[post("/exclui_acao")]
async fn exclui_acao(form: web::Form<Del>) -> impl Responder {
    unsafe{remove_acao(&REDES[form.rede],"agenda",vec![form.posi]);}
    format!("{:?}","")
}
#[post("/par_view")]
async fn par_view(form: web::Form<Parvw>) -> HttpResponse {
    let mut res = "Sem liquidez..".to_string();
    let mut nome = "".to_string();
    let file = unsafe{REDES[SELECSIS]};
    let (_,_,_,factory,_,_) = contantes(file);
    let (namesg,pricesg,_,dec,_,_,addr,_,_,_,_,_) = prc_name(file,"geral");
    let mut price = 0.0;let mut decimal:u8 = 0;
    let mut esse:Address = "0x0000000000000000000000000000000000000000".parse().unwrap();
    let mut paraesse:Address = "0x0000000000000000000000000000000000000000".parse().unwrap();
    for name in 0..namesg.len(){
        if namesg[name] == form.token0{
            esse = addr[name];
            break;
        }
    }
    for name in 0..namesg.len(){
        if namesg[name] == form.token1{
            nome  = namesg[name].clone();
            paraesse = addr[name];
            price = pricesg[name];
            decimal = dec[name];
            break;
        }
    }
    if esse != "0x0000000000000000000000000000000000000000".parse().unwrap() &&
    paraesse != "0x0000000000000000000000000000000000000000".parse().unwrap(){
        let provedor = provedor(unsafe{&KEY},unsafe{&LINKS[SELECSIS]});
        let provider0 = provedor.await;
        match provider0 {
            Ok(provider) =>{
                let (_,libera,balanco) = feende(factory,esse,paraesse,&provider).await;
                if libera == true{
                    let vv = calculate_value(balanco,price,decimal);
                    let valor = (vv * 100.0).trunc() / 100.0;
                    res = format!("O token {nome} possui ${valor} de liquidez para essa troca.");
                }
            },Err(_) =>println!("Erro conecção tente novamente..")
        }
    }
    HttpResponse::Ok().body(format!("{res}"))
}

#[post("/poll_nun")]
async fn submit_poll_nun(form: web::Form<Nuns>) -> HttpResponse {
    let file:&str = unsafe{REDES[SELECSIS]};
    let mut token0 = "".to_string();let mut token1 = "".to_string();
    let mut go:bool = false;let mut lincy:u128 = 0;
    if unsafe{WRIR == true}{
        unsafe{WRIR = false};
        let (_,unipoolgestaddr,_,_,_,_) = contantes(file);
        let provedor = provedor(unsafe{&KEY},unsafe{&LINKS[SELECSIS]});
        let provider0 = provedor.await;
        match provider0 {
            Ok(provider) =>{
                let publicaddr = provider.address();
                let poolinfo = unipoolgest::unipoolgest::unipoolgest::new(unipoolgestaddr, provider.clone());
                let poolsts = poolinfo.positions(U256::from(form.nun)).await;
                match poolsts {
                    Ok(pool) =>{
                            let balnn = balancofull(pool.2, &provider).await;
                            match balnn {
                                Ok((tk,_,_,_)) =>{
                                    token0 = tk;
                                },Err(_) =>println!("erro ao ler a pool..")
                            }
                            let balnn = balancofull(pool.3, &provider).await;
                            match balnn {
                                Ok((tk,_,_,_)) =>{
                                    token1 = tk;
                                    lincy = pool.7;
                                    match poolinfo.owner_of(U256::from(form.nun)).call().await {
                                        Ok(onerpool) => {
                                            if onerpool == publicaddr{go = true}
                                        },Err(_) =>println!("dono da pool não identificado tentando novamente")
                                    }
                                },Err(_) =>println!("erro ao ler a pool..")
                            }
                    },Err(_) =>{}
                }   
            },
            Err(_) =>println!("Erro conecção tentando novamente..")
        }
        unsafe{WRIR = true};
    }
    HttpResponse::Ok().body(format!("{{\"lincy\":{:?},\"token0\":\"{}\",\"token1\":\"{}\",\"go\":{},\"tokenid\":{:?}}}",lincy,token0,token1,go,form.nun))
}
#[post("/submit_cont")]
async fn submit_cont(form: web::Form<new_cont>) -> HttpResponse {
    let addrs = form.cont.parse::<Address>();
    let addro = form.oracle.parse::<Address>();
    let mut name = "".to_string();let mut res = "".to_string();
    match addro {
        Ok(addroracle) =>{
            match addrs {
                Ok(addr) =>{
                    let provedor = provedor(unsafe{&KEY},unsafe{&LINKS[SELECSIS]});
                    let provider0 = provedor.await;
                    match provider0 {
                        Ok(provider) =>{
                            let cont = ERC20::ERC20::new(addr, provider.clone());
                            match cont.symbol().await {
                                Ok(nn) =>{
                                    name = nn;
                                    let file:&str = unsafe{REDES[SELECSIS]};
                                    let mut jsons = {
                                        let text = std::fs::read_to_string(&file).unwrap();
                                        serde_json::from_str::<Value>(&text).unwrap()
                                    };
                                    let mut geral: Vec<String> = serde_json::from_str(&jsons["contratos"].to_string()).unwrap();
                                    let mut geral_o: Vec<String> = serde_json::from_str(&jsons["oracles"].to_string()).unwrap();
                                    if geral.contains(&format!("{:?}",addr)){
                                        res = "contrato já existe.".to_string();
                                    }else{
                                        geral.push(format!("{:?}",addr));
                                        geral_o.push(format!("{:?}",addroracle));
                                        jsons["contratos"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",geral).as_str()).unwrap();
                                        jsons["oracles"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",geral_o).as_str()).unwrap();
                                        if unsafe{WRIR == true}{
                                            unsafe{WRIR = false};
                                                std::fs::write(
                                                    &file,
                                                    serde_json::to_string_pretty(&jsons).unwrap(),
                                                ).unwrap();
                                            }
                                            unsafe{WRIR = true};
                                        res = format!("Contrato do token: {name} adicionado!");
                                        save_addr(&file,&provider).await;
                                        println!("{res}");
                                    }
                                },Err(_) => {res = "contrato invalido.".to_string();}
                            }
                        },
                        Err(_) =>println!("Erro conecção tentando novamente..")
                    }
                },Err(_) =>{res = "contrato invalido.".to_string();}
            }
        },Err(_) =>{res = "oraculo invalido.".to_string();}
    }
    HttpResponse::Ok().body(format!("{res}"))
}
#[post("/submit")]
async fn submit_form(form: web::Form<Info>) -> HttpResponse {
    let json_data = json!({
        "acao": form.acao,
        "name": form.name,
        "price": form.price,
        "balance": form.balance,
        "quantia": form.quantia,
        "token0": form.token0,
        "token1": form.token1,
        "tokenid": form.tokenid,
        "callput": form.callput,
        "dias": form.dias,
        "modo":form.modo
    });
    let mut html = "";
    let swapjson = json_data.to_string();
    let file:&str = unsafe{REDES[SELECSIS]};
    let balance = format!("{}",form.balance.replace('"',"").replace('.',"")).parse::<u128>();
    match balance{
        Ok(_) =>{
            if unsafe{WRIR == true}{
                unsafe{WRIR = false};
                let mut jsons = {
                    let text = std::fs::read_to_string(&file).unwrap();
                    serde_json::from_str::<Value>(&text).unwrap()
                };
                let mut geral: Vec<String> = serde_json::from_str(&jsons["agenda"].to_string()).unwrap();
                geral.push(swapjson);
                jsons["agenda"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",geral).as_str()).unwrap();
                std::fs::write(
                    &file,
                    serde_json::to_string_pretty(&jsons).unwrap(),
                ).unwrap();
                unsafe{WRIR = true};
                html = r#"<!DOCTYPE html>
                <html>
                <head>
                <meta charset="UTF-8">
                <meta http-equiv="X-UA-Compatible" content="IE=edge">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Agendamento Enviado</title>
                    <style>
                        body {
                            background: linear-gradient(45deg, #ff008a, #ffc0cb);
                            overflow: hidden;
                            text-align: center;
                            width: 100%;
                            height: 100vh;
                        }
                        .message {
                            font-family: 'Monument Ultrabold';
                            text-transform: uppercase;
                            color: #fff;
                            box-shadow: 0 2px 0 #fff;
                            font-size: 20px;
                            font-weight: 700;
                            letter-spacing: 3px;
                            padding: 10px;
                            transition: .6s;
                        }
                        .button {margin-top: 20px;}
                        .button a:hover {background-color: #45a049;}
                        .button a {
                            display: inline-block;
                            padding: 10px 20px;
                            background-color: #4CAF50;
                            color: white;
                            text-decoration: none;
                            border-radius: 4px;
                            transition: .6s;
                        }
                    </style>
                </head>
                <body>
                    <div class="message"><p>Transação agendada com sucesso!</p></div>
                    <div class="button"><a href="/">Retornar à página inicial</a></div>
                </body>
                </html>"#;
            }
        },Err(_) =>println!("balanço invalido..")
    }
    HttpResponse::Ok().body(html)
}
async fn server(){
    let jsons = {
        let text = std::fs::read_to_string(&"info.json").unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let porta: u16 = serde_json::from_str(&jsons["porta"].to_string()).unwrap();
    HttpServer::new(|| {
        App::new()
        .service(submit_cont)
        .service(exclui_acao)
        .service(par_view)
        .service(submit_poll_nun)
        .service(submit_form)
        .service(jsons_get)
        .service(fs::Files::new("/", "staticos").index_file("index.html"))
    })
    .bind(("127.0.0.1", porta)).unwrap()
    .run()
    .await.unwrap();
}
/*




    _         _        _   _       _  __     ___ 
   / \  _   _| |_ ___ | | | |_ __ (_) \ \   / / |
  / _ \| | | | __/ _ \| | | | '_ \| |  \ \ / /| |
 / ___ \ |_| | || (_) | |_| | | | | |   \ V / | |
/_/   \_\__,_|\__\___/ \___/|_| |_|_|    \_/  |_|
                                                  
| __ ) _   _ 
|  _ \| | | |
| |_) | |_| |
|____/ \__, |
       |___/ 
   ____      _        _       _ _            
  / ___|__ _| |_ _ __(_)_ __ (_) | __ _ _ __ 
 | |   / _` | __| '__| | '_ \| | |/ _` | '__|
 | |__| (_| | |_| |  | | |_) | | | (_| | |   
  \____\__,_|\__|_|  |_| .__/|_|_|\__,_|_|   
                       |_|                   




 */
#[tokio::main]
async fn main() {

    let permit_polygon:Vec<Address> = vec![
        "0x2791bca1f2de4661ed88a30c99a7a9449aa84174".parse().unwrap(),
        "0xc2132D05D31c914a87C6611C10748AEb04B58e8F".parse().unwrap(),
        "0x8f3cf7ad23cd3cadbd9735aff958023239c6a063".parse().unwrap()
    ];
    let permit_arbitrum:Vec<Address> = vec![
        "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1".parse().unwrap(),
        "0xfd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9".parse().unwrap(),
        "0xFF970A61A04b1cA14834A43f5dE4533eBDDB5CC8".parse().unwrap()
    ];
    let permit_etherium:Vec<Address> = vec![
        "0x6b175474e89094c44da98b954eedeac495271d0f".parse().unwrap(),
        "0xdac17f958d2ee523a2206206994597c13d831ec7".parse().unwrap(),
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse().unwrap()
    ];
    let permit_optimism: Vec<Address> = vec![
        "0x94b008aA00579c1307B0EF2c499aD98a8ce58e58".parse().unwrap(),
        "0x7F5c764cBc14f9669B88837ca1490cCa17c31607".parse().unwrap(),
        "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1".parse().unwrap()
    ];
    let mut txent = "".to_string();
    let mut nun: usize = 0;
    for j in 0..unsafe{REDES.len()}{
        let file:&str = unsafe{REDES[j]};
        let jsons = {
            let text = std::fs::read_to_string(&file).unwrap();
            serde_json::from_str::<Value>(&text).unwrap()
        };
        let tx_pagamento: String = serde_json::from_str(&jsons["tx_hash"].to_string()).unwrap();
        let rcp: String = serde_json::from_str(&jsons["rcp"].to_string()).unwrap();
        unsafe{
            LINKS.push(rcp)
        }
        let rede_pagamento: usize = serde_json::from_str(&jsons["rede_tx"].to_string()).unwrap();
        if tx_pagamento != ""{txent = tx_pagamento;nun = rede_pagamento}
    }
    let mut keyjson = {
        let text = std::fs::read_to_string("info.json").unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let key: String = serde_json::from_str(&keyjson["key"].to_string()).unwrap();
    let tkey: String = serde_json::from_str(&keyjson["telegram_token"].to_string()).unwrap();
    let tid: ChatId = serde_json::from_str(&keyjson["telegram_id"].to_string()).unwrap();
    let termos: String = serde_json::from_str(&keyjson["termos"].to_string()).unwrap();
    let aceito: bool = serde_json::from_str(&keyjson["aceito_termos"].to_string()).unwrap();
    if aceito == false{
        loop {
            println!("{termos}");
            println!("Digite 's' para aceitar ou 'n' para negar: ");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "s" {
                keyjson["aceito_termos"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",true).as_str()).unwrap();
                std::fs::write(&"info.json",serde_json::to_string_pretty(&keyjson).unwrap()).unwrap();
                break;
            } else if input == "n" {
                keyjson["aceito_termos"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",false).as_str()).unwrap();
                std::fs::write(&"info.json",serde_json::to_string_pretty(&keyjson).unwrap()).unwrap();
                break;
            } else {
                println!("Entrada inválida. Tente novamente.");
            }
        }
    }
    if key == ""{
        let wallet = Wallet::new(&mut OsRng);
        keyjson["key"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",wallet.signer().as_nonzero_scalar().to_string()).as_str()).unwrap();
        std::fs::write(&"info.json",serde_json::to_string_pretty(&keyjson).unwrap()).unwrap();
        println!("Endereço da carteira AutoUni V1: {:?}", wallet.address());
        thread::sleep(Duration::from_secs(5));
    }
    let aceitoatual: bool = serde_json::from_str(&keyjson["aceito_termos"].to_string()).unwrap();
    let key: String = serde_json::from_str(&keyjson["key"].to_string()).unwrap();
    if key != "" && aceitoatual == true{
        println!(r#"
         _         _        _   _       _  __     ___ 
        / \  _   _| |_ ___ | | | |_ __ (_) \ \   / / |
       / _ \| | | | __/ _ \| | | | '_ \| |  \ \ / /| |
      / ___ \ |_| | || (_) | |_| | | | | |   \ V / | |
     /_/   \_\__,_|\__\___/ \___/|_| |_|_|    \_/  |_|                                                      
       "#);
        unsafe{KEY = key}
        if tkey != "" && tid != ChatId(0){
        unsafe{TKEY = tkey;TID = tid}}
        println!("Para ter acesso ao telegram substitua o telegram_code pela sua chave BotFather e telegram_id como seu ID pessoal..");
        let t1 = server().fuse();
        let t2 = sistema().fuse();
        pin_mut!(t1, t2);
        select! {
            () = t1 => println!("task one completed first"),
            () = t2 => println!("task two completed first"),
        }
    }
}
async fn simple_swap(esse:Address,paraesse:Address,router:Address,quantia:U256,factory:Address,tempo:u64,gwei:U256,
    provider0: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) -> bool{
        let mut success: bool = false;
        let (fee,libera,_) = feende(factory,esse,paraesse,provider0).await;
        let mut msg_aprov: String = "".to_string();let mut msg_status: bool = false;
        if libera == true{
            for _ in 0..10 {
                let appove = aprovar(esse, router,tempo,gwei,&provider0).await;
                match appove{
                    Ok((s,b)) =>{
                        msg_aprov = s;
                        msg_status = b;
                        break;
                    },Err(_) => {thread::sleep(Duration::from_secs(30));}
                }
            }
            if msg_status == true{
                println!("{}",msg_aprov);
                let mut msg_swap: String = "".to_string();let mut msg_status_w: bool = false;
                loop {
                    let trade = trocar(esse, paraesse,
                        router, fee, tempo, quantia,gwei,&provider0).await;
                    match trade{
                        Ok((s,b)) =>{
                            msg_swap = s;
                            msg_status_w = b;
                            break;
                        },Err(_) => {thread::sleep(Duration::from_secs(30));}
                    }
                }
                if msg_status_w == true{
                    success = true;
                    println!("{}",msg_swap);
                }else{
                    println!("{}",msg_swap);
                }
            }else {
                println!("{}",msg_aprov);
            }
        }else{println!("liquidez para a troca do par especificado insuficiente.")}
    return success;
}
async fn simple_rmv_pool(tokid:U256,liquidy:u128,tokens:Vec<Address>,safe:Address,poolmaneger:Address,router:Address,factory:Address,tempo: u64,gwei:U256,
    provider0: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) -> bool{
        let mut success = false;
        let mut tokensBA:Vec<U256> = vec![];
        let mut tokensBD:Vec<U256> = vec![];
        let mut tokensF:Vec<U256> = vec![];
        for token in tokens.clone(){
            loop {
                let balnn = balancofull(token, &provider0).await;
                match balnn {
                    Ok((_,_,balanco,_)) =>{
                        tokensBA.push(balanco);
                        break;
                    },Err(_) =>{}
                }
            }
        }
        let mut msg_aprov: String = "".to_string();let mut msg_status: bool = false;
        loop {
            let deslpol = desliga_pool(tokid,liquidy, poolmaneger,tempo,gwei,&provider0).await;
            match deslpol{
                Ok((s,b)) =>{
                    msg_aprov = s;
                    msg_status = b;
                    break;
                },Err(_) =>{thread::sleep(Duration::from_secs(30));}
            }
        }
        if msg_status == true{
            println!("{}",msg_aprov);
            let mut msg_swap: String = "".to_string();let mut msg_status_w: bool = false;
            loop {
                let rmvpol = rmvliq_pool(tokid, poolmaneger,tempo,gwei,&provider0).await;
                match rmvpol{
                    Ok((s,b)) =>{
                        msg_swap = s;
                        msg_status_w = b;
                        break;
                    },Err(_) => {thread::sleep(Duration::from_secs(30));}
                }
            }
            if msg_status_w == true{
                for token in tokens.clone(){
                    loop {
                        let balnn = balancofull(token, &provider0).await;
                        match balnn {
                            Ok((_,_,balanco,_)) =>{
                                tokensBD.push(balanco);
                                break;
                            },Err(_) =>{}
                        }
                    }
                }
                for i in 0..tokensBD.len(){
                    tokensF.push(tokensBD[i]-tokensBA[i]);
                }
                println!("{}",msg_swap);
                for i in 0..tokens.len(){
                    if safe != tokens[i]{
                        let ss = simple_swap(tokens[i], safe, router, tokensF[i], factory, tempo,gwei,&provider0).await;
                        if ss == true{
                            println!("token na posição {} da pool realizado",i);
                            success = true;
                        }else {
                            success = false;
                        }
                    }
                }
            }else{
                println!("{}",msg_swap);
            }
        }else {
            println!("{}",msg_aprov);
        }
    return success;
}
async fn aprovar(erc_20:Address,contrato:Address,tempo:u64,gwei:U256,
    provider0: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>)-> Result<(String,bool), Box<dyn std::error::Error>>{
    let mut aprovado:String = "".to_string();let  mut sucesso: bool = false;
    let publicaddr = provider0.address();
    loop{
        let cont = ERC20::ERC20::new(erc_20, provider0.clone());
        let balanco = cont.balance_of(publicaddr).call().await?;
        let transaction = cont.approve(contrato, balanco.clone()).from(publicaddr);
        let gas = transaction.estimate_gas().await?;
        let gasprice = provider0.get_gas_price().await?;
        let multgas = multiply_u256_by_f64(gas, 1.1);
        let multgasprice = multiply_u256_by_f64(gasprice, 1.1);
        let balnc: U256 = provider0.get_balance(publicaddr, None).await?;
        if (gasprice*multgas) > gwei || (gasprice*multgas) > balnc {aprovado = "Taxa superior ao limite disponivel aguarde a normalização da rede ou adcione mais tokens..".to_string();return Ok((aprovado,false));}
        let transaction_gas = transaction.gas_price(multgasprice).gas(multgas);
        let transaction_tx = transaction_gas.send().await;
        match transaction_tx{
            Ok(approve_tx) =>{
                let start = Instant::now();
                loop{
                    if start.elapsed().as_secs() > (tempo*60){
                        aprovado = "sessão espirou tentando novamente..".to_string();
                        println!("{aprovado}");
                        sucesso = false;break;
                    }
                    let receipt = provider0.get_transaction_receipt(approve_tx.tx_hash()).await;
                    match receipt {
                        Ok(rec) =>{
                            let (apr,suc) = conf(&format!("aprovação em {}",unsafe{REDES[SELEC].replace(".json", "")}),rec);
                            if apr != ""{
                                if suc == false{
                                    println!("{}",apr);
                                }
                                aprovado = apr;
                                sucesso = suc;
                                break;
                            }
                        },Err(_) =>{}
                    }
                }
                if sucesso == true{break;}
            },Err(_) =>{thread::sleep(Duration::from_secs(30));}
        }
    }
    return Ok((aprovado,sucesso));
}
async fn trocar(erc_20_entrada:Address,erc_20_saida:Address,contrato:Address,fee:u32,tempo:u64,quantia:U256,gwei:U256,
    provider0: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>)-> Result<(String,bool), Box<dyn std::error::Error>>{
    let mut aprovado:String = "".to_string();let  mut sucesso: bool = false;
    let publicaddr = provider0.address();
    loop{
        let (_,pricesg,_,decimais,_,_,addr,_,_,_,_,_) = prc_name(unsafe{REDES[SELEC]},"geral");
        let (_,_,_,_,_,porcent) = contantes(unsafe{REDES[SELEC]});
        let mut price = 0.0;
        let mut minvalue = U256::zero();
        for i in 0..pricesg.len(){
            if erc_20_entrada == addr[i]{
                let vv = calculate_value(quantia,pricesg[i],decimais[i]);
                price = vv;
                break;
            }
        }
        for i in 0..pricesg.len(){
            if erc_20_saida == addr[i]{
                let ex = calculate_expect(price,pricesg[i],decimais[i],porcent);
                minvalue = ex;
                break;
            }
        }
        let cont_uni = routeruni::routeruni::routeruni::new(contrato, provider0.clone());
        let params = routeruni::routeruni::ExactInputSingleParams{
            token_in:erc_20_entrada,
            token_out:erc_20_saida,
            fee:fee,
            recipient:publicaddr,
            deadline:calculate_deadline(tempo),
            amount_in:quantia,
            amount_out_minimum:minvalue,
            sqrt_price_limit_x96:U256::zero()
        };
        let transaction = cont_uni.exact_input_single(params).from(publicaddr);
        let gas = transaction.estimate_gas().await?;
        let gasprice = provider0.get_gas_price().await?;
        let multgas = multiply_u256_by_f64(gas, 1.1);
        let multgasprice = multiply_u256_by_f64(gasprice, 1.1);
        let balnc: U256 = provider0.get_balance(publicaddr, None).await?;
        if (gasprice*multgas) > gwei || (gasprice*multgas) > balnc {aprovado = "Taxa superior ao limite disponivel aguarde a normalização da rede ou adcione mais tokens..".to_string();return Ok((aprovado,false));}
        let transaction_gas = transaction.gas_price(multgasprice).gas(multgas);
        let transaction_tx = transaction_gas.send().await;
        match transaction_tx{
            Ok(stx) =>{
                let start: Instant = Instant::now();
                loop{
                    if start.elapsed().as_secs() > (tempo*60){
                        aprovado = "sessão espirou tentando novamente..".to_string();
                        println!("{aprovado}");
                        sucesso = false;break;
                    }
                    let receipt = provider0.get_transaction_receipt(stx.tx_hash()).await;
                    match receipt {
                        Ok(rec) =>{
                            let (apr,suc) = conf(&format!("swap na {}",unsafe{REDES[SELEC].replace(".json", "")}),rec);
                            if apr != ""{
                                if suc == false{
                                    println!("{}",apr);
                                }
                                aprovado = apr;
                                sucesso = suc;
                                break;
                            }
                        },Err(_) =>{}
                    }
                }
                if sucesso == true{break;}
            },Err(_) =>{thread::sleep(Duration::from_secs(30));}
        }
    }
    return Ok((aprovado,sucesso));
}
async fn desliga_pool(tokid:U256,lincy:u128,contrato:Address,tempo: u64,gwei:U256,
    provider0: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>)-> Result<(String,bool), Box<dyn std::error::Error>>{
    let mut aprovado:String = "".to_string();let  mut sucesso: bool = false;
    let publicaddr = provider0.address();
    loop{
        let poolinfo = unipoolgest::unipoolgest::unipoolgest::new(contrato, provider0.clone());
        let params = unipoolgest::unipoolgest::DecreaseLiquidityParams{
            token_id:tokid,
            liquidity:lincy,
            amount_0_min:U256::zero(),
            amount_1_min:U256::zero(),
            deadline:calculate_deadline(tempo)
        };
        let transaction = poolinfo.decrease_liquidity(params).from(publicaddr);
        let gas = transaction.estimate_gas().await?;
        let gasprice = provider0.get_gas_price().await?;
        let multgas = multiply_u256_by_f64(gas, 1.1);
        let multgasprice = multiply_u256_by_f64(gasprice, 1.1);
        let balnc: U256 = provider0.get_balance(publicaddr, None).await?;
        if (gasprice*multgas) > gwei || (gasprice*multgas) > balnc {aprovado = "Taxa superior ao limite disponivel aguarde a normalização da rede ou adcione mais tokens..".to_string();return Ok((aprovado,false));}
        let transaction_gas = transaction.gas_price(multgasprice).gas(multgas);
        let transaction_tx = transaction_gas.send().await;
        match transaction_tx{
            Ok(stx) =>{
                let start = Instant::now();
                loop{
                    if start.elapsed().as_secs() > (tempo*60){
                        aprovado = "sessão espirou tentando novamente..".to_string();
                        println!("{aprovado}");
                        sucesso = false;break;
                    }
                    let receipt = provider0.get_transaction_receipt(stx.tx_hash()).await;
                    match receipt {
                        Ok(rec) =>{
                            let (apr,suc) = conf(&format!("inativar pool na {}",unsafe{REDES[SELEC].replace(".json", "")}),rec);
                            if apr != ""{
                                if suc == false{
                                    println!("{}",apr);
                                }
                                aprovado = apr;
                                sucesso = suc;
                                break;
                            }
                        },Err(_) =>{}
                    }
                }
                if sucesso == true{break;}
            },Err(_) =>{thread::sleep(Duration::from_secs(30));}
        }
    }
    return Ok((aprovado,sucesso));
}
async fn rmvliq_pool(tokid:U256,contrato:Address,tempo:u64,gwei:U256,
    provider0: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>)-> Result<(String,bool), Box<dyn std::error::Error>>{
    let mut aprovado:String = "".to_string();let  mut sucesso: bool = false;
    let publicaddr = provider0.address();
    loop{
        let poolinfo = unipoolgest::unipoolgest::unipoolgest::new(contrato, provider0.clone());
        let coletar = unipoolgest::unipoolgest::CollectParams{
            token_id:tokid,
            recipient:provider0.clone().address(),
            amount_0_max:u128::max_value(),
            amount_1_max:u128::max_value()
        };
        let transaction = poolinfo.collect(coletar).from(publicaddr);
        let gas = transaction.estimate_gas().await?;
        let gasprice = provider0.get_gas_price().await?;
        let multgas = multiply_u256_by_f64(gas, 1.1);
        let multgasprice = multiply_u256_by_f64(gasprice, 1.1);
        let balnc: U256 = provider0.get_balance(publicaddr, None).await?;
        if (gasprice*multgas) > gwei || (gasprice*multgas) > balnc {aprovado = "Taxa superior ao limite disponivel aguarde a normalização da rede ou adcione mais tokens..".to_string();return Ok((aprovado,false));}
        let transaction_gas = transaction.gas_price(multgasprice).gas(multgas);
        let transaction_tx = transaction_gas.send().await;
        match transaction_tx{
            Ok(stx) =>{
                let start = Instant::now();
                loop{
                    if start.elapsed().as_secs() > (tempo*60){
                        aprovado = "sessão espirou tentando novamente..".to_string();
                        println!("{aprovado}");
                        sucesso = false;break;
                    }
                    let receipt = provider0.get_transaction_receipt(stx.tx_hash()).await;
                    match receipt {
                        Ok(rec) =>{
                            let (apr,suc) = conf(&format!("sacar liquidez selecionada pool na {}",unsafe{REDES[SELEC].replace(".json", "")}),rec);
                            if apr != ""{
                                if suc == false{
                                    println!("{}",apr);
                                }
                                aprovado = apr;
                                sucesso = suc;
                                break;
                            }
                        },Err(_) =>{}
                    }
                }
                if sucesso == true{break;}
            },Err(_) =>{thread::sleep(Duration::from_secs(30));}
        }
    }
    return Ok((aprovado,sucesso));
}
async fn get_erc20_transfer_info(
    provider: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    tx_hash: H256,
) -> Result<(Address, Address, Address, U256, u64), Box<dyn std::error::Error>> {

    let receipt = provider.get_transaction_receipt(tx_hash).await?;
    if let Some(receipt) = receipt {
        let block = provider.get_block(receipt.block_number.unwrap()).await?;
        if let Some(block) = block {
            let timestamp = block.timestamp.as_u64();
            let unixtime = get_unixtime(timestamp)?;
            
            let logs = receipt.logs;
            for log in logs {
                if log.topics.len() >= 3 && log.topics[0] == erc20_topic_hash("Transfer(address,address,uint256)") {
                    let contract_addr = log.address;
                    let from = Address::from_slice(&log.topics[1].as_fixed_bytes()[12..]);
                    let to = Address::from_slice(&log.topics[2].as_fixed_bytes()[12..]);
                    let amount = U256::from_big_endian(&log.data);

                    return Ok((from, to, contract_addr, amount, unixtime));
                }
            }
        }
    }
    Err("Nenhuma transferência de tokens ERC-20 encontrada.".into())
}

fn get_unixtime(timestamp: u64) -> Result<u64, Box<dyn std::error::Error>> {
    let unixtime = SystemTime::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(timestamp))
        .ok_or("Erro ao converter o timestamp")?;
    let unixtime = unixtime
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_| "Erro ao converter o timestamp")?
        .as_secs();
    Ok(unixtime)
}
fn get_days_difference(unixtime: u64) -> u64 {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current time")
        .as_secs();
    let seconds_in_a_day: u64 = 24 * 60 * 60;
    let current_days = current_time / seconds_in_a_day;
    let given_days = unixtime / seconds_in_a_day;
    if current_days >= given_days {current_days - given_days}
    else {given_days - current_days}
}
fn erc20_topic_hash(event_signature: &str) -> H256 {
    // Calcular o hash do tópico do evento ERC-20
    let signature = ethers::utils::keccak256(event_signature.as_bytes());
    H256::from_slice(&signature)
}
fn calculate_deadline(tempo:u64) -> U256 {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let deadline = current_time + (tempo * 60);
    U256::from(deadline)
}
async fn provedor(chave:&str,link:&str) ->Result<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>, Box<dyn std::error::Error>>{
    let provider:Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>
     = Arc::new({
        let provider:Provider<Http> = Provider::<Http>::try_from(link)?;
        let mut wallet = chave.parse::<LocalWallet>()?;
        let chain_id = provider.get_chainid().await?;
        wallet = wallet.with_chain_id(chain_id.as_u64());
        SignerMiddleware::new(provider, wallet)
    });
    return Ok(provider);
}
async fn balancofull(erc_20:Address,provider:&Arc<SignerMiddleware<Provider<Http>, 
    Wallet<SigningKey>>>)-> Result<(String,String,U256,u8), Box<dyn std::error::Error>>{
        let cont = ERC20::ERC20::new(erc_20, provider.clone());
        let syb: String = cont.symbol().call().await?;
        let decimal: u8 = cont.decimals().call().await?;
        let balanco: U256 = cont.balance_of(provider.address()).call().await?;
        return Ok((syb,format_u256_to_f64(balanco,decimal),balanco,decimal));
    }
async fn blc_nm(erc_20:Vec<Address>,provider:&Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        ex_names:Vec<String>,ex_dec:Vec<u8>)->Result<(Vec<String>,Vec<String>,Vec<u8>), Box<dyn std::error::Error>>{
        let mut balanc:Vec<String> = vec![];let mut tokens:Vec<String> = vec![];let mut decimal:Vec<u8> = vec![];
        for i in 0..erc_20.len(){
            if ex_names.len() == ex_dec.len() && erc_20.len() == ex_dec.len(){
                let cont = ERC20::ERC20::new(erc_20[i], provider.clone());
                let balanco: U256 = cont.balance_of(provider.address()).call().await?;
                let ballin: String = format_u256_to_f64(balanco,ex_dec[i]);
                balanc.push(ballin);
                tokens.push(ex_names[i].clone());
                decimal.push(ex_dec[i]);
            }else{
                let (name,balance,_,dec) = balancofull(erc_20[i], &provider).await?;
                balanc.push(balance);
                tokens.push(name);
                decimal.push(dec);
            }
        }
    return Ok((tokens,balanc,decimal));
}
fn conf(nome:&str,receipt:Option<TransactionReceipt>)-> (String,bool){
    let mut aprovado:String = "".to_string();let mut sucesso:bool = false;
    match receipt {
        Some(rsp) =>{
            match rsp.status {
                Some(s) =>{
                    if s == U64::from(1) {
                        sucesso = true;
                        aprovado = format!("{} lançado em: {:?}",nome,rsp.transaction_hash);
                    } 
                    if s == U64::from(0) {
                        sucesso = false;
                        aprovado = format!("erro ao lançar {} em: {:?}",nome,rsp.transaction_hash);
                    }
                },_ =>{}
            }
        },_ =>{}
    }
    thread::sleep(Duration::from_secs(15));
    return (aprovado,sucesso);
}
fn multiply_u256_by_f64(u256: U256, f64_value: f64) -> U256 {
    let u256_value: u128 = u256.as_u128();
    let result = (u256_value as f64 * f64_value) as u128;
    U256::from(result)
}
fn format_u256_to_f64(value: U256, decimals: u8) -> String {
    let value_string = value.to_string();
    let value_length = value_string.len();
    
    if value_length < decimals as usize {
        let padding_zeros = "0".repeat(decimals as usize - value_length);
        format!("0.{}{}", padding_zeros, value_string)
    } else {
        let (integer_part, decimal_part) = value_string.split_at(value_length - decimals as usize);
        format!("{}.{}", integer_part, decimal_part)
    }
}
async fn fetch_token_prices(addro: Vec<Address>,provider:&Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) 
-> Result<(Vec<String>), Box<dyn std::error::Error>> {
    let mut prices:Vec<String> = vec![];
    for or_addr in addro{
        let oraculo = oracle::oracle::oracle::new(or_addr, provider.clone());
        let price: u128 = oraculo.latest_answer().call().await?.as_u128();
        let decimal: u8 = oraculo.decimals().call().await?;
        prices.push(format_u256_to_f64(U256::from(price),decimal));
    }

    return Ok(prices);
}
fn convert_to_json(name: Vec<String>, price: Vec<String>, balance: Vec<String>,addr: Vec<Address>,dec: Vec<u8>) -> Vec<String> {
    let mut json_data = Vec::new();

    for i in 0..name.len() {
        let uno = format!("{{\"name\":\"{}\",\"price\":{:?},\"balance\":{:?},\"{}\":\"{:?}\",\"decimal\":\"{}\"}}",name[i],price[i],balance[i],name[i],addr[i],dec[i]);
        json_data.push(uno);
    }

    return json_data;
}
fn prc_name(file:&str,element:&str) -> (Vec<String>,Vec<f64>,Vec<U256>,Vec<u8>,Vec<String>,Vec<bool>,Vec<Address>,Vec<U256>,Vec<String>,Vec<String>,Vec<u64>,Vec<String>){
    let jsons = {
        let text = std::fs::read_to_string(&file).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let geral: Vec<String> = serde_json::from_str(&jsons[element].to_string()).unwrap();
    let mut prices:Vec<f64> = vec![];
    let mut balances:Vec<U256> = vec![];
    let mut names:Vec<String> = vec![];
    let mut decimais:Vec<u8> = vec![];
    let mut acao:Vec<String> = vec![];
    let mut callput:Vec<bool> = vec![];
    let mut addr:Vec<Address> = vec![];
    let mut tokenid:Vec<U256> = vec![];
    let mut token0:Vec<String> = vec![];
    let mut token1:Vec<String> = vec![];
    let mut diasvec:Vec<u64> = vec![];
    let mut modos:Vec<String> = vec![];
    for g in geral{
        let unit: Value = serde_json::from_str(&g).unwrap();
        let json = json!(unit);
        let balance:U256 = U256::from(format!("{}",json["balance"].to_string().replace('"',"").replace('.',"")).parse::<u128>().unwrap());
        let price:f64 = format!("{}",json["price"]).to_string().replace('"',"").parse::<f64>().unwrap();
        let name:String = json["name"].to_string().replace('"',"");
        if element == "geral"{
            let decimal:u8 = format!("{}",json["decimal"]).to_string().replace('"',"").parse::<u8>().unwrap();
            let addrs: Address = json[&name].to_string().replace('"',"").parse().unwrap();
            addr.push(addrs);
            decimais.push(decimal);
        }
        if element == "agenda"{
            let acaos:String = json["acao"].to_string().replace('"',"");
            let token0s:String = json["token0"].to_string().replace('"',"");
            let token1s:String = json["token1"].to_string().replace('"',"");
            let modo:String = json["modo"].to_string().replace('"',"");
            let tokenids:U256 = U256::from(format!("{}",json["tokenid"].to_string().replace('.',"")).parse::<u128>().unwrap());
            let callputs:bool = json["callput"].as_bool().unwrap();
            let dias:u64 = format!("{}",json["dias"]).to_string().replace('"',"").parse::<u64>().unwrap();
            modos.push(modo);
            acao.push(acaos);
            token0.push(token0s);
            token1.push(token1s);
            tokenid.push(tokenids);
            callput.push(callputs);
            diasvec.push(dias);
        }
        prices.push(price);
        names.push(name);
        balances.push(balance);
    }
    return (names,prices,balances,decimais,acao,callput,addr,tokenid,token0,token1,diasvec,modos);
}
async fn save_addr(file:&str,
    provider:&Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>){
    let mut jsons = {
        let text = std::fs::read_to_string(&file).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let contratos: Vec<String> = serde_json::from_str(&jsons["contratos"].to_string()).unwrap();
    let oracles: Vec<String> = serde_json::from_str(&jsons["oracles"].to_string()).unwrap();
    let geral: Vec<String> = serde_json::from_str(&jsons["geral"].to_string()).unwrap();
    let mut ex_names: Vec<String> = vec![];
    let mut ex_decimal: Vec<u8> = vec![];
    let mut cont_addr:Vec<Address> = vec![];
    let mut oralcle_addr:Vec<Address> = vec![];
    for i in 0..contratos.len(){
        let addro = oracles[i].parse::<Address>();
        let addrs = contratos[i].parse::<Address>();
        match addrs {
            Ok(addr) =>{
                cont_addr.push(addr);
            },Err(_) =>{println!("Erro ao salvar contrato.");break;}
        }
        match addro {
            Ok(addr) =>{
                oralcle_addr.push(addr);
            },Err(_) =>{println!("Erro ao salvar oráculo.");break;}
        }
    }
    if cont_addr.len() == oralcle_addr.len(){
        if geral.len() == oralcle_addr.len(){
            let (namesg,_,_,dec,_,_,_,_,_,_,_,_) = prc_name(file,"geral");
            ex_names = namesg;
            ex_decimal = dec;
        }
        let blc = blc_nm(cont_addr.clone(), provider,ex_names,ex_decimal).await;
        let priceerr = fetch_token_prices(oralcle_addr,provider).await;
        match blc {
            Ok((name,balance,decimal)) =>{
                match priceerr {
                    Ok(price) =>{
                        if unsafe{APIS == false}{println!("conecção estabelecida com oráculo");unsafe{APIS = true}}
                        let result = convert_to_json(name,price,balance,cont_addr,decimal);
                        jsons["geral"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",result).as_str()).unwrap();
                        std::fs::write(
                            &file,
                            serde_json::to_string_pretty(&jsons).unwrap(),
                        ).unwrap();
                    },Err(e) =>{if unsafe{APIS == true}{println!("erro ao estabelecer conecção com o oráculo..");unsafe{APIS = false}}}
                }
            },
            Err(_) =>{if unsafe{APIS == true}{println!("erro ao capturar informações..");unsafe{APIS = false}}}
        }
    }
}
fn remove_acao(file:&str,element:&str,index: Vec<usize>) {
    let mut jsons = {
        let text = std::fs::read_to_string(&file).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let mut geral: Vec<String> = serde_json::from_str(&jsons[element].to_string()).unwrap();
    let mut retorno: Vec<String> = vec![];
    for i in 0..geral.len(){
        if index.contains(&i) == false{
            retorno.push(geral[i].clone())
        }
    }
    jsons[element] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",retorno).as_str()).unwrap();
    unsafe{WRIR = false;}
    std::fs::write(
        &file,
        serde_json::to_string_pretty(&jsons).unwrap(),
    ).unwrap();
    unsafe{WRIR = true;}
}
fn contantes(file:&str) ->(Address,Address,u64,Address,U256,u64) {
    let jsons = {
        let text = std::fs::read_to_string(&file).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let uni_router: String = serde_json::from_str(&jsons["contantes"]["uni_router"].to_string()).unwrap();
    let uni_pool: String = serde_json::from_str(&jsons["contantes"]["uni_pool"].to_string()).unwrap();
    let uni_factory: String = serde_json::from_str(&jsons["contantes"]["factory"].to_string()).unwrap();
    let uni_router_addr: Address = uni_router.parse().unwrap();
    let uni_factory_addr: Address = uni_factory.parse().unwrap();
    let uni_pool_addr: Address = uni_pool.parse().unwrap();
    let tempo: u64 = serde_json::from_str(&jsons["contantes"]["tempo"].to_string()).unwrap();
    let gwei: u128 = serde_json::from_str(&jsons["contantes"]["gwei"].to_string()).unwrap();
    let porcent: u64 = serde_json::from_str(&jsons["contantes"]["porcentagem"].to_string()).unwrap();
    return (uni_router_addr,uni_pool_addr,tempo,uni_factory_addr,U256::from(gwei),porcent);
}
fn extract_digits(u: U256, digit: u8) -> u64 {
    let mut ssst = "1".to_string();
    for i in 0..digit {
        if i == digit-2{break;}
        ssst = ssst+"0"
    }
    let nuntest = U256::from(ssst.parse::<u128>().unwrap());
    (u/nuntest).as_u64()
}
fn promos(valor:u64,dias:u64) -> (String,bool,u64){
    let mut pago = false;
    if valor >= 5950 && dias <= 365 && unsafe{SOLO == true}{
        pago = true;
        return (format!("Plano Master identificado!! Restam mais {} dias de uso.",365-dias),pago,365-dias);
    }else if valor >= 1350 && dias <= 93 && unsafe{SOLO == true}{
        pago = true;
        return (format!("Plano Profit identificado!! Restam mais {} dias de uso.",93-dias),pago,93-dias);
    }else if valor >= 590 && dias <= 31 && unsafe{SOLO == true}{
        pago = true;
        return (format!("Plano Starter identificado!! Restam mais {} dias de uso.",31-dias),pago,31-dias);
    }else if valor >= 49990 && dias <= 31 && unsafe{SOLO == false}{
        pago = true;
        return (format!("Teste grátis identificado!! Restam mais {} dias de uso.",31-dias),pago,31-dias);
    }
    return (format!("nenhum plano encontrato no momento.. \nRealize o pagamento conforme as diretrizes da página onde foi realizada o Download"),pago,0);
}
async fn feende(factory:Address,esse:Address,paraesse:Address
    ,provider0:&Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) ->(u32,bool,U256){
    let mut libera: bool = false;
    let fees: Vec<u32> = vec![100,500,3000,10000];
    let mut fee:u32 = 0;
    let mut balancos:U256 = U256::zero();
    let mut feesliq:Vec<u128> = vec![];
    let mut feesAddr:Vec<Address> = vec![];
    let cont_factory = unifact::unifact::unifact::new(factory, provider0.clone());
    for fe in &fees{
        loop{
            let feeliq = cont_factory.get_pool(esse, paraesse, *fe).call().await;
            match feeliq {
                Ok(fee_addr) => {
                    feesAddr.push(fee_addr.clone());
                    if fee_addr == "0x0000000000000000000000000000000000000000".parse().unwrap(){feesliq.push(0);break;}
                    let get_pool = unipooladdr::unipooladdr::unipooladdr::new(fee_addr, provider0.clone());
                    let realfee = get_pool.liquidity().call().await;
                    match realfee {
                        Ok(rf) =>{
                            feesliq.push(rf);
                            break;
                        },Err(_) =>{}
                    }
                },Err(_) =>{},
            } 
        }
    }
    for lqf in &feesliq{
        if *lqf != 0{libera = true;break;}
    }
    if libera == true{
        let feeindex = encontrar_maior_valor(feesliq);
        fee = fees[feeindex];
        let cont = ERC20::ERC20::new(paraesse, provider0.clone());
        loop{
            let balanco = cont.balance_of(feesAddr[feeindex]).call().await;
            match balanco {
                Ok(b) =>{
                    balancos = b;
                    break;
                },Err(_) =>{}
            }
        }
    }
    return (fee,libera,balancos);
}
fn encontrar_maior_valor(vetor: Vec<u128>) -> usize {
    let mut indice_maior_valor = 0;
    let mut maior_valor = 0;

    for (indice, valor) in vetor.iter().enumerate() {
        if *valor > maior_valor {
            maior_valor = *valor;
            indice_maior_valor = indice;
        }
    }
    indice_maior_valor
}
fn calculate_value(value: U256, price: f64, decimals: u8) -> f64 {
    let divisor = 10u64.pow(decimals.into()) as f64;
    let calculated_value = value.low_u128() as f64 / divisor;
    calculated_value * price
}
fn calculate_expect(qb: f64, preco: f64, casa: u8,porcent:u64) -> U256 {
    let decimal_multiplier = 10u64.pow(casa.into()) as f64;
    let result = (((qb / preco)*(1.0-(porcent as f64/100.0))) * decimal_multiplier) as u128;
    U256::from(result)
}
