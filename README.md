Splitwise is a decentralized group finance application built natively on the **Stellar blockchain**. It is designed for trusted groups — friends, travel squads, flatmates, or colleagues — who want a transparent, trustless, and borderless way to manage shared money.

Every group on Splitwise is backed by a **Stellar multisig account** — a real on-chain Stellar account where the group's treasury lives. No middleman holds your funds. No bank. No app company. The money sits directly on the Stellar ledger, and only moves when the group agrees.

---

## Core Features

### 👥 Invite-Only Groups on Stellar

- Create a group — a dedicated **Stellar multisig account** is generated on-chain instantly
- Invite members via link, QR code, their **Stellar public key (G... address)**, or email
- Each accepted member is added as a **signer** to the group's Stellar account via `SetOptions`
- No one can join or access group funds without an explicit invitation

### 💸 Bill Splitting via Stellar Payments

- Add a shared expense and split it equally or by custom amounts
- Each member's share is settled as a direct **Stellar payment operation** — peer-to-peer, on-chain
- Pay in **XLM** or **USDC** (native Stellar stablecoin) — no volatile exchange rates when splitting bills
- Every settled payment is permanently recorded on the **Stellar ledger**
- Track who has paid and who still owes, all verifiable on-chain via **Stellar Horizon**

### 🏦 Group Treasury — A Shared Stellar Account

- Members contribute funds directly to the group's **Stellar multisig account**
- Set a savings goal (e.g. "Weekend trip — 500 USDC") and track progress on-chain
- The treasury balance is fully transparent — any member can verify it via **Stellar Explorer**
- Funds are held trustlessly on the Stellar ledger — not by Splitwise, not by any third party
- Use the treasury to cover group bills directly from the shared Stellar account

### ✍️ Multi-Signature Spending (Stellar Native)

- Any treasury withdrawal is built as a **Stellar transaction envelope** and sent for co-signing
- The co-signer threshold (e.g. 3-of-5) is enforced at the **Stellar protocol level** via account thresholds — not by app logic
- Members sign with their **Freighter wallet** (ed25519 signatures)
- Once the signing weight threshold is met, the transaction is submitted to **Stellar Horizon** and executes immediately
- No single member — not even the group creator — can bypass the multisig rule

---

## How It Works on Stellar

| Feature               | Stellar Mechanism                                         |
| --------------------- | --------------------------------------------------------- |
| Group treasury        | Multisig account (`SetOptions` with signers + thresholds) |
| Co-sign approval      | Stellar transaction with collected signatures             |
| Bill settlement       | Stellar payment operations (XLM or USDC)                  |
| Savings contributions | Direct Stellar payments to group multisig account         |
| Invite via wallet     | Stellar account ID (G...) used as member identifier       |

---

## User Flow

```
1. Create Group
   └─ Set name, emoji, co-signer threshold
   └─ Stellar multisig account is created for the group

2. Invite Members
   └─ Share invite link / QR / wallet address / email
   └─ Each accepted member is added as a signer to the Stellar account

3. Contribute to Treasury
   └─ Members send USDC/XLM to the group's Stellar account
   └─ Progress tracked against the group's savings goal

4. Add a Bill
   └─ Enter description + total amount
   └─ App calculates each member's share
   └─ Members pay their share via Stellar payment

5. Spend from Treasury
   └─ Any member requests a treasury withdrawal
   └─ Other members receive a co-sign notification
   └─ Once N members sign, the Stellar transaction is submitted and executed
```
