// Importa o ethers
const ethers = require('ethers');

// URL do Goerli RPC
const GOERLI_RPC_URL = "https://goerli.infura.io/v3/0x5719046dd09aF1718306Fb6f6d3AB106B95C0d31"; // Substitua pelo seu URL do Infura

// Gera uma nova carteira (chave privada e endereço)
const wallet = ethers.Wallet.createRandom();
console.log("Endereço:", wallet.address);
console.log("Chave Privada:", wallet.privateKey);
console.log("Frase Mnêmica:", wallet.mnemonic.phrase);

// Chave privada gerada anteriormente
const PRIVATE_KEY = wallet.privateKey; // Utiliza a chave privada gerada

// Cria um provedor usando o URL do Goerli
const provider = new ethers.JsonRpcProvider(GOERLI_RPC_URL);

// Cria uma carteira usando a chave privada e o provedor
const walletWithProvider = new ethers.Wallet(PRIVATE_KEY, provider);

// Função para obter o saldo do endereço da carteira
async function getBalance() {
    const balance = await walletWithProvider.getBalance();
    console.log("Saldo em Ether:", ethers.formatEther(balance));
}

// Executa a função
getBalance();