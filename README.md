# Aleph Zero Ink! Challenge

### List of Challenges
1. HardXORe: WIP
   - Contracts: `contracts/hardxore` deployed at `5GErKuHmZ8ytupuZb78AJbHY9yoaDnKLdLUYKchYukhrsjVj`. Metadata is available in `metadata/hardxore.json`.
   - Badges to be collected: `WARMUP`, `XOR-0`, `XOR-1`, `XOR-2`, `XOR-3`.
   - To check if an account `acc` has been awarded badge `b` call the method `has_badge(acc, b)`.


### What is this challenge about?
This challenge is all about learning blockchain and smart contract development -- more specifically about substrate and ink! smart contracts. The skills you can develop by taking part in the challenge are:

 - Reading and understanding smart contracts source code.
 - Writing smart contracts.
 - Auditing smart contracts.
 - Out of the box thinking.
 - Cryptography.

### How is the challenge structured?
We regularly publish new parts of the challenge. Each part comes with one or a few smart contracts. These contracts are deployed on the Aleph Zero Testnet (we provide contract addresses and metadata files) and their source code is available in this repository. The goal of each part is to earn one or more badges: each badge is awarded automatically by a suitable contract for performing a particular task. Typically the task require you to read some data on chain and interact with the contract by sending some transactions. The difficulty of getting particular badges varies -- some of the badges are as simple as "hello world", others require more thinking, and finally there are badges that ask you to crack some really tricky problems, and are virtually impossible to get.

### How do I take part in the challenge?
Just pick the current or any previous challenge part and start hacking! The goal is to get badges, it's up to you how to achieve that! There are no deadlines for any of the challenges.

### What are the rules of the challenge?
1. Don't spoil the fun for others: once you complete a part of the challenge and get a badge, don't publish any hints on how did you do that.
2. Related to the above: don't create pull requests to this repository that would give any hints to other challengers.
3. Don't spam the chain: the solution will never require you to send excessive numbers of transactions, or overload the public endpoints in any other way. 

### Are there any rewards for getting badges?
In general no, but there might occasionally be special challenges with rewards. Some of the challenges are also used for recruitment purposes for developer positions on Aleph Zero. 

There is also (will be, WIP) a public dashboard listing badge holders -- so there is fame to be gained.

### Technical hints
Here are some resources that might help you to get started, depending on which language do you prefer:

 - Python: You can use the [py-substrate-interface](https://github.com/polkascan/py-substrate-interface) library. It can be installed via `pip install substrate-interface`. See a python example in `examples/python/example.py` for interacting with the `hardxore` contract from the first challenge.
 - JavaScript: You can use the polkadot.js library. Examples: WIP.
 - Rust: You can use the subxt library. Examples: WIP.

