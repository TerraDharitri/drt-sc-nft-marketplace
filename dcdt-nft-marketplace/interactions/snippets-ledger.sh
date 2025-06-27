LEDGER_INDEX=0
ROOT=".."

CONTRACT_ADDRESS=$(drtpy data load --key=address-nft-marketplace)
DEPLOY_TRANSACTION=$(drtpy data load --key=deploy-tx-nft-marketplace)

PROXY=https://testnet-gateway.numbat.com
CHAIN_ID=T

GAS=100000000
BID_PERCENT=1000


deploy() {
  drtpy contract deploy --bytecode=${ROOT}/output/dcdt-nft-marketplace.wasm \
  --ledger --ledger-account-index=$LEDGER_INDEX \
  --proxy=${PROXY} --chain=${CHAIN_ID} --gas-limit=${GAS} \
  --arguments ${BID_PERCENT} \
  --outfile="deploy.json" --recall-nonce --send  || return

  TX=$(drtpy data parse --file="deploy.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(drtpy data parse --file="deploy.json" --expression="data['contractAddress']")

  drtpy data store --key=address-nft-marketplace --value=${ADDRESS}
  drtpy data store --key=deploy-tx-nft-marketplace --value=${TX}

  echo ""
  echo "Smart contract address: ${ADDRESS}"
}

pause() {
  function_name="pause"
  drtpy contract call $CONTRACT_ADDRESS \
  --ledger --ledger-account-index=$LEDGER_INDEX \
  --proxy=${PROXY} --chain=${CHAIN_ID} --gas-limit=5000000 \
  --function ${function_name} \
  --recall-nonce --send  || return
}

unpause() {
  function_name="unpause"
  drtpy contract call $CONTRACT_ADDRESS \
  --ledger --ledger-account-index=$LEDGER_INDEX \
  --proxy=${PROXY} --chain=${CHAIN_ID} --gas-limit=5000000 \
  --function ${function_name} \
  --recall-nonce --send  || return
}

upgrade() {
  drtpy contract upgrade $CONTRACT_ADDRESS --bytecode=${ROOT}/output/dcdt-nft-marketplace.wasm \
  --ledger --ledger-account-index=$LEDGER_INDEX \
  --proxy=${PROXY} --chain=${CHAIN_ID} --gas-limit=${GAS} \
  --arguments ${BID_PERCENT} \
  --outfile="upgrade.json" --recall-nonce --send  || return
}