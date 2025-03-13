# OpenHack - FRAME Pallet Challenge

This project provides a hands-on introduction to Substrate's FRAME development patterns by implementing a simplified version of common blockchain pallets.

## (Optional) Setup environment and register for the challenges

TLDR: If you are not familiar with Git & Github, follow these steps below to fork and setup your own repository.

- Step 1: Install Git & Github Desktop (optional) on your local device
- Step 2: Fork this repository by click the `Fork button` on Github

![image](https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7fa2f01a-b523-4208-92db-d8af7a274d98)

- Step 3: `Clone` the forked repository to your local device using the below command

```sh
git clone https://github.com/<your_github_username>/frame-challenges.git
```

Replace the `[name-of-your-account]` with your Github username. For example, if my username is `chungquantin`, I would do the below command to clone the repository to my local device.

```sh
git clone https://github.com/openguild-labs/frame-challenges.git
```

- Step 4: Edit the `README.md` file to register for official participation

Go to **Participant Registration** section and register to be the workshop participants. Add the below to the list, replace any placeholder with your personal information.

```
| 🦄 | Name | Github username | Your current occupation |
```

- Step 5: `Commit` your code and push to the forked Github repository

```
git add .
git commit -m "Register for OpenGuild FRAME Challenges"
```

- Step 6: Create a `Pull Request` to merge your changes to this repository and name your PR as `Your name | Register for OpenGuild FRAME Challenges`

<img width="1166" alt="Screenshot 2024-04-19 at 16 23 45" src="https://github.com/openguild-labs/open-hack-rust-starter/assets/56880684/7554ca7d-da68-4a23-893a-4f2c11a78d37">

<br/>

<div align="center">

</div>

<br/>

## Challenge Overview

After learning about Substrate, FRAME, Runtime, and Pallets in Polkadot SDK, this coding challenge helps you understand how these components work together by implementing a simplified version of the actual architecture.

### Key Concepts:

- **FRAME Pallets**: Libraries for building Substrate runtime, including core frame components and functional pallets
- **Runtime**: A collection of FRAME pallets that define blockchain logic

### Project Structure

This challenge simulates three core components:
- `system.rs`: Foundation module similar to `frame_system`
- `staking.rs`: Token staking module similar to `pallet_staking`
- `governance.rs`: On-chain proposal system similar to `pallet_collective` or `pallet_democracy`

### Runtime Configuration

The project defines a simplified runtime that configures concrete types for the generic pallets:

```rust
pub struct Runtime;

impl SystemConfig for Runtime {
    type AccountId = u64;
}

impl StakingConfig for Runtime {
    type Balance = u64;
}

impl GovernanceConfig for Runtime {
    // No additional types needed
}
```

## 👉 Contribute to OpenGuild Community

OpenGuild is a builder-driven community centered around Polkadot. OpenGuild is built by Web3 builders for Web3 builders. Our primary aim is to cater to developers seeking a comprehensive understanding of the Polkadot blockchain, providing curated, in-depth materials with a low-level approach.

- **About us:** [Learn more about us](https://openguild.wtf/about)
- **Website:** [OpenGuild Website](https://openguild.wtf/)
- **Github:** [OpenGuild Labs](https://github.com/openguild-labs)
- **Discord**: [Openguild Discord Channel](https://discord.gg/bcjMzxqtD7)

## Participant Registration

Add your information to the below list to officially participate in the workshop challenge (This is the first mission of the whole workshop)

| Emoji | Name            | Github Username                                       | Occupations              |
| ----- | --------------- | ----------------------------------------------------- | ------------------------ |
|   🦁  | Muhammad Ramadhani            | [Dante4rt](https://github.com/dante4rt)                 | Software Engineer                   |
