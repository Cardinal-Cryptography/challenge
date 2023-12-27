import os
from substrateinterface import SubstrateInterface, Keypair, ContractInstance

TESTNET_WS = "wss://ws.test.azero.dev"
HARDXORE_ADDRESS = "5GErKuHmZ8ytupuZb78AJbHY9yoaDnKLdLUYKchYukhrsjVj"


def randomness_from_sim(sim):
    try:
        return sim['result']['Ok']['data']['Ok']
    except Exception:
        return None


def send_register_randomness_tx(contract, keypair, chain):
    print(f"Simulating register_randomness...")
    prediction = contract.read(keypair, 'register_randomness')
    print(f"Result of simulation: {prediction}")
    predicted_gas = prediction.gas_required
    print(f"Simulation of register_randomness cost {predicted_gas} gas.")
    predicted_gas['ref_time'] *=2
    predicted_gas['proof_size'] *=2
    print(f"Sending actual transaction...")
    result = contract.exec(keypair, 'register_randomness', args={}, gas_limit = predicted_gas)
    print(f"Events triggered: {result.contract_events}")
    number = chain.get_block_number(result.block_hash)
    print(f"Transaction landed in block {result.block_hash} ({number})")
    simulation_result = contract.read(keypair, 'get_randomness', args={"num": number})
    randomness = randomness_from_sim(simulation_result.value)
    print(f"Randomness for block {number} is {randomness}\n\n")


def has_badge(contract, keypair, acc, badge):
    sim_result = contract.read(keypair, 'has_badge', args = {"acc": acc, "badge": badge})
    return sim_result.value['result']['Ok']['data'] is not None


def send_attempt_xor_3_tx(contract, keypair):
    solution = [14000000, 14000005]
    print(f"Simulating attempt_xor_3 on input {solution}")
    prediction = contract.read(keypair, 'attempt_xor_3', args = {"solution":solution})
    print(f"Result of simulation: {prediction}")
    predicted_gas = prediction.gas_required
    predicted_gas['ref_time'] *=2
    predicted_gas['proof_size'] *=2
    print(f"Sending actual transaction...")
    result = contract.exec(keypair, 'attempt_xor_3', args = {"solution": solution}, gas_limit = predicted_gas)
    # Don't expect a success here -- the above solution is obviously wrong for `XOR-3`
    print(result.error_message, '\n\n')


def example_interaction_hardxore():
    try:
        with open(os.path.join(os.path.dirname(__file__), 'seed.phrase'), 'r') as file:
            phrase = file.read()
            keypair = Keypair.create_from_uri(phrase)
    except Exception as e:
        print(f"Unable to get a keypair -- please place your developer seedphrase in the seed.phrase file in the same directory as example.py")
        return
    account_id = keypair.ss58_address
    # Make sure there is TZERO on this account -- otherwise sending transactions will fail
    print(f"Keypair read -- AccountId = {account_id}")
    aleph_testnet = SubstrateInterface(url=TESTNET_WS, ss58_format=42,type_registry={
            "types": {
                "ContractExecResult": "ContractExecResultTo269"
            }
        })
    metadata_file = os.path.join(os.path.dirname(__file__), '../../metadata/hardxore.json')

    contract = ContractInstance.create_from_address(contract_address=HARDXORE_ADDRESS, metadata_file=metadata_file, substrate=aleph_testnet)

    send_register_randomness_tx(contract, keypair, aleph_testnet)
    send_attempt_xor_3_tx(contract, keypair)
    for badge in ["WARMUP", "XOR-0", "XOR-1", "XOR-2", "XOR-3"]:
        res = has_badge(contract, keypair, "5G9s3sTZ1QdCyYpYySDLaUMLtFzWqcB2qKHnzkcoT7ceuu8A", badge)
        print(f"Checking if {account_id} has badge {badge} -- {res}")


if __name__ == '__main__':
    example_interaction_hardxore()
