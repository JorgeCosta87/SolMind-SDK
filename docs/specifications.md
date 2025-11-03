# SolMind SDK Specifications

## Overview

SolMind SDK is a gaming infrastructure layer for Solana that enables Web3 games to implement token staking, resource generation, and peer-to-peer trading. The system uses Token 2022 with metadata extensions to create game resources (resources, XP) as on-chain tokens with rich metadata.

## Gaming Context

In a typical game scenario:
- **Game Tokens**: The game's primary currency (e.g., "Game token")
- **Game Resources**: In-game items represented as Token 2022 tokens (e.g., "Resources", "Energy Crystals", "XP")
- **Staking**: Players lock their game tokens to generate game resources over time
- **Trading**: Players can trade resources with each other via P2P trading (planned)

## User Stories

### US-001: Game Developer Initializes Resource Staking Vault
**As a** game developer  
**I want to** initialize a staking vault for a specific game resource  
**So that** players can stake game tokens and earn that resource type

**Acceptance Criteria:**
- Developer can create a new staking vault for a game resource type
- Resource mint is created using Token 2022 program with metadata pointer extension
- Resource metadata (name, symbol, URI, custom attributes) is initialized
- Base yield rate is configured (resources generated per staked token)
- Lock durations are set (e.g., 15, 30, 90 days)
- Stake config PDA is created and initialized
- Resource can be identified by game systems via metadata
---

### US-002: Player Stakes Game Tokens to Earn Resources
**As a** player  
**I want to** stake my game tokens for a specified lock duration  
**So that** I can earn game resources (Resources, XP) that I can use in-game or trade

**Acceptance Criteria:**
- Player can transfer game tokens from their wallet to the vault
- Player selects a valid lock duration from configured options
- Staking position PDA is created for the player
- Vault token account receives the staked game tokens
- Stake amount, stake slot, and unlock slot are recorded
- Global statistics (total_staked, total_positions) are updated
- Player cannot stake with invalid lock duration
- Player receives game resource tokens as rewards (claimable in future)
---

### US-003: Player Claims Game Resources
**As a** player  
**I want to** claim the game resources I've earned from staking  
**So that** I can use them in-game or trade them

**Acceptance Criteria:**
- Player can claim accumulated resources based on stake time
- Resources are minted to player's wallet as Token 2022 tokens
- Claim amount is calculated based on stake amount, yield rate, and time elapsed
- Claim updates `last_claim_slot` and `total_claimed` in position
- Player can claim multiple times before unlock period ends
- Yield calculation uses basis points (100 = 1% APY)
- Formula: `yield = (amount_staked × yield_rate_bps × slots_elapsed) / (10000 × slots_per_year)`

---

### US-004: Player Trades Resources P2P (Planned)
**As a** player  
**I want to** trade my game resources with other players  
**So that** I can acquire resources I need or sell excess resources

**Acceptance Criteria:**
- Player can create buy/sell orders for game resources
- Orders are matched with other players' orders
- Trades execute atomically with escrow protection
- Players can cancel their pending orders
- Trading history is trackable on-chain
---

## State Models

### StakeConfig
- `admin: Pubkey` - Admin authority
- `deposit_mint: Pubkey` - Token mint users deposit
- `reward_mint: Pubkey` - Token mint users receive as rewards
- `base_yield_rate: u64` - Base yield rate for rewards
- `lock_durations: Vec<u64>` - Available lock duration options (days)
- `total_staked: u64` - Total amount staked across all positions
- `total_positions: u64` - Total number of active staking positions
- `bump: u8` - PDA bump seed

### StakingPosition
- `user: Pubkey` - User who owns the position
- `stake_config: Pubkey` - Reference to the stake configuration
- `amount_staked: u64` - Amount of tokens staked
- `stake_slot: u64` - Slot when staking occurred
- `unlock_slot: u64` - Slot when tokens can be unstaked
- `last_claim_slot: u64` - Last slot when rewards were claimed
- `total_claimed: u64` - Total rewards claimed so far
- `yield_rate: u64` - Yield rate in basis points
- `bump: u8` - PDA bump seed

## Token 2022 Integration

The system uses Token 2022 program with the following features:

### Metadata Pointer Extension
- Reward mint includes metadata pointer extension
- Metadata is stored on-chain in TLV format
- Includes name, symbol, and URI fields

## Lock Duration System

- Lock durations are specified in days
- Conversion: 216,000 Solana slots = 1 day
- Users select from pre-configured duration options
- Longer lock durations may provide higher yield rates (planned)

## Yield Calculation

Rewards are calculated using a basis points system with annual percentage yield:

```
yield = (amount_staked × yield_rate_bps × slots_elapsed) / (10000 × slots_per_year)
```

Where:
- `amount_staked`: Tokens staked by the player
- `yield_rate_bps`: Yield rate in basis points (100 = 1% APY, 500 = 5% APY)
- `slots_elapsed`: Slots since last claim (or since staking if first claim)
- `slots_per_year`: 365 × 216,000 = 78,840,000 slots

### Example Calculation
- Player stakes 1,000,000,000 tokens (1B with 9 decimals)
- Yield rate: 100 bps (1% APY)
- Time elapsed: 500 slots
- Yield = (1e9 × 100 × 500) / (10000 × 78840000) ≈ 0.63 tokens

## Error Handling

### StakeError
- `InsufficientFunds` - User doesn't have enough tokens to stake
- `InvalidLockDuration` - Selected lock duration is not in allowed list
- `NoYieldAccrued` - No yield has been earned yet (called claim too early or already up-to-date)

## Future Enhancements

### Phase 1: Core Staking ✅
- [x] Initialize vault with resource mint
- [x] Stake game tokens for resources
- [x] Claim resources instruction
- [ ] Unstake tokens instruction (after lock period)
- [ ] Variable yield rates based on lock duration
- [ ] Compound staking (re-stake earned resources)

### Phase 2: P2P Trading (Planned)
- [ ] Create trading orders (buy/sell game resources)
- [ ] Order matching engine
- [ ] Escrow system for secure trades
- [ ] Order cancellation
- [ ] Trading history and analytics

### Phase 3: Advanced Gaming Features (Future)
- [ ] Resource fusion/crafting (combine resources)
- [ ] Automated reward distribution
- [ ] Multi-resource staking pools
- [ ] Cross-game resource trading
- [ ] Game marketplace integration
- [ ] NFT resource upgrade paths
