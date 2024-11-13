🦀 CrabbyAuth Architecture
Table of Contents

Overview
Core Components
Data Flow
Plugin System
Security Architecture
Storage Architecture
Deployment Architecture

Overview
CrabbyAuth is designed with modularity, security, and extensibility as its core principles. The architecture follows the Rust idioms of zero-cost abstractions and compile-time guarantees while providing a flexible plugin system.
High-Level Architecture

```mermaid
graph TB
    Client[Client Application] --> Auth[Auth Handler]
    Auth --> OAuth[OAuth Module]
    Auth --> SCIM[SCIM Module]
    
    OAuth --> TokenMgr[Token Manager]
    OAuth --> FlowMgr[Flow Manager]
    
    SCIM --> UserMgr[User Manager]
    SCIM --> GroupMgr[Group Manager]
    
    TokenMgr --> Storage[(Storage)]
    UserMgr --> Storage
    
    subgraph Plugins[Plugin System]
        CustomAuth[Custom Auth]
        CustomProvider[Custom Provider]
        CustomStorage[Custom Storage]
    end
    
    OAuth -.-> Plugins
    SCIM -.-> Plugins
    Storage -.-> Plugins
```

## Core Components

### Component Relationships

```mermaid
classDiagram
    class OAuth {
        +handle_authorization()
        +validate_token()
        +refresh_token()
    }
    
    class SCIM {
        +provision_user()
        +sync_groups()
        +handle_lifecycle()
    }
    
    class PluginManager {
        +load_plugin()
        +validate_plugin()
        +execute_plugin()
    }
    
    class Storage {
        +store()
        +retrieve()
        +delete()
    }
    
    OAuth --> PluginManager
    SCIM --> PluginManager
    OAuth --> Storage
    SCIM --> Storage
```

# Data Flow

## Authentication Flow
```mermaid
sequenceDiagram
    participant C as Client
    participant A as CrabbyAuth
    participant P as Provider
    participant S as Storage
    
    C->>A: Initialize Auth Flow
    A->>P: Request Authorization
    P->>C: Redirect with Code
    C->>A: Exchange Code
    A->>P: Validate Code
    P->>A: Return Tokens
    A->>S: Store Tokens
    A->>C: Return Session
```

## SCIM Provisioning Flow
```mermaid
sequenceDiagram
    participant IDP as Identity Provider
    participant C as CrabbyAuth
    participant S as Storage
    participant H as Hooks
    
    IDP->>C: Provision Request
    C->>C: Validate Request
    C->>S: Store User Data
    C->>H: Trigger Hooks
    H-->>C: Hook Results
    C->>IDP: Confirmation
```

# Plugin System
## Plugin Architecture

```mermaid
graph LR
    Core[Core System] --> PM[Plugin Manager]
    PM --> Validator[Plugin Validator]
    PM --> Loader[Plugin Loader]
    PM --> Executor[Plugin Executor]
    
    subgraph Plugins
        Auth[Auth Plugins]
        Storage[Storage Plugins]
        Provider[Provider Plugins]
    end
    
    Executor --> Auth
    Executor --> Storage
    Executor --> Provider
    
    subgraph Sandbox
        ValidateInput[Input Validation]
        ResourceLimits[Resource Limits]
        SecurityChecks[Security Checks]
    end
    
    Plugins --> Sandbox
```

# Security Architecture

## Security Layers

```mermaid
graph TD
    subgraph External[External Security]
        TLS[TLS 1.3]
        CORS[CORS Policy]
        RateLimit[Rate Limiting]
    end
    
    subgraph Application[Application Security]
        TokenSec[Token Security]
        InputVal[Input Validation]
        AuthN[Authentication]
        AuthZ[Authorization]
    end
    
    subgraph Data[Data Security]
        Encrypt[Encryption]
        Audit[Audit Logging]
        Backup[Backup]
    end
    
    External --> Application
    Application --> Data
```

# Storage Architecture

## Storage Implementation
```mermaid
graph TB
    API[Storage API] --> Interface[Storage Interface]
    
    Interface --> InMem[In-Memory]
    Interface --> Redis[Redis]
    Interface --> SQL[SQL]
    
    subgraph "Storage Types"
        Token[Token Storage]
        Session[Session Storage]
        User[User Storage]
    end
    
    InMem --> Token
    Redis --> Session
    SQL --> User
```

# Deployment Architecture
## Deployment Options

```mermaid
graph TB
    subgraph Cloud[Cloud Deployment]
        LB[Load Balancer]
        LB --> API1[API Instance 1]
        LB --> API2[API Instance 2]
        API1 --> Cache[Redis Cache]
        API2 --> Cache
        API1 --> DB[(Database)]
        API2 --> DB
    end
    
    subgraph Monitoring
        Metrics[Metrics Collection]
        Logs[Log Aggregation]
        Traces[Distributed Tracing]
    end
    
    Cloud --> Monitoring
```

# Implementation Details
Each component in CrabbyAuth is designed with the following principles:

Zero-Cost Abstractions: Using Rust's trait system to provide flexible interfaces without runtime overhead

Type Safety: Leveraging Rust's type system to prevent common security issues at compile time
Async by Default: Built on async/await for scalable performance

Plugin Safety: Sandboxed plugin execution with resource limits and security boundaries

Core Trait Examples

```rust
pub trait AuthenticationFlow {
    async fn authenticate(&self, request: AuthRequest) -> Result<AuthResponse, AuthError>;
    async fn validate(&self, token: Token) -> Result<Claims, ValidationError>;
}

#[async_trait]
pub trait IdentityProvider {
    async fn provision_user(&self, user: User) -> Result<ProvisionedUser, ProvisionError>;
    async fn sync_groups(&self, user_id: UserId) -> Result<Vec<Group>, SyncError>;
}

#[async_trait]
pub trait StorageBackend {
    async fn store<T: Serialize>(&self, key: &str, value: T) -> Result<(), StorageError>;
    async fn retrieve<T: DeserializeOwned>(&self, key: &str) -> Result<T, StorageError>;
}
```

The architecture is designed to be both secure by default and extensible through a carefully controlled plugin system. Each component is isolated and communicates through well-defined interfaces, making the system both maintainable and secure.
For more detailed information about specific components, please refer to their respective documentation sections:

### OAuth Implementation
### SCIM Implementation
### Plugin System
### Security Model