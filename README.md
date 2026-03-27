# 🌟 Subscription Token Smart Contract (Soroban)

## 📌 Project Description

This project implements a **subscription-based token system** using Soroban smart contracts on the Stellar network. It enables decentralized subscription management where users can subscribe to services for a fixed duration and verify their active status directly on-chain.

The contract is designed to be simple, efficient, and easily extendable for real-world Web3 applications such as SaaS platforms, content platforms, and membership-based ecosystems.

---

## ⚙️ What It Does

The smart contract provides core subscription functionality:

* Allows users to subscribe to a plan with a fixed duration
* Stores subscription data securely on-chain
* Tracks expiration timestamps
* Enables public verification of subscription status
* Allows users to cancel subscriptions at any time

---

## 🚀 Features

### 🔐 Secure Authentication

All subscription actions require authorization from the user's Stellar address.

### ⏱ Time-Based Subscriptions

Each subscription includes an expiration timestamp to manage access duration.

### 🔄 Renewable Plans

Users can renew or extend subscriptions by calling the `subscribe` function again.

### ❌ Cancellation Support

Users can cancel their subscription anytime, removing their record from contract storage.

### 🔍 Public Verification

Anyone can check whether a subscription is active using on-chain data.

### 📦 Lightweight & Efficient

Optimized use of Soroban storage for minimal cost and high performance.

---

## 🧠 Possible Extensions

* 💰 Token-based payments (e.g., USDC on Stellar)
* 🪙 NFT-based subscription passes
* 📊 Multiple pricing tiers (Basic, Pro, Premium)
* 🔁 Auto-renewal subscriptions
* 🛠 Admin-controlled subscription plans

---

## 🔗 Smart Contract Deployment

**Contract ID:**
`CAKXAF4HRIVXICLQIJGTALNDDLC66R7VOU6LZELK54OUMHCK5F6JMWFH`

**Live Interface:**
👉 https://stellaride.vercel.app/

> You can interact with the contract using Soroban tools, SDKs, or the interface above.

---

## 🛠 Tech Stack

* Rust
* Soroban SDK
* Stellar Blockchain

---

## 📄 License

MIT License

---

## 🙌 Acknowledgements

Built using Soroban, the smart contract platform on Stellar, enabling fast and scalable decentralized applications.
