# TrustSplit

**TrustSplit** is a decentralized group finance application built on the Stellar network.

It enables trusted groups  friends, roommates, travel squads, teams, and communities  to manage shared money transparently using **on-chain multisignature accounts**, shared **group treasuries**, and **savings goals with target amounts**.

Instead of relying on a central service to hold funds, every group in TrustSplit is backed by a **real Stellar account** where the treasury lives directly on the Stellar ledger. Members can contribute funds, track progress toward group financial goals, split expenses, and approve spending collectively.

Funds move only when the group agrees.

---

## Key Features

### Group Treasury on Stellar

Each group creates a shared treasury represented by a **Stellar multisig account**.

Members contribute funds directly to this account using Stellar payments.
The treasury balance is transparent and verifiable on the Stellar ledger.

---

### Group Savings Goals

Groups can set a **target amount for their shared treasury**.

Example:

Trip Fund Target: **500 USDC**

Members contribute funds directly to the group’s Stellar account, and progress toward the goal is tracked transparently on-chain.

Once the goal is reached, the group can use the treasury for shared expenses through the multisignature approval process.

---

### Invite-Only Membership

Members join groups via invite links, QR codes, or their **Stellar public key (G… address)**.

When a member joins, their account is added as a signer using Stellar’s `SetOptions` operation.

Only invited members can access or co-manage the treasury.

---

### Bill Splitting with On-Chain Payments

Members can record shared expenses and settle them using Stellar payments.

Bills can be split:

* equally
* by custom amounts
* by percentages

Payments can be made in **XLM** or Stellar assets such as **USDC**.

---

### Multi-Signature Spending

Group funds are protected using Stellar’s native **multisignature accounts**.

Example:

3 of 5 members must approve before funds can be withdrawn.

Members sign transactions using their wallet before they are submitted to the Stellar network.

---

### Programmable Group Wallet

TrustSplit extends group treasuries with **programmable financial rules**.

Examples include:

* spending limits for members
* automatic bill settlement
* recurring group contributions
* savings goals for group projects

These rules are powered by **Soroban smart contracts**.

---

## How It Works

| Feature              | Stellar Mechanism                           |
| -------------------- | ------------------------------------------- |
| Group treasury       | Multisig account                            |
| Member permissions   | Stellar signers                             |
| Spending approval    | Threshold signatures                        |
| Payments             | Stellar payment operations                  |
| Savings goals        | Contributions tracked via on-chain payments |
| Transaction tracking | Horizon API                                 |

---

## Tech Stack

Frontend
Next.js

Backend
Node.js

Blockchain
Stellar Network

Smart Contracts
Soroban 


---

## Why Stellar

TrustSplit uses the Stellar network because it provides:

* fast transaction settlement
* extremely low fees
* global payments
* native multisignature accounts

---


