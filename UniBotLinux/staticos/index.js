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
var selectedTokenid = 0;
var selectedOption = false;
var got = false;
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
    xhr.onreadystatechange = function() {
      if (xhr.readyState === 4 && xhr.status === 200) {
        var response = xhr.responseText;
        const parsedData = JSON.parse(response);
        var rangeInput = document.getElementById('PoolLiquidity');
        var rangeValue = document.getElementById('PoolValue');
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
          poolpair.textContent = "A piscina selecionada Não pertence a carteira selecionada.";
        }
      }
    };
    xhr.open("POST", "/poll_nun", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhr.send("nun="+number);
  }
  function addcont() {
    var contrato = document.getElementById("new_cont").value;
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function() {
      if (xhr.readyState === 4 && xhr.status === 200) {
        var response = xhr.responseText;
        var cont_infos = document.getElementById('cont_infos');
        cont_infos.textContent = response;
      }
    };
    xhr.open("POST", "/submit_cont", true);
    xhr.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    xhr.send("cont="+contrato);
  }
  function swap() {
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
            alert("Já existe uma transação em andamento. Tente novamente quando ela for finalizada.");
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
      if (selectedLiquidity > MaxLiquidity){return}
    }
    if (selectedLiquidity <= 0){return}
    xhr.send(
    "acao="+ (selectedName)+
    "&name=" + (selectedAllCoin)+
    "&price="+ (selectedPrice)+
    "&balance="+ (selectedLiquidity)+
    "&quantia="+ (selectedquantia)+
    "&token0="+ (selectedOwnedCoin)+
    "&token1="+ (selectedOtherCoin)+
    "&tokenid="+ (selectedTokenid)+
    "&callput="+ (selectedOption)
    );
    selectedName = '';
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
  }
  function rede(name){
    datat = {"moedas": []};
    agendamento = [];
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
    removeCards();
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
      if (name === 'polygon') {
        document.body.style.backgroundColor = '#8247e5';
    } else if (name === 'arbitrum') {
        document.body.style.backgroundColor = '#28a0f0';
    }
      var newLiquiditySlider = document.getElementById('newLiquidity');
      newLiquiditySlider.addEventListener('input', updateLiquidityValue);

      var ownedCoinsSelect = document.getElementById('ownedCoinsSelect');
      ownedCoinsSelect.addEventListener('change', updateCoinOptions);

      var allCoinsSelect = document.getElementById('allCoins');

      allCoinsSelect.addEventListener('change', function() {
          var selectedCoin = allCoinsSelect.value;
          var selectedCoinData = datat.moedas.find(function(coin) {
              return coin.name === selectedCoin;
          });
          var priceInput = document.getElementById('priceInput');
          priceInput.value = selectedCoinData.price;
      });

      var submitButton = document.getElementById('submitButton');
      submitButton.addEventListener('click', saveData);

      fillOptions('ownedCoinsSelect', datat.moedas);
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

function updateLiquidityValue() {
    var slider = document.getElementById('newLiquidity');
    var selectedCoin = document.getElementById('ownedCoinsSelect').value;
    var selectedCoinData = datat.moedas.find(function(coin) {
        return coin.name === selectedCoin;
    });
    var decimalPlaces = getDecimalPlaces(selectedCoinData.balance);
    var liquidity = ((Number(slider.value*MaxLiquidity))/MaxPrice).toFixed(decimalPlaces);
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
    selectedPrice = document.getElementById('priceInput').value;
    selectedOption = document.getElementById('checkBox').checked;
}
function updateCoinOptions() {
    var ownedCoinsSelect = document.getElementById('ownedCoinsSelect');
    var otherCoinsSelect = document.getElementById('otherCoins');
    var allCoinsSelect = document.getElementById('allCoins');
    var priceInput = document.getElementById('priceInput');

    selectedOwnedCoin = ownedCoinsSelect.value;
    var selectedCoinData = datat.moedas.find(function(coin) {
        return coin.name === selectedOwnedCoin;
    });

    var newLiquiditySlider = document.getElementById('newLiquidity');
    MaxLiquidity = Number(selectedCoinData.balance).toFixed(getDecimalPlaces(selectedCoinData.balance));
    MaxPrice = (MaxLiquidity*(selectedCoinData.price)).toFixed(2);
    newLiquiditySlider.value = MaxPrice;
    updateLiquidityValue();

    priceInput.value = selectedCoinData.price;

    fillOptions('otherCoins', datat.moedas.filter(function(coin) {
        return coin.name !== selectedOwnedCoin;
    }));
    fillOptions('allCoins', datat.moedas);
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
function createCard(data, index) {
  const parsedData = JSON.parse(data);
  const card = document.createElement("div");
  card.classList.add("card");
  if (index == true) {
      card.classList.add("highlight");
  }
  let message = "";
  if (parsedData.acao === "swap") {
      const direction = parsedData.callput ? "acima" : "abaixo";
      message = `A ação ${parsedData.acao} 
      foi agendada para ser realizada quando o ${parsedData.name} estiver ${direction} 
      de $${parsedData.price} para trocar $${parsedData.quantia} de ${parsedData.token0} por ${parsedData.token1}.`;
  } else if (parsedData.acao === "stop") {
      const direction = parsedData.callput ? "acima" : "abaixo";
      message = `A ação ${parsedData.acao} 
      foi agendada para ser realizada quando o ${parsedData.name} estiver ${direction} 
      de $${parsedData.price} para recolher a liquidez de ${parsedData.quantia} da piscina com o id: ${parsedData.tokenid} para o token: ${parsedData.token0}.`;
  }

  card.textContent = message;
  return card;
}
function renderCards() {
  const cardContainer = document.getElementById("cardContainer");
  if (agendamento.length > 0){
    for (let i = 0; i < agendamento.length; i++) {
      var selecard = false;
        if (num-1 >= 0 &&
        i == num-1){selecard = true}
          const card = createCard(agendamento[i], selecard);
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
function changeText(checkbox) {
  var labelText = document.getElementById("labelText");
  labelText.innerHTML = checkbox.checked ? "Acima:" : "Abaixo:";
}