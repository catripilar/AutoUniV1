var datat = {"moedas": []};
var agendamento = [];
var num = 0;
var selectedName = "swap";
var selectedOwnedCoin = '';
var selectedquantia = '';
var selectedLiquidity = 0;
var MaxLiquidity = 0;
var MaxPrice = 0;
var selectedOtherCoin = '';
var selectedAllCoin = '';
var selectedPrice = 0;
var selectedFIXPrice = 0;
var selectedTokenid = 0;
var selectedOption = false;
var dias = 2;
var got = false;
var selectedchain = -1;
var redeatual = "";
var wkday = "Dia(s)";
  function isNumber(event) {
    var charCode = event.charCode;
    var inputValue = event.target.value;
    
    // Verifica se o caractere é um número de 0 a 9 ou se é o ponto decimal
    if (
      (charCode >= 48 && charCode <= 57) || // Números de 0 a 9
      (charCode === 46 && inputValue.indexOf('.') === -1) // Ponto decimal (somente se ainda não houver ponto no valor)
    ) {
      return true; // Permite a digitação do caractere
    } else {
      event.preventDefault(); // Impede a digitação do caractere
      return false;
    }
  }

  function sendData() {
    var number = document.getElementById("numberInput").value;
    var xhr = new XMLHttpRequest();
    var rangeValue = document.getElementById('PoolValue');
    xhr.onreadystatechange = function() {
      if (xhr.readyState === 4 && xhr.status === 200) {
        var response = xhr.responseText;
        const parsedData = JSON.parse(response);
        var rangeInput = document.getElementById('PoolLiquidity');
        var poolpair = document.getElementById('Poolpair');
        got = parsedData.go;
        if (got == true){
          poolpair.textContent = "Par da piscina selecionada: "+parsedData.token0+"/"+parsedData.token1;
          rangeInput.addEventListener('input', function() {
              var value = rangeInput.value;
              rangeValue.textContent = value+"%";
              selectedTokenid = parsedData.tokenid;
              selectedLiquidity = (parsedData.lincy/100)*value;
          });
        }else{
          selectedLiquidity = 0;
          poolpair.textContent = "A piscina selecionada não pertence a carteira selecionada.";
        }
      }
    };
    xhr.open("POST", "/poll_nun", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    rangeValue.textContent = "aguarde..";
    xhr.send("nun="+number);
  }
  function addcont() {
    var contrato = document.getElementById("new_cont").value;
    var oraculo = document.getElementById("new_oracle").value;
    var xhr = new XMLHttpRequest();
    var cont_infos = document.getElementById('cont_infos');
    xhr.onreadystatechange = function() {
      if (xhr.readyState === 4 && xhr.status === 200) {
        var response = xhr.responseText;
        cont_infos.textContent = response;
      }
    };
    xhr.open("POST", "/submit_cont", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    cont_infos.textContent = "aguarde..";
    xhr.send("cont="+contrato+"&oracle="+oraculo);
  }
  function par_view() {
    var xhr = new XMLHttpRequest();
    var cont_infos = document.getElementById('liqtrade');
    xhr.onreadystatechange = function() {
      if (xhr.readyState === 4 && xhr.status === 200) {
        var response = xhr.responseText;
        cont_infos.textContent = response;
      }
    };
    xhr.open("POST", "/par_view", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    saveData();
    cont_infos.textContent = "aguarde..";
    xhr.send("token0="+ (selectedOwnedCoin)+"&token1="+ (selectedOtherCoin));
  }
  function agendar(dados) {
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
      if (xhr.readyState === 4) {
        if (xhr.status === 200) {
          var response = xhr.responseText;
          if (response != ""){
            document.open();
            document.write(response);
            document.close();
          }else{
            alert("Já existe uma transação em andamento. Caso contrário, verifique o console.");
          }
        } else {
          console.error('Erro ao transmitir os dados do formulário. Status: ' + xhr.status);
        }
      }
    };
    xhr.open("POST", "/submit", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    saveData();
    if (selectedName == "stop"){
      if (got == false){
        return
      }
      selectedLiquidity = parseInt(selectedLiquidity);
      selectedquantia = document.getElementById('PoolValue').textContent;
      selectedOwnedCoin = selectedOtherCoin;
    }else{
      selectedquantia = document.getElementById('newLiquidity').value;
    }
    if (selectedLiquidity <= 0){return}
    if (dados.dataset.info == "price"){
      dias = 1;
      if (selectedAllCoin == "-"){return}
    }
    xhr.send(
    "acao="+ (selectedName)+
    "&name=" + (selectedAllCoin)+
    "&price="+ (selectedPrice)+
    "&balance="+ (selectedLiquidity)+
    "&quantia="+ (selectedquantia)+
    "&token0="+ (selectedOwnedCoin)+
    "&token1="+ (selectedOtherCoin)+
    "&tokenid="+ (selectedTokenid)+
    "&callput="+ (selectedOption)+
    "&dias="+ (dias)+
    "&modo="+ (dados.dataset.info)
    );
    selectedFIXPrice = 0;
    selectedOwnedCoin = '';
    selectedLiquidity = 0;
    MaxLiquidity = 0;
    MaxPrice = 0;
    selectedOtherCoin = '';
    selectedAllCoin = '';
    selectedPrice = 0;
    selectedTokenid = 0;
    selectedOption = false;
    got = false;
    dias = 2;
    selectedchain = -1;
  }
  function rede(name){
    datat = {"moedas": []};
    agendamento = [];
    selectedFIXPrice = 0;
    selectedOwnedCoin = '';
    selectedLiquidity = 0;
    selectedOtherCoin = '';
    selectedAllCoin = '';
    selectedPrice = 0;
    MaxLiquidity = 0;
    MaxPrice = 0;
    selectedTokenid = 0;
    selectedOption = false;
    got = false;
    num = 0;
    dias = 2;
    removeCards();
    if (name == "etherium"){
      selectedchain = 0;
    }
    if (name == "polygon"){
      selectedchain = 1;
    }
    if (name == "arbitrum"){
      selectedchain = 2;
    }
    if (name == "optimism"){
      selectedchain = 3;
    }
    redeatudias = 1;al = name+".json";
    fetch('/json/'+name)
    .then(response => response.json())
    .then(data => {
      const parsedData = JSON.parse(data);
      num = parsedData.operacao;
      parsedData.agenda.forEach(g => {
        agendamento.push(g);
      });
      parsedData.geral.forEach(g => {
        const parsedSubData = JSON.parse(g);
        datat.moedas.push(parsedSubData);
      });
    if (name === 'etherium') {
        document.getElementById('cont_desc').innerText = "Rede Ethereum";
        document.getElementById('orac_desc').innerText = "Oráculo Ethereum";
        document.getElementById('cont_desc').href = "https://etherscan.io/";
        document.getElementById('orac_desc').href = "https://docs.chain.link/data-feeds/price-feeds/addresses?network=ethereum";
        document.body.style.backgroundColor = '#0e408b';
    } else if (name === 'polygon') {
        document.getElementById('cont_desc').innerText = "Rede Polygon";
        document.getElementById('orac_desc').innerText = "Oráculo Polygon";
        document.getElementById('cont_desc').href = "https://polygonscan.com/";
        document.getElementById('orac_desc').href = "https://docs.chain.link/data-feeds/price-feeds/addresses?network=polygon";
        document.body.style.backgroundColor = '#8247e5';
    } else if (name === 'arbitrum') {
        document.getElementById('cont_desc').innerText = "Rede Arbitrum";
        document.getElementById('orac_desc').innerText = "Oráculo Arbitrum";
        document.getElementById('cont_desc').href = "https://arbiscan.io/";
        document.getElementById('orac_desc').href = "https://docs.chain.link/data-feeds/price-feeds/addresses?network=arbitrum";
        document.body.style.backgroundColor = '#28a0f0';
    }else if (name === 'optimism') {
        document.getElementById('cont_desc').innerText = "Rede Optimism";
        document.getElementById('orac_desc').innerText = "Oráculo Optimism";
        document.getElementById('cont_desc').href = "https://optimistic.etherscan.io/";
        document.getElementById('orac_desc').href = "https://docs.chain.link/data-feeds/price-feeds/addresses?network=optimism";
        document.body.style.backgroundColor = '#f02828';
    }
      var newLiquiditySlider = document.getElementById('newLiquidity');
      newLiquiditySlider.addEventListener('input', updateLiquidityValue);

      var rangeDCA = document.getElementById('DCA');
      rangeDCA.addEventListener('input', updateDCAValue);

      var rangepct = document.getElementById('pricepct');
      rangepct.addEventListener('input', updatepctValue);

      var ownedCoinsSelect = document.getElementById('ownedCoinsSelect');
      ownedCoinsSelect.addEventListener('change', updateCoinOptions);

      var allCoinsSelect = document.getElementById('allCoins');

      allCoinsSelect.addEventListener('change', function() {
          var selectedCoin = allCoinsSelect.value;
          var selectedCoinData = datat.moedas.find(function(coin) {
              return coin.name === selectedCoin;
          });
          var priceInput = document.getElementById('priceInput');
          selectedFIXPrice = selectedCoinData.price;
          priceInput.value = selectedFIXPrice;
      });
      var moedabalance = [];
      datat.moedas.forEach(coin => {
        if (coin.balance > 0) {
          moedabalance.push(coin);
        }
      });
      fillOptions('ownedCoinsSelect', moedabalance);
      updateCoinOptions();
      renderCards();
    })
    .catch(error => {
      console.error('Erro ao obter os dados JSON:', error);
    });
}
function fillOptions(elementId, coins) {
    var select = document.getElementById(elementId);
    select.innerHTML = '';
    coins.forEach(function(coin) {
        var option = document.createElement('option');
        option.value = coin.name;
        option.text = coin.name;
        select.appendChild(option);
    });
}
function fillOptionsAll(elementId, coins) {
  var select = document.getElementById(elementId);
  select.innerHTML = '';
  var option = document.createElement('option');
  option.value = "-";
  option.text = "-";
  select.appendChild(option);

  coins.forEach(function(coin) {
      var option = document.createElement('option');
      option.value = coin.name;
      option.text = coin.name;
      select.appendChild(option);
  });
}
function updatepctValue(){
  var slider = document.getElementById('pricepct');
  var sliderdata = document.getElementById('pricepctvalue');
  var value = slider.value;
  var valuetup = ((value/100) + 1);
  var decimalPlaces = getDecimalPlaces(selectedFIXPrice);
  selectedPrice = (selectedFIXPrice*valuetup).toFixed(decimalPlaces);
  if (valuetup >= 1){
    selectedOption = true;
    sliderdata.textContent = "Realizar quando o preço estiver a +"+value+"% ($"+selectedPrice+") do Atual"
    sliderdata.style.backgroundColor = "#32CD32";
  }else{
    selectedOption = false;
    sliderdata.textContent = "Realizar quando o preço estiver a "+value+"% ($"+selectedPrice+") do Atual"
    sliderdata.style.backgroundColor = "#DC143C";
  }
}
function updateDCAValue(){
  var slider = document.getElementById('DCA');
  var sliderdata = document.getElementById('DCAdays');
  var value = slider.value;
  dias = value;
  sliderdata.textContent = dias+" "+wkday+" comprando.";
}
function updateDurationText() {
  var selector = document.getElementById("durationSelector");
  var dy = document.getElementById("dy");
  var wk = document.getElementById("wk");

  if (selector.value === "days") {
    wkday = "Dia(s)";
    dy.style.display = "block";
    wk.style.display = "none";
  } else if (selector.value === "weeks") {
    wkday = "Semana(s)";
    dy.style.display = "none";
    wk.style.display = "block";
  }

  document.getElementById("DCAdays").textContent = text;
}
function updateLiquidityValue() {
    var slider = document.getElementById('newLiquidity');
    var selectedCoin = document.getElementById('ownedCoinsSelect').value;
    var selectedCoinData = datat.moedas.find(function(coin) {
        return coin.name === selectedCoin;
    });
    var decimalPlaces = getDecimalPlaces(selectedCoinData.balance);
    var liquidity = ((Number((slider.value)*MaxLiquidity))/MaxPrice).toFixed(decimalPlaces);
    selectedLiquidity = liquidity;
}
function getDecimalPlaces(balance) {
    var decimalString = String(balance).split('.')[1] || '';
    return decimalString.length;
}
function saveData() {
    selectedquantia = document.getElementById('liquidityValue').textContent;
    selectedOwnedCoin = document.getElementById('ownedCoinsSelect').value;
    selectedOtherCoin = document.getElementById('otherCoins').value;
    selectedAllCoin = document.getElementById('allCoins').value;
}
function updateCoinOptions() {
    var ownedCoinsSelect = document.getElementById('ownedCoinsSelect');

    selectedOwnedCoin = ownedCoinsSelect.value;
    var selectedCoinData = datat.moedas.find(function(coin) {
        return coin.name === selectedOwnedCoin;
    });

    var newLiquiditySlider = document.getElementById('newLiquidity');
    MaxLiquidity = Number(selectedCoinData.balance).toFixed(getDecimalPlaces(selectedCoinData.balance));
    MaxPrice = (MaxLiquidity*(selectedCoinData.price)).toFixed(2);
    newLiquiditySlider.value = MaxPrice;
    updateLiquidityValue();

    fillOptions('otherCoins', datat.moedas.filter(function(coin) {
        return coin.name !== selectedOwnedCoin;
    }));
    fillOptionsAll('allCoins', datat.moedas);
}
function showContent(contentId) {
  selectedName = document.getElementById(contentId).dataset.info;
  var contents = document.getElementsByClassName('content');
  for (var i = 0; i < contents.length; i++) {
      contents[i].style.display = 'none';
  }

  var content = document.getElementById(contentId);
  if (content) {
      content.style.display = 'block';
  }
  var sendinfos = document.getElementById('sendinfos');
  if (sendinfos) {
    sendinfos.style.display = 'block';
  }
}
function createCard(i,data, index) {
  const parsedData = JSON.parse(data);
  const card = document.createElement("div");
  const aElement = document.createElement("a");
  aElement.setAttribute('href', '/');
  aElement.textContent = 'Clique aqui para excluir essa ação..';
  aElement.addEventListener("click",function(){
    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/exclui_acao", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhr.send("posi="+i+"&rede="+selectedchain);
  });
  card.classList.add("card");
  if (index == true) {
      card.classList.add("highlight");
  }
  let tipo = "";
  let message = "";
  if (parsedData.acao === "swap") {
      const direction = parsedData.callput ? "acima" : "abaixo";
      if (parsedData.modo == "dca"){tipo = `Em todo fechamento nos proximos ${parsedData.dias} Dia(s)`}
      else if(parsedData.modo == "dcawk"){tipo = `Toda Segunda-feira nas proximas ${parsedData.dias} Semana(s)`}
      else{tipo = `quando o ${parsedData.name} estiver ${direction} de $${parsedData.price}`}

      message = `A ação ${parsedData.acao} 
      foi agendada para ser realizada ${tipo} para trocar $${parsedData.quantia}(USD)
       de ${parsedData.token0} por ${parsedData.token1}.`;

  } else if (parsedData.acao === "stop") {
      const direction = parsedData.callput ? "acima" : "abaixo";
      if (parsedData.modo == "dca"){tipo = `Em todo fechamento nos proximos ${parsedData.dias} Dia(s)`}
      else if(parsedData.modo == "dcawk"){tipo = `Toda Segunda-feira nas proximas ${parsedData.dias} Semana(s)`}
      else{tipo = `quando o ${parsedData.name} estiver ${direction} de $${parsedData.price}`}

      message = `A ação ${parsedData.acao} 
      foi agendada para ser realizada ${tipo} para recolher a liquidez
       de ${parsedData.quantia} porcento da piscina com o id: ${parsedData.tokenid} para o token: ${parsedData.token0}.`;
  }

  card.textContent = message;
  card.appendChild(aElement);
  return card;
}
function renderCards() {
  const cardContainer = document.getElementById("cardContainer");
  if (agendamento.length > 0){
    for (let i = 0; i < agendamento.length; i++) {
      var selecard = false;
        if (num-1 >= 0 &&
        i == num-1){selecard = true}
          const card = createCard(i,agendamento[i], selecard);
          cardContainer.appendChild(card);
      }
  }
}
function removeCards() {
  const cardContainer = document.getElementById("cardContainer");

  while (cardContainer.firstChild) {
      cardContainer.firstChild.remove();
  }
}