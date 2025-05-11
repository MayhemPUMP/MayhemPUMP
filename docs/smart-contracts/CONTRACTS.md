# Mayhem PUMP 智能合约文档

## 合约架构

### 核心合约

1. **游戏合约**
   - GameCore：游戏核心逻辑
   - GameFactory：游戏实例创建
   - GameRegistry：游戏注册管理

2. **代币合约**
   - PUMPToken：ERC20游戏代币
   - TokenVesting：代币解锁机制
   - TokenStaking：质押奖励系统

3. **NFT合约**
   - CharacterNFT：角色NFT
   - EquipmentNFT：装备NFT
   - AchievementNFT：成就NFT

4. **DAO合约**
   - Governance：治理决策
   - Treasury：资金管理
   - Proposal：提案系统

## 合约接口

### GameCore

```solidity
interface IGameCore {
    // 游戏状态
    enum GameState { Created, Active, Ended }
    
    // 事件
    event GameCreated(uint256 indexed gameId, address creator);
    event PlayerJoined(uint256 indexed gameId, address player);
    event GameStarted(uint256 indexed gameId);
    event GameEnded(uint256 indexed gameId, address winner);
    
    // 核心功能
    function createGame(uint256 _minPlayers, uint256 _maxPlayers) external returns (uint256);
    function joinGame(uint256 _gameId) external payable;
    function startGame(uint256 _gameId) external;
    function makeMove(uint256 _gameId, uint256 _move) external;
    function endGame(uint256 _gameId) external;
    
    // 查询功能
    function getGameState(uint256 _gameId) external view returns (GameState);
    function getGamePlayers(uint256 _gameId) external view returns (address[] memory);
    function getPlayerStats(address _player) external view returns (uint256 wins, uint256 played);
}
```

### PUMPToken

```solidity
interface IPUMPToken {
    // 事件
    event Staked(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount);
    event RewardPaid(address indexed user, uint256 reward);
    
    // 代币功能
    function mint(address _to, uint256 _amount) external;
    function burn(uint256 _amount) external;
    function stake(uint256 _amount) external;
    function withdraw(uint256 _amount) external;
    function getReward() external;
    
    // 查询功能
    function balanceOf(address _account) external view returns (uint256);
    function stakedBalance(address _account) external view returns (uint256);
    function earned(address _account) external view returns (uint256);
}
```

### CharacterNFT

```solidity
interface ICharacterNFT {
    // 角色属性
    struct Character {
        uint256 level;
        uint256 experience;
        uint256[] skills;
        uint256 lastActionTime;
    }
    
    // 事件
    event CharacterMinted(uint256 indexed tokenId, address owner);
    event CharacterLevelUp(uint256 indexed tokenId, uint256 newLevel);
    event SkillLearned(uint256 indexed tokenId, uint256 skillId);
    
    // NFT功能
    function mintCharacter(address _to) external returns (uint256);
    function levelUp(uint256 _tokenId) external;
    function learnSkill(uint256 _tokenId, uint256 _skillId) external;
    
    // 查询功能
    function getCharacter(uint256 _tokenId) external view returns (Character memory);
    function getSkills(uint256 _tokenId) external view returns (uint256[] memory);
}
```

### Governance

```solidity
interface IGovernance {
    // 提案状态
    enum ProposalState { Pending, Active, Canceled, Defeated, Succeeded, Executed }
    
    // 事件
    event ProposalCreated(uint256 indexed proposalId, address proposer);
    event VoteCast(address indexed voter, uint256 proposalId, bool support);
    event ProposalExecuted(uint256 indexed proposalId);
    
    // 治理功能
    function propose(bytes[] calldata _calls, string memory _description) external returns (uint256);
    function castVote(uint256 _proposalId, bool _support) external;
    function execute(uint256 _proposalId) external;
    
    // 查询功能
    function getProposalState(uint256 _proposalId) external view returns (ProposalState);
    function getVotes(address _account) external view returns (uint256);
}
```

## 部署指南

### 准备工作

1. **环境配置**
   ```bash
   # 安装依赖
   npm install
   
   # 编译合约
   npx hardhat compile
   ```

2. **配置文件**
   ```javascript
   // hardhat.config.js
   module.exports = {
     networks: {
       hardhat: {},
       testnet: {
         url: process.env.TESTNET_URL,
         accounts: [process.env.PRIVATE_KEY]
       },
       mainnet: {
         url: process.env.MAINNET_URL,
         accounts: [process.env.PRIVATE_KEY]
       }
     }
   };
   ```

### 部署步骤

1. **部署代币合约**
   ```bash
   npx hardhat run scripts/deploy-token.js --network testnet
   ```

2. **部署NFT合约**
   ```bash
   npx hardhat run scripts/deploy-nft.js --network testnet
   ```

3. **部署游戏合约**
   ```bash
   npx hardhat run scripts/deploy-game.js --network testnet
   ```

4. **部署治理合约**
   ```bash
   npx hardhat run scripts/deploy-dao.js --network testnet
   ```

### 合约验证

```bash
# 验证合约代码
npx hardhat verify --network testnet [CONTRACT_ADDRESS] [CONSTRUCTOR_ARGS]
```

## 安全考虑

### 访问控制

1. **角色管理**
   - 使用OpenZeppelin的AccessControl
   - 实现多级权限系统
   - 权限分离原则

2. **暂停机制**
   - 紧急暂停功能
   - 升级保护机制
   - 资金安全保护

### 安全检查

1. **合约审计**
   - 代码审查
   - 漏洞扫描
   - 形式化验证

2. **测试覆盖**
   - 单元测试
   - 集成测试
   - 压力测试

## 升级机制

### 代理模式

1. **透明代理**
   - 实现可升级性
   - 状态保持
   - 向后兼容

2. **升级流程**
   - 提案投票
   - 多重签名
   - 时间锁定

## 测试网络

### 测试部署

1. **测试网信息**
   - 网络：Sepolia
   - 区块浏览器：Etherscan
   - Faucet地址

2. **合约地址**
   - Token: [地址]
   - NFT: [地址]
   - Game: [地址]
   - DAO: [地址]

## 主网部署

### 部署检查清单

1. **准备工作**
   - 代码审计完成
   - 测试全部通过
   - 多重签名设置

2. **部署步骤**
   - 按序部署合约
   - 验证合约代码
   - 设置初始参数

### 监控维护

1. **监控系统**
   - 事件监听
   - 异常报警
   - 性能监控

2. **维护计划**
   - 定期检查
   - 升级规划
   - 应急响应