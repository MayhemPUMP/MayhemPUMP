# Mayhem PUMP API 参考文档

## API 概述

本文档详细说明了 Mayhem PUMP 平台的 API 接口规范和使用方法。所有 API 遵循 RESTful 设计原则，使用 HTTPS 进行安全传输。

## 基础信息

- **基础URL**: `https://api.mayhempump.com/v1`
- **认证方式**: Bearer Token
- **响应格式**: JSON

## 认证

### 获取访问令牌

```http
POST /auth/token
```

#### 请求参数

| 参数名 | 类型 | 必选 | 描述 |
|--------|------|------|------|
| address | string | 是 | 用户钱包地址 |
| signature | string | 是 | 签名信息 |

#### 响应示例

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600
}
```

## 用户接口

### 获取用户信息

```http
GET /users/me
```

#### 响应示例

```json
{
  "id": "user123",
  "address": "0x1234...",
  "nickname": "Player1",
  "avatar": "https://...",
  "created_at": "2024-01-01T00:00:00Z"
}
```

### 更新用户信息

```http
PUT /users/me
```

#### 请求参数

| 参数名 | 类型 | 必选 | 描述 |
|--------|------|------|------|
| nickname | string | 否 | 用户昵称 |
| avatar | string | 否 | 头像URL |

## 游戏接口

### 获取游戏列表

```http
GET /games
```

#### 查询参数

| 参数名 | 类型 | 必选 | 描述 |
|--------|------|------|------|
| page | integer | 否 | 页码 |
| limit | integer | 否 | 每页数量 |
| status | string | 否 | 游戏状态 |

#### 响应示例

```json
{
  "total": 100,
  "items": [
    {
      "id": "game123",
      "name": "PUMP Challenge",
      "status": "active",
      "players": 10,
      "prize_pool": "1000000000000000000"
    }
  ]
}
```

### 加入游戏

```http
POST /games/{game_id}/join
```

#### 请求参数

| 参数名 | 类型 | 必选 | 描述 |
|--------|------|------|------|
| stake_amount | string | 是 | 质押金额 |

## NFT接口

### 获取NFT列表

```http
GET /nfts
```

#### 查询参数

| 参数名 | 类型 | 必选 | 描述 |
|--------|------|------|------|
| owner | string | 否 | 所有者地址 |
| collection | string | 否 | 收藏集ID |

#### 响应示例

```json
{
  "total": 50,
  "items": [
    {
      "token_id": "1",
      "collection": "collection123",
      "owner": "0x1234...",
      "metadata": {
        "name": "PUMP Hero #1",
        "image": "https://..."
      }
    }
  ]
}
```

## 市场接口

### 获取市场数据

```http
GET /market/stats
```

#### 响应示例

```json
{
  "token_price": "1.5",
  "market_cap": "1000000",
  "volume_24h": "50000",
  "holders": 1000
}
```

## 错误处理

### 错误响应格式

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "错误描述"
  }
}
```

### 常见错误码

| 错误码 | 描述 |
|--------|------|
| 400 | 请求参数错误 |
| 401 | 未授权 |
| 403 | 权限不足 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

## 速率限制

- 基础限制：60次/分钟
- 认证用户：300次/分钟

## WebSocket API

### 连接信息

- **WebSocket URL**: `wss://api.mayhempump.com/ws`
- **认证**: 通过URL参数传递token

### 订阅主题

```json
{
  "type": "subscribe",
  "channel": "game_updates",
  "game_id": "game123"
}
```

### 事件类型

- game_start: 游戏开始
- game_end: 游戏结束
- price_update: 价格更新
- player_action: 玩家操作

## SDK 支持

我们提供以下语言的SDK：

- JavaScript/TypeScript
- Python
- Java
- Go

## 更新日志

### v1.0.0 (2024-01-01)

- 初始版本发布
- 基础API功能实现
- WebSocket支持